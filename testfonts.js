const path = require('path'),
    {Canvas, FontLibrary} = require('./lib'),
    {globSync, convertPathToPattern} = require('fast-glob')

let file = path.normalize('test/assets/*.ttf')

// let pat = convertPathToPattern(file)
// let matchF = globSync(file, {dot:true})
// let matchP = globSync(pat, {dot:true})

// console.log({file, matchF, pat, matchP})


console.log({file})
FontLibrary.use(file)
