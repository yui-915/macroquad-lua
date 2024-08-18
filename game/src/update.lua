-- all variables were declared globally in main.lua (X, Y and mq)
-- you can access them here
function mq.update()
  X = X + 1
  Y = Y + 2

  -- emulate an error
  if X + Y > 500 then
    a_function_that_doesnt_exist()
  end
end
