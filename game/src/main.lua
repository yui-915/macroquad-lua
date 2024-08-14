local mq = require("macroquad")
mq.extra.global_use(mq.prelude)
local colors = require("macroquad").color.colors
local game = {}

function game.start()
	X = 0
	Y = 0
end

function game.update()
	-- if is_key_down(KeyCode.D) then
	-- 	X = X + 5
	-- end
	-- if is_key_down(KeyCode.A) then
	-- 	X = X - 5
	-- end
	-- if is_key_down(KeyCode.S) then
	-- 	Y = Y + 5
	-- end
	-- if is_key_down(KeyCode.W) then
	-- 	Y = Y - 5
	-- end

	-- local keys = get_keys_down()
	-- print(keys)
	-- if keys then
	-- 	for key in pairs(keys) do
	-- 		print(key)
	-- 	end
	-- 	print("===================")
	-- end

	-- if is_mouse_button_down(MouseButton.Left) then
	-- 	X = X + 5
	-- 	Y = Y + 5
	-- end

	-- print(get_last_key_pressed())

	draw_text("Hello World", 100, 100, 50, RED)

	draw_rectangle_lines(X, Y, 100, 100, 15, get_random_element(colors))
end

function get_random_element(input_table)
	local keyset = {}
	for k in pairs(input_table) do
		table.insert(keyset, k)
	end
	return input_table[keyset[math.random(#keyset)]]
end

return game
