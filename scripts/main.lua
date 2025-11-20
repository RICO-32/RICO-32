local player = require("player")

function start()
    log("Start function")
end

function update(dt)
	player.upd()
	if player.x % 10 == 0 then
		log("Player's X is: " .. tostring(player.x) .. " and " .. tostring(dt) .. " time has passed")
	end
end
