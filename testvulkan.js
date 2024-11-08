const {Canvas, FontLibrary} = require('./lib')
let WIDTH = 512, HEIGHT = 128,
    canvas = new Canvas(WIDTH, HEIGHT),
    ctx = canvas.getContext("2d");


console.log(canvas.engine)
