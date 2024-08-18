-- pub use macroquad as mq;
mq = require("macroquad")

-- pub use mq::prelude::*;
mq.extra.global_use(mq.prelude)

-- only gets called once, when the game starts
-- optional
function mq.init()
	X = 0
end

-- gets called everytime the game is loaded
-- ie. when hot-reloading in dev mode, or when the game first starts in release mode
-- optional
function mq.load()
	Y = 0
end

-- gets called every frame before mq.draw
-- optional if mq.draw is provided
function mq.update() end

-- ^ this is just a placeholder and removing it won't change anything
-- you can import other files and have them implement the function
require("update")

-- gets called every frame after mq.update
-- optional if mq.update is provided
function mq.draw()
	clear_background(ORANGE)
	draw_text("X: " .. X, 100, 100, 50, BLACK)
	draw_text("Y:." .. Y, 100, 200, 50, BLACK)
end

-- gets called once everytime the game panics (throws an error)
-- could get called multiple times in dev mode (ie. after reloading)
-- takes a string as an argument (the error message)
-- mq.load might not be called by the time the game panics
-- if this function errors the game will just quit
-- optional, if not provided the game will just quit
local error = ""
function mq.panic(err)
	error = err
end

-- gets called every frame while the game is in a panic state (after mq.panic)
-- if this function errors the game will just quit
-- optional, if not provided the game will just quit
function mq.panic_draw()
	clear_background(RED)
	draw_multiline_text(error, 36, 36, 24, 1, BLACK)
end
