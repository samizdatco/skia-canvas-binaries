const {Canvas, FontLibrary} = require('skia-canvas')
let WIDTH = 512, HEIGHT = 128,
    canvas = new Canvas(WIDTH, HEIGHT),
    ctx = canvas.getContext("2d");

let fnt = FontLibrary.use('Arista.ttf')
console.log(fnt);

function textFit(text, font, px, x, y, maxWidth) {
  for (let index = 0; index < px; index++) {
      ctx.font = `${px - index}px ${font}`
      let txt = ctx.measureText(text)
      console.log(px - index, txt.width);
      if (txt.width < maxWidth) {
          ctx.fillText(text, x, y);
          return px-index
      }
  }
}

let size = textFit('Hello', 'Arista', 40, 196, 50, 315)
console.log(size);
