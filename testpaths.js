let {globSync, convertPathToPattern} = require('fast-glob'),
    path = require('path')

/**
 * The device path (\\.\ or \\?\).
 * https://learn.microsoft.com/en-us/dotnet/standard/io/file-path-formats#dos-device-paths
 */
const DOS_DEVICE_PATH_RE = /^\\\\(?<path>[.?])/;
/**
 * All backslashes except those escaping special characters.
 * Windows: !()+@{}
 * https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file#naming-conventions
 */
const WINDOWS_BACKSLASHES_RE = /\\(?![!()+@[\]{}])/g;

const ROOT_DIR = __dirname.replace()
        .replace(DOS_DEVICE_PATH_RE, '//$1')
    		.replaceAll(WINDOWS_BACKSLASHES_RE, '/')

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
