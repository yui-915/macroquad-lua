local mq = require("macroquad").prelude
local game = {}

function game.start()
	X = 0
end

function game.update()
	X = X + 0.5
	mq.draw_rectangle(X, 0, 100, 100, mq.RED)
end

return game
