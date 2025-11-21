local player = require("player")

function start()
    log("Start function")
end

function update(dt)
	player.upd()
	if player.x % 10 == 0 then
		log("Player's X is: " .. tostring(player.x) .. " and " .. tostring(dt) .. " time has passed")
	end
	if player.x % 20 == 0 then
		draw(1, 2, "WOWAZA")
		button(1, 2, "WOWAZA")
		print_scr(1, 2, "WOWAZA")
	end
	if player.x % 100 == 0 then
		local new_frame_rate = math.floor(player.x / 10)
		log("Changing framerate to " .. tostring(new_frame_rate))
		set_frame_rate(new_frame_rate)
	end
end
