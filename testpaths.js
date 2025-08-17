let {globSync, convertPathToPattern} = require('fast-glob'),
    path = require('path')

// const ROOT_DIR = __dirname
// const ROOT_DIR = 'D:\\a\\skia-canvas-binaries\\skia-canvas-binaries'.replace()
//         .replace(/^\\\\(?<path>[.?])/, '//$1') // The device path (\\.\ or \\?\).
//     		.replaceAll(/\\(?![!()+@[\]{}])/g, '/') // All backslashes except those escaping special characters.
// console.log({ROOT_DIR})
const ASSETS_DIR = path.join(__dirname, 'test/assets'),
      FONTS_DIR = path.join(ASSETS_DIR, 'fonts')

function glob(pat){
  pat = pat.replace(/^\\\\(?<path>[.?])/, '//$1') // The device path (\\.\ or \\?\).
    .replaceAll(/\\(?![!()+@[\]{}])/g, '/') // All backslashes except those escaping special characters.
  console.log('pattern:', pat)
  console.log(globSync(pat))
}
glob(path.join(FONTS_DIR,`montserrat*`,`montserrat-v30-latin-italic.woff2`))
glob(path.join(FONTS_DIR,`montserrat-latin`,`*700*.woff2`))
glob(path.join(ASSETS_DIR,`**`,`montserrat-v30-latin-italic.woff2`))
glob(path.join(ASSETS_DIR,`**`,`montserrat*italic.*`))
