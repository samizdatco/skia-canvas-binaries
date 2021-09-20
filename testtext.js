const {Canvas, FontLibrary} = require('skia-canvas')
let WIDTH = 512, HEIGHT = 128,
    canvas = new Canvas(WIDTH, HEIGHT),
    ctx = canvas.getContext("2d");

ctx.font = '80px Arial'
ctx.fillText('ABC abc 123', 10, 80)
ctx.fillRect(10, 100, WIDTH-20, 10)
canvas.saveAs('abc.png', {density:2})