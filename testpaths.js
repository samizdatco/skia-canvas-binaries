let {globSync, convertPathToPattern} = require('fast-glob')

const ASSETS_DIR = path.join(__dirname, 'test/assets'),
      FONTS_DIR = path.join(ASSETS_DIR, 'fonts')

function glob(pat){
  console.log('pattern:', pat)
  console.log(globSync(pat))
}
glob(`${FONTS_DIR}/montserrat*/montserrat-v30-latin-italic.woff2`)
glob(`${FONTS_DIR}/montserrat-latin/*700*.woff2`)
glob(`${ASSETS_DIR}/**/montserrat-v30-latin-italic.woff2`)
glob(`${ASSETS_DIR}/**/montserrat*italic.*`)
