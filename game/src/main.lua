local mq = require("macroquad").prelude
local game = {}

function game.start()
	X = 0
end

function game.update()
	X = X + 0.5
	mq.draw_rectangle_lines(X, 0, 100, 100, 50, mq.RED)
end

return game
