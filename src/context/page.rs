use std::fs;
use std::path::Path as FilePath;
use rayon::prelude::*;
use neon::prelude::*;
use skia_safe::{
  image::BitDepth, images, pdf,
  svg::{self, canvas::Flags},
  Canvas as SkCanvas, ClipOp, Color, ColorSpace, ColorType, AlphaType, Data, Document,
  Image as SkImage, ImageInfo, EncodedImageFormat, Matrix, Path, Picture, PictureRecorder, Rect, Size,
  IPoint,
};

use crc::{Crc, CRC_32_ISO_HDLC};
const CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

use crate::canvas::BoxedCanvas;
use crate::context::BoxedContext2D;
use crate::gpu::RenderingEngine;

//
// Deferred canvas (records drawing commands for later replay on an output surface)
//

pub struct PageRecorder{
  current: PictureRecorder,
  layers: Vec<Picture>,
  cache: Option<SkImage>,
  bounds: Rect,
  matrix: Matrix,
  clip: Option<Path>,
  changed: bool,
}

impl PageRecorder{
  pub fn new(bounds:Rect) -> Self {
    let mut rec = PictureRecorder::new();
    rec.begin_recording(bounds, None);
    rec.recording_canvas().unwrap().save(); // start at depth 2
    PageRecorder{ current:rec, changed:false, layers:vec![], cache:None, matrix:Matrix::default(), clip:None, bounds }
  }

  pub fn append<F>(&mut self, f:F)
    where F:FnOnce(&SkCanvas)
  {
    if let Some(canvas) = self.current.recording_canvas() {
      f(canvas);
      self.changed = true;
    }
  }

  pub fn set_bounds(&mut self, bounds:Rect){
    *self = PageRecorder::new(bounds);
  }

  pub fn update_bounds(&mut self, bounds:Rect){
    self.bounds = bounds; // non-destructively update the size
  }

  pub fn set_matrix(&mut self, matrix:Matrix){
    self.matrix = matrix;
    if let Some(canvas) = self.current.recording_canvas() {
      canvas.set_matrix(&matrix.into());
    }
  }

  pub fn set_clip(&mut self, clip:&Option<Path>){
    self.clip = clip.clone();
    self.restore();
  }

  pub fn restore(&mut self){
    if let Some(canvas) = self.current.recording_canvas() {
      canvas.restore_to_count(1);
      canvas.save();
      if let Some(clip) = &self.clip{
        canvas.clip_path(clip, ClipOp::Intersect, true /* antialias */);
      }
      canvas.set_matrix(&self.matrix.into());
    }
  }

  pub fn get_pixels(&mut self, origin: impl Into<IPoint>, dst_info:&ImageInfo, engine:RenderingEngine) -> Result<Data, String>{
    let src_info = ImageInfo::new_n32_premul(self.bounds.size().to_floor(), dst_info.color_space());
    let image = self.get_image().ok_or("Could not render bitmap")?; // use the cached bitmap if available

    engine.with_surface(&src_info, Some(0), |surface|{
      surface // draw to (potentially gpu-backed) rasterizer
        .canvas()
        .draw_image(image, -origin.into(), None);

      // copy pixels into buffer (and convert to requested color_type)
      let mut buffer: Vec<u8> = vec![0; dst_info.bytes_per_pixel() * (dst_info.width() * dst_info.height()) as usize];
      match surface.read_pixels(&dst_info, &mut buffer, dst_info.min_row_bytes(), (0,0)){
        true => Ok(Data::new_copy(&buffer)),
        false => Err(format!("Could get pixels in format: {:?}", dst_info.color_type()))
      }
    })
  }

  pub fn get_page(&mut self) -> Page{
    if self.changed {
      // stop and restart the recorder while adding its content as a new layer
      if let Some(palimpsest) = self.current.finish_recording_as_picture(Some(&self.bounds)) {
        self.layers.push(palimpsest);
      }
      self.current.begin_recording(self.bounds, None);
      self.changed = false;
      self.cache = None;
      self.restore();
    }

    Page{
      layers: self.layers.clone(),
      bounds: self.bounds,
    }
  }

  pub fn get_image(&mut self) -> Option<SkImage>{
    let page = self.get_page();
    if self.cache.is_none(){
      if let Some(pict) = page.get_picture(None){
        let size = page.bounds.size().to_floor();
        self.cache = images::deferred_from_picture(pict, size, None, None, BitDepth::U8, Some(ColorSpace::new_srgb()), None);
      }
    }
    self.cache.clone()
  }
}

//
// Image generator for a single drawing context
//

#[derive(Debug, Clone)]
pub struct Page{
  pub layers: Vec<Picture>,
  pub bounds: Rect,
}

impl Page{

  pub fn get_picture(&self, matte:Option<Color>) -> Option<Picture> {
    let mut compositor = PictureRecorder::new();
    compositor.begin_recording(self.bounds, None);
    if let Some(output) = compositor.recording_canvas() {
      matte.map(|c| output.clear(c));
      for pict in self.layers.iter(){
        pict.playback(output);
      }
    }
    compositor.finish_recording_as_picture(Some(&self.bounds))
  }

  pub fn encoded_as(&self, options:ExportOptions, engine:RenderingEngine) -> Result<Data, String> {
    let ExportOptions{ format, quality, density, outline, matte, msaa, color_type } = options;

    let picture = self.get_picture(matte).ok_or("Could not generate an image")?;
    if self.bounds.is_empty(){
      Err("Width and height must be non-zero to generate an image".to_string())
    }else{
      let img_dims = self.bounds.size();
      let img_format = match format.as_str() {
        "jpg" | "jpeg" => Some(EncodedImageFormat::JPEG),
        "png"          => Some(EncodedImageFormat::PNG),
        "webp"         => Some(EncodedImageFormat::WEBP),
        "raw"          => Some(EncodedImageFormat::BMP), // just use BMP as a flag, don't actually encode
        _ => None
      };

      if let Some(img_format) = img_format{
        let img_scale = Matrix::scale((density, density));
        let img_dims = Size::new(img_dims.width * density, img_dims.height * density).to_floor();
        let img_info = ImageInfo::new_n32_premul(img_dims, Some(ColorSpace::new_srgb()));

        engine.with_surface(&img_info, msaa, |surface|{
          surface // draw to (potentially gpu-backed) rasterizer
            .canvas()
            .set_matrix(&img_scale.into())
            .draw_picture(&picture, None, None);

          if format=="raw"{
            // copy raw pixels to buffer, then copy again to Data (waiting for skia_safe to implement Data.writable_data)
            let dst_info = ImageInfo::new(img_dims, color_type, AlphaType::Unpremul, Some(ColorSpace::new_srgb()));
            let mut buffer: Vec<u8> = vec![0; dst_info.bytes_per_pixel() * (img_dims.width * img_dims.height) as usize];
            match surface.read_pixels(&dst_info, &mut buffer, dst_info.min_row_bytes(), (0,0)){
              true => Ok(Data::new_copy(&buffer)),
              false => Err(format!("Could not encode as {} ({:?})", format, color_type))
            }
          }else{
            surface // generate bitmap in specified format
              .image_snapshot()
              .encode(&mut surface.direct_context(), img_format, (quality*100.0) as u32)
              .map(|data| with_dpi(data, img_format, density))
              .ok_or(format!("Could not encode as {}", format))
          }
        })
      }else if format == "pdf"{
        let mut pdf_bytes = Vec::new();
        let mut document = pdf_document(&mut pdf_bytes, quality, density).begin_page(img_dims, None);
        let canvas = document.canvas();
        canvas.draw_picture(&picture, None, None);
        document.end_page().close();
        Ok(Data::new_copy(&pdf_bytes))
      }else if format == "svg"{
        let flags = outline.then_some(Flags::CONVERT_TEXT_TO_PATHS);
        let canvas = svg::Canvas::new(Rect::from_size(img_dims), flags);
        canvas.draw_picture(&picture, None, None);
        Ok(canvas.end())
      }else{
        Err(format!("Unsupported file format {}", format))
      }
    }
  }

  pub fn write(&self, filename: &str, options:ExportOptions, engine:RenderingEngine) -> Result<(), String> {
    let path = FilePath::new(&filename);
    let data = self.encoded_as(options, engine)?;
    fs::write(path, data.as_bytes()).map_err(|why|
      format!("{}: \"{}\"", why, path.display())
    )
  }

  fn append_to<'a>(&self, doc:Document<'a>, matte:Option<Color>) -> Result<Document<'a>, String>{
    if !self.bounds.is_empty(){
      let mut doc = doc.begin_page(self.bounds.size(), None);
      let canvas = doc.canvas();
      if let Some(picture) = self.get_picture(matte){
        canvas.draw_picture(&picture, None, None);
      }
      Ok(doc.end_page())
    }else{
      Err("Width and height must be non-zero to generate a PDF page".to_string())
    }
  }
}


//
// Container for a canvas's entire stack of page contexts
//

pub struct PageSequence{
  pub pages: Vec<Page>,
  pub engine: RenderingEngine
}

impl PageSequence{
  pub fn from(pages:Vec<Page>, engine:RenderingEngine) -> Self{
    PageSequence { pages, engine }
  }

  pub fn first(&self) -> &Page {
    &self.pages[0]
  }

  pub fn len(&self) -> usize{
    self.pages.len()
  }

  pub fn as_pdf(&self, options:ExportOptions) -> Result<Data, String>{
    let ExportOptions{ quality, density, matte, .. } = options;
    let mut pdf_bytes = Vec::new();
    self.pages
      .iter()
      .try_fold(pdf_document(&mut pdf_bytes, quality, density), |doc, page| page.append_to(doc, matte))
      .map(|doc| doc.close())?;
    Ok(Data::new_copy(&pdf_bytes))
  }

  pub fn write_image(&self, pattern:&str, options:ExportOptions) -> Result<(), String>{
    self.first().write(pattern, options, self.engine)
  }

  #[allow(clippy::too_many_arguments)]
  pub fn write_sequence(&self, pattern:&str, padding:f32, options:ExportOptions) -> Result<(), String>{
    let padding = match padding as i32{
      -1 => (1.0 + (self.pages.len() as f32).log10().floor()) as usize,
      pad => pad as usize
    };

    self.pages
      .par_iter()
      .enumerate()
      .try_for_each(|(pp, page)|{
        let folio = format!("{:0width$}", pp+1, width=padding);
        let filename = pattern.replace("{}", folio.as_str());
        page.write(&filename, options.clone(), self.engine)
      })
  }

  pub fn write_pdf(&self, path:&str, options:ExportOptions) -> Result<(), String>{
    let path = FilePath::new(&path);
    match self.as_pdf(options){
      Ok(document) => fs::write(path, document.as_bytes()).map_err(|why|
        format!("{}: \"{}\"", why, path.display())
      ),
      Err(msg) => Err(msg)
    }
  }

}

//
// Helpers
//

pub fn pages_arg(cx: &mut FunctionContext, idx:usize, canvas:&BoxedCanvas) -> NeonResult<PageSequence> {
  let engine = canvas.borrow_mut().engine();
  let pages = cx.argument::<JsArray>(idx)?
      .to_vec(cx)?
      .iter()
      .map(|obj| obj.downcast::<BoxedContext2D, _>(cx))
      .filter( |ctx| ctx.is_ok() )
      .map(|obj| obj.unwrap().borrow().get_page())
      .collect();
  Ok(PageSequence::from(pages, engine))
}

fn pdf_document(buffer:&mut impl std::io::Write, quality:f32, density:f32) -> Document{
  pdf::new_document(buffer, Some(&pdf::Metadata {
    producer: "Skia Canvas <https://github.com/samizdatco/skia-canvas>".to_string(),
    encoding_quality: Some((quality*100.0) as i32),
    raster_dpi: Some(density * 72.0),
    ..Default::default()
  }))
}

fn with_dpi(data:Data, format:EncodedImageFormat, density:f32) -> Data{
  if density as u32 == 1 { return data }

  let mut bytes = data.as_bytes().to_vec();
  match format{
    EncodedImageFormat::JPEG => {
      let [l, r] = (72 * density as u16).to_be_bytes();
      bytes.splice(13..18, [1, l, r, l, r].iter().cloned());
      Data::new_copy(&bytes)
    }
    EncodedImageFormat::PNG => {
      let mut digest = CRC32.digest();
      let [a, b, c, d] = ((72.0 * density * 39.3701) as u32).to_be_bytes();
      let phys = vec![
        b'p', b'H', b'Y', b's',
        a, b, c, d, // x-dpi
        a, b, c, d, // y-dpi
        1, // dots per meter
      ];
      digest.update(&phys);

      let length = 9u32.to_be_bytes().to_vec();
      let checksum = digest.finalize().to_be_bytes().to_vec();
      bytes.splice(33..33, [length, phys, checksum].concat());
      Data::new_copy(&bytes)
    }
    _ => data
  }
}

#[derive(Clone)]
pub struct ExportOptions{
  pub format: String,
  pub quality: f32,
  pub density: f32,
  pub outline: bool,
  pub matte: Option<Color>,
  pub msaa: Option<usize>,
  pub color_type: ColorType
}
