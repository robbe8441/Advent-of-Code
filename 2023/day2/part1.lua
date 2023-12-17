local io, game_id, answer = require("io"), 0, 0

for line in io.lines("./input.txt") do
    game_id = game_id + 1
    local line = line:gsub("Game %d+:", ""):gsub(" ", "") .. ";"

    local function processGame(game)
        local restlts = {green = 0, red = 0, blue = 0}
        for count, color in game:gmatch("(%d+)(%w+)") do
            restlts[color] = restlts[color] + tonumber(count)
        end
        return restlts.green <= 13 and restlts.red <= 12 and restlts.blue <= 14
    end

    for game in string.gmatch(line, "([^;]+);") do
        if not processGame(game) then goto nextGame end
    end

    answer = answer + game_id
    ::nextGame::
end

print("answer", answer)
