local mq = require("macroquad").prelude
local colors = require("macroquad").color.colors
local game = {}

function game.start()
	X = 0
	Y = 0
end

function game.update()
	-- if mq.is_key_down(mq.KeyCode.D) then
	-- 	X = X + 5
	-- end
	-- if mq.is_key_down(mq.KeyCode.A) then
	-- 	X = X - 5
	-- end
	-- if mq.is_key_down(mq.KeyCode.S) then
	-- 	Y = Y + 5
	-- end
	-- if mq.is_key_down(mq.KeyCode.W) then
	-- 	Y = Y - 5
	-- end

	-- local keys = mq.get_keys_down()
	-- print(keys)
	-- if keys then
	-- 	for key in pairs(keys) do
	-- 		print(key)
	-- 	end
	-- 	print("===================")
	-- end

	-- if mq.is_mouse_button_down(mq.MouseButton.Left) then
	-- 	X = X + 5
	-- 	Y = Y + 5
	-- end

	-- print(mq.get_last_key_pressed())

	mq.draw_text("Hello World", 100, 100, 50, mq.RED)

	mq.draw_rectangle_lines(X, Y, 100, 100, 15, get_random_element(colors))
end

function get_random_element(input_table)
	local keyset = {}
	for k in pairs(input_table) do
		table.insert(keyset, k)
	end
	return input_table[keyset[math.random(#keyset)]]
end

return game
