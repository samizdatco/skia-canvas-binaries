const path = require('path'),
    {Canvas, FontLibrary} = require('./lib')

let file = path.normalize('test/assets/AmstelvarAlpha-VF.ttf')
console.log({file})
FontLibrary.use(file)
