use std::fs;
use std::io::Write;
use std::path::Path;
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use skia_safe::{icu, Rect, Surface, Canvas, FontMgr, Paint, Point, EncodedImageFormat, PictureRecorder};

static LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur at leo at nulla tincidunt placerat. Proin eget purus augue. Quisque et est ullamcorper, pellentesque felis nec, pulvinar massa. Aliquam imperdiet, nulla ut dictum euismod, purus dui pulvinar risus, eu suscipit elit neque ac est. Nullam eleifend justo quis placerat ultricies. Vestibulum ut elementum velit. Praesent et dolor sit amet purus bibendum mattis. Aliquam erat volutpat.";

fn main() {
    let width = 256;
    let height = 64;

    let mut surface = Surface::new_raster_n32_premul((width * 2, height * 2)).unwrap();
    let mut canvas = surface.canvas();
    canvas.scale((2.0, 2.0));
    draw_lorem_ipsum(&mut canvas);
    save_as(&mut surface, "immediate.png");


    let mut layer_recorder = PictureRecorder::new();
    let bounds = Rect::from_xywh(0.0, 0.0, width as f32, height as f32);
    layer_recorder.begin_recording(bounds, None);
    if let Some(mut layer) = layer_recorder.recording_canvas() {
        draw_lorem_ipsum(&mut layer);
    }

    let mut surface = Surface::new_raster_n32_premul((width * 2, height * 2)).unwrap();
    let canvas = surface.canvas();
    canvas.scale((2.0, 2.0));
    if let Some(pict) = layer_recorder.finish_recording_as_picture(Some(&bounds)){
        canvas.draw_picture(&pict, None, None);
    }
    save_as(&mut surface, "recorded.png");
}


fn draw_lorem_ipsum(canvas: &mut Canvas) {
    icu::init();
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(FontMgr::new(), None);
    let paragraph_style = ParagraphStyle::new();
    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
    let mut ts = TextStyle::new();
    ts.set_foreground_color(Paint::default());
    paragraph_builder.push_style(&ts);
    paragraph_builder.add_text(LOREM_IPSUM);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(256.0);
    paragraph.paint(canvas, Point::default());
}

fn save_as(surface: &mut Surface, name: &str){
    let image = surface.image_snapshot();
    let data = image.encode_to_data(EncodedImageFormat::PNG).unwrap();
    let file_path = Path::new(name);
    let mut file = fs::File::create(file_path).expect("failed to create file");
    file.write_all(data.as_bytes()).expect("failed to write to file");
}