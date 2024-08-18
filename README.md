
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


## Running

It's as simple as

```bash
git clone https://github.com/yui-915/macroquad-lua
cd macroquad-lua
cargo run
```
You can start doing changes in `game/src/main.lua`\
then you can build it as a standalone executable using
```bash
cargo build --release
```
which you can find in `target/release/macroquad-lua`


## Features

- hot-reloading in dev mode
- boundle lua source files in release mode
- ...

## Platform support
- Only tested on linux & windows
- macos is not tested but it should work (?)
- web builds are not possible right now
- android and ios are not tested but they shouldn't work

## Roadmap
check the `TODO` and `TODO-API` files
## Contributing

Why?


## Acknowledgements

 - [macroquad](https://github.com/not-fl3/macroquad)
 - [mlua](https://github.com/mlua-rs/mlua)
