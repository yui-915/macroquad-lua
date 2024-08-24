# macroquad-lua

Building games with macroquad using lua instead of rust

## DO NOT USE

this project is not done and will probably never be, thus you should not even think about using it for making anything more than a moving square

## Example

```lua
mq = require("macroquad")
// equivalent of use macroquad::prelude::*;
mq.extra.global_use(mq.prelude)

x = 0

function mq.update()
  x = x + 5
  if x > screen_width() then
    x = 0
  end
end

function mq.draw()
  clear_background(ORANGE)
  draw_rectangle(x, 50, 120, 60, BLACK)
end
```

## Features

- hot-reloading in dev mode
- embed lua source files in release mode
- ...

## Running

The recommended way of using the project is `macroquad-lua-cli`

You'll need cargo with a working rust compiler

```bash
# installing the cli
cargo install macroquad-lua-cli
# if you have PATH setup correctly
# you should be able to just run `mql help`

# installing/updating macroquad-lua using the cli
mql update

# creating a new project
mql new project-name
cd project-name

# running in dev mode
mql run

# building in release mode
mql build

# if you have rust cross-compilation working
# you can do something like
mql build --target x86_64-pc-windows-gnu
# (cross compiling from linux host to windows target)

# is you have a working emscripten compiler and add to PATH
# you can build directly to web
mql build --target emscripten
```

\*Tested on linux and windows only

While it's possible to build apk for android (using [cargo-quad-apk](https://github.com/not-fl3/cargo-quad-apk)), it hasn't been implemented to macroquad-lau-cli yet

idk about MacOS and IOS

## Roadmap

check the `TODO` and `TODO-API` files

## Contributing

Why?

## Also see

- macroquad ([website](https://macroquad.rs/) - [github](https://github.com/not-fl3/macroquad)): awesome game library
- mlua ([github](https://github.com/mlua-rs/mlua)): lua bindings for rust
- readme.so ([website](https://readme.so/editor) - [github](https://github.com/octokatherine/readme.so)): online editor for making readmes
