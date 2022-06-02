const fs = require('fs'),
      tmp = require('tmp'),
      glob = require('glob').sync

TMP = tmp.dirSync().name
console.log('DIR', TMP)

fs.writeFileSync(`${TMP}/output-01.png`)
fs.writeFileSync(`${TMP}/output-02.png`)
fs.writeFileSync(`${TMP}/output-03.png`)
fs.writeFileSync(`${TMP}/output-04.png`)
fs.writeFileSync(`${TMP}/output-05.png`)
fs.writeFileSync(`${TMP}/output-06.png`)

let qmark = glob(`${TMP}/output-0?.png`),
    star = glob(`${TMP}/output-*.png`)

console.log({qmark, star})
