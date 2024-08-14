local mq = require("macroquad")
mq.extra.global_use(mq.prelude)
local game = {}

function game.start() end

function a_function_that_exist()
	a_function_that_doesnt_exist()
end

local frame = 0
function game.update()
	if is_key_down(KeyCode.S) then
		frame = frame - 1
	else
		frame = frame + 1
	end
	draw_text("Hello World " .. frame, 100, 100, 50, WHITE)
	if frame == 180 then
		a_function_that_exist()
	end
end

function game.panic_update(err)
	clear_background(RED)
	draw_multiline_text(err, 10, 15, 24, 1, BLACK)
end

return game
