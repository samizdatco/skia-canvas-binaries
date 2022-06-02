const fs = require('fs'),
      tmp = require('tmp'),
      glob = require('glob').sync

TMP = tmp.dirSync().name
console.log('DIR', TMP)

fs.writeFileSync(`${TMP}/output-01.png`, 'empty')
fs.writeFileSync(`${TMP}/output-02.png`, 'empty')
fs.writeFileSync(`${TMP}/output-03.png`, 'empty')
fs.writeFileSync(`${TMP}/output-04.png`, 'empty')
fs.writeFileSync(`${TMP}/output-05.png`, 'empty')
fs.writeFileSync(`${TMP}/output-06.png`, 'empty')

let qmark = glob(`${TMP}/output-0?.png`),
    star = glob(`${TMP}/output-*.png`)

console.log({qmark, star})
