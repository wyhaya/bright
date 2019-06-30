
# bright

[![Build Status](https://img.shields.io/travis/wyhaya/bright.svg?style=flat-square)](https://travis-ci.org/wyhaya/bright)
[![Crates.io](https://img.shields.io/crates/v/bright.svg?style=flat-square)](https://crates.io/crates/bright)
[![LICENSE](https://img.shields.io/crates/l/bright.svg?style=flat-square)](https://github.com/wyhaya/bright/blob/master/LICENSE)

![preview](https://user-images.githubusercontent.com/23690145/59586970-beecaa80-9116-11e9-88c4-e0335096115f.png)

> Beautiful terminal color

## Install

Add this in your `Cargo.toml`:

```toml
[dependencies]
bright = "0.1.0"
```

## Usage
```rust
use bright::*;

fn main() {
    println!("{}", "Hello world".bold().red().bg_green());
}
```

## Styles

### Modifiers

* `.bold()`
* `.dim()`
* `.italic()`
* `.underline()`
* `.slow_blink()`
* `.fast_blink()`
* `.invert()`
* `.hidden()`
* `.cross_out()`

### Colors
* `.black()`
* `.red()`
* `.green()`
* `.yellow()`
* `.blue()`
* `.magenta()`
* `.cyan()`
* `.white()`
* `.rgb(r, g, b)`

### Background colors

* `.bg_black()`
* `.bg_red()`
* `.bg_green()`
* `.bg_yellow()`
* `.bg_blue()`
* `.bg_magenta()`
* `.bg_cyan()`
* `.bg_white()`
* `.bg_rgb(r, g, b)`


