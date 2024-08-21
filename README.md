
# macroquad-lua

love macroquad but hate rust, so use lua instead


## DO NOT USE
this project is still in devolopment and will probably be forever, thus you should not even think about using it for making anything more than a moving square
## Example

```lua
mq = require("macroquad")
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
- boundle lua source files in release mode
- ...

## Running

It's as simple as

```bash
git clone https://github.com/yui-915/macroquad-lua
cd macroquad-lua
cargo run
```
You can start doing changes in `game/src/main.lua`



## Building

For linux and windows (MacOS not tested) it's as simple as
```bash
cargo build --release
```
then you can find it in `target/release/macroquad-lua`
\
\
For android you can follow [this macroquad atricle](https://macroquad.rs/articles/android/)\
a few notes if you're going to build without using docker:
- you might have to change `android_version` and `target_sdk_version` in `Cargo.toml` depending on the `android-sdk` version you have installed
- if you're getting an `no such file or directory` error without any context, if could be due to [this function](https://github.com/not-fl3/cargo-quad-apk/blob/master/src/ops/build/util.rs#L167) trying to read a folder called `lib64`, check your android-ndk installation (eg. `android-ndk/toolchains/llvm/prebuilt/linux-x86_64/`), if it only contains a `lib` but not `lib64`, you can try using [my fork](https://github.com/yui-915/cargo-quad-apk) instead (`cargo uninstall cargo-quad-apk` and `cargo install --git https://github.com/yui-915/cargo-quad-apk.git`)
-  some java installation (like `java-17-graalvm-ee`) doesn't have `rt.jar` which is required for the build, I'd recommend just sticking to `java-8-openjdk`, also `cargo-quad-apk` priortize the java installation in your `PATH` over `JAVA_HOME`

\
For web we have to use `wasm32-unknown-emscripten` instead of `wasm32-unknown-unknown` to build lua\
as long as emscripten correctly setup, and `emcc` and emscripten provided `clang` available in `PATH`, you should be able to just do
```bash
cargo build --release --target wasm32-unknown-emscripten
```
then use the `js` and `wasm` files from `target/wasm32-unknown-emscripten/release` and `index.js` from the `web` folder

(I should make a docker image for it at some point)
\
\
About ios ..... idk

## Roadmap
check the `TODO` and `TODO-API` files
## Contributing

Why?


## Acknowledgements

 - [macroquad](https://github.com/not-fl3/macroquad)
 - [mlua](https://github.com/mlua-rs/mlua)
 - [readme.so](https://github.com/octokatherine/readme.so)
