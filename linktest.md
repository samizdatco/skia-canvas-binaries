<a href="https://skia-canvas.org">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/assets/hero-dark@2x.png">
  <img alt="Skia Canvas" src="docs/assets/hero@2x.png">
</picture>

</a>

---

<div align="center">
  <a href="http://skia-canvas.org/getting-started">Getting Started</a> <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="http://skia-canvas.org/api">Documentation</a> <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="http://skia-canvas.org/releases">Release Notes</a>
</div>

---

Skia Canvas is a browser-less implementation of the HTML Canvas drawing API for Node.js. It is based on Google’s [Skia](https://skia.org) graphics engine and, accordingly, produces very similar results to Chrome’s `<canvas>` element. The library is well suited for use on desktop machines where you can render hardware-accelerated graphics to a window and on the server where it can output a variety of image formats.

While the primary goal of this project is to provide a reliable emulation of the [standard API](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API) according to the [spec](https://html.spec.whatwg.org/multipage/canvas.html), it also extends it in a number of areas to take greater advantage of Skia's advanced graphical features and provide a more expressive coding environment.

In particular, Skia Canvas:

  - is fast and compact since rendering takes place on the GPU and all the heavy lifting is done by native code written in Rust and C++
  - can render to [windows](#window) using an OS-native graphics pipeline and provides a browser-like [UI event][win_bind] framework
  - generates output in both raster (JPEG & PNG) and vector (PDF & SVG) image formats
  - can save images to [files][saveAs], return them as [Buffers][toBuffer], or encode [dataURL][toDataURL_ext] strings
  - uses native threads and the Node [worker pool](https://github.com/neon-bindings/rfcs/pull/35) for asynchronous rendering and file I/O
  - can create [multiple ‘pages’][newPage] on a given canvas and then [output][saveAs] them as a single, multi-page PDF or an image-sequence saved to multiple files
  - can [simplify][p2d_simplify], [blunt][p2d_round], [combine][bool-ops], [excerpt][p2d_trim], and [atomize][p2d_points] bézier paths using [efficient](https://www.youtube.com/watch?v=OmfliNQsk88) boolean operations or point-by-point [interpolation][p2d_interpolate]
  - provides [3D perspective][createProjection()] transformations in addition to [scaling][scale()], [rotation][rotate()], and [translation][translate()]
  - can fill shapes with vector-based [Textures][createTexture()] in addition to bitmap-based [Patterns][createPattern()] and supports line-drawing with custom [markers][lineDashMarker]
  - supports the full set of [CSS filter][filter] image processing operators
  - offers rich typographic control including:
    - multi-line, [word-wrapped](#textwrap) text
    - line-by-line [text metrics](#measuretextstr-width)
    - small-caps, ligatures, and other opentype features accessible using standard [font-variant](#fontvariant) syntax
    - proportional [letter-spacing][letterSpacing], [word-spacing][wordSpacing], and [leading](#font)
    - support for [variable fonts][VariableFonts] and transparent mapping of weight values
    - use of non-system fonts [loaded](#usefamilyname-fontpaths) from local files

## Installation

If you’re running on a supported platform, installation should be as simple as:
```console
npm install skia-canvas
```

This will download a pre-compiled library from the project’s most recent [release](https://github.com/samizdatco/skia-canvas/releases).

### Platform Support

The underlying Rust library uses [N-API](https://nodejs.org/api/n-api.html) v6 which allows it to run on Node.js versions:
  - 10.20+
  - 12.17+
  - 14.0, 15.0, and later

Pre-compiled binaries are available for:

  - Linux (x64, arm64, & armhf)
  - macOS (x64 & Apple silicon)
  - Windows (x64)

Nearly everything you need is statically linked into the library. A notable exception is the [Fontconfig](https://www.freedesktop.org/wiki/Software/fontconfig/) library which must be installed separately if you’re running on Linux.


### Running in Docker

The library is compatible with Linux systems using [glibc](https://www.gnu.org/software/libc/) 2.28 or later as well as Alpine Linux (x64 & arm64) and the [musl](https://musl.libc.org) C library it favors. In both cases, Fontconfig must be installed on the system for `skia-canvas` to operate correctly.

If you are setting up a [Dockerfile](https://nodejs.org/en/docs/guides/nodejs-docker-webapp/) that uses [`node`](https://hub.docker.com/_/node) as its basis, the simplest approach is to set your `FROM` image to one of the (Debian-derived) defaults like `node:lts`, `node:18`, `node:16`, `node:14-buster`, `node:12-buster`, `node:bullseye`, `node:buster`, or simply:
```dockerfile
FROM node
```

You can also use the ‘slim’ image if you manually install fontconfig:

```dockerfile
FROM node:slim
RUN apt-get update && apt-get install -y -q --no-install-recommends libfontconfig1
```

If you wish to use Alpine as the underlying distribution, you can start with something along the lines of:

```dockerfile
FROM node:alpine
RUN apk update && apk add fontconfig
```

### Compiling from Source

If prebuilt binaries aren’t available for your system you’ll need to compile the portions of this library that directly interface with Skia.

Start by installing:

  1. The [Rust compiler](https://www.rust-lang.org/tools/install) and cargo package manager using [`rustup`](https://rust-lang.github.io/rustup/)
  2. A C compiler toolchain like LLVM/Clang or MSVC
  3. Python 2.7 (used by Skia's [build process](https://skia.org/docs/user/build/))
  4. On Linux: Fontconfig and OpenSSL

[Detailed instructions](https://github.com/rust-skia/rust-skia#building) for setting up these dependencies on different operating systems can be found in the ‘Building’ section of the Rust Skia documentation. Once all the necessary compilers and libraries are present, running `npm run build` will give you a usable library (after a fairly lengthy compilation process).

## Example Usage

#### Generating image files

```js
const {Canvas} = require('skia-canvas')

let canvas = new Canvas(400, 400),
    {width, height} = canvas,
    ctx = canvas.getContext("2d");

let sweep = ctx.createConicGradient(Math.PI * 1.2, width/2, height/2)
sweep.addColorStop(0, "red")
sweep.addColorStop(0.25, "orange")
sweep.addColorStop(0.5, "yellow")
sweep.addColorStop(0.75, "green")
sweep.addColorStop(1, "red")
ctx.strokeStyle = sweep
ctx.lineWidth = 100
ctx.strokeRect(100,100, 200,200)

// render to multiple destinations using a background thread
async function render(){
  // save a ‘retina’ image...
  await canvas.saveAs("rainbox.png", {density:2})
  // ...or use a shorthand for canvas.toBuffer("png")
  let pngData = await canvas.png
  // ...or embed it in a string
  let pngEmbed = `<img src="${await canvas.toDataURL("png")}">`
}
render()

// ...or save the file synchronously from the main thread
canvas.saveAsSync("rainbox.pdf")
```

#### Multi-page sequences

```js
const {Canvas} = require('skia-canvas')

let canvas = new Canvas(400, 400),
    {width, height} = canvas;

for (const color of ['orange', 'yellow', 'green', 'skyblue', 'purple']){
  ctx = canvas.newPage()
  ctx.fillStyle = color
  ctx.fillRect(0,0, width, height)
  ctx.fillStyle = 'white'
  ctx.arc(width/2, height/2, 40, 0, 2 * Math.PI)
  ctx.fill()
}

async function render(){
  // save to a multi-page PDF file
  await canvas.saveAs("all-pages.pdf")

  // save to files named `page-01.png`, `page-02.png`, etc.
  await canvas.saveAs("page-{2}.png")
}
render()

```

#### Rendering to a window

```js
const {Window} = require('skia-canvas')

let win = new Window(300, 300)
win.title = "Canvas Window"
win.on("draw", e => {
  let ctx = e.target.canvas.getContext("2d")
  ctx.lineWidth = 25 + 25 * Math.cos(e.frame / 10)
  ctx.beginPath()
  ctx.arc(150, 150, 50, 0, 2 * Math.PI)
  ctx.stroke()

  ctx.beginPath()
  ctx.arc(150, 150, 10, 0, 2 * Math.PI)
  ctx.stroke()
  ctx.fill()
})
```
