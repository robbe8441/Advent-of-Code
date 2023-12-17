local io, total_cubes = require("io"), 0

for line in io.lines("./input.txt") do
    local line = line:gsub("Game %d+:", ""):gsub(" ", "") .. ";"
    local restlts = {green = 0, red = 0, blue = 0}

    for count, color in line:gmatch("(%d+)(%w+)") do
        restlts[color] = math.max(restlts[color], tonumber(count))
    end

    total_cubes = total_cubes + restlts.blue * restlts.green * restlts.red
    print(total_cubes)
end

print("answer", total_cubes)
