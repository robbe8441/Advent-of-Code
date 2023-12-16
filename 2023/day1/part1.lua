local io = require("io")
local res = 0

for i in io.lines("./input.txt") do
    local a = i:match("%d")
    local b = i:reverse():match("%d")

    if a and b then
        local num =  a*10 + b
        res = res + num
    end
end

print(res)