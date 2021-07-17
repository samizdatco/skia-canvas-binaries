const {Canvas, FontLibrary} = require('./lib')
let WIDTH = 512,
    HEIGHT = 512;
    canvas = new Canvas(WIDTH, HEIGHT),
    ctx = canvas.getContext("2d")

// ctx.font = '80px Arial'
ctx.fillText('ABC abc 123', 10, 200)

ctx.fillRect(10, 220, WIDTH-20, 10)
canvas.saveAs('abc.png', {density:2})