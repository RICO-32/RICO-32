local player = require("player")

function start()
    log("Start function")
end

math.randomseed(os.time())
function update(dt)
	player.upd()
	if player.x % 10 == 0 then
		log("Player's X is: " .. tostring(player.x) .. " and " .. tostring(dt) .. " time has passed")

		local x = math.random(0, 127)
		local y = math.random(0, 127)
		local cols = {"GREEN", "CYAN", "BLUE", "RED", "TEAL"}
		local col = cols[math.random(1, #cols)]
		local old_col = get_pix(x, y)
		log("Pixel " .. tostring(x) .. " " .. tostring(y) .. " was " .. old_col)
		set_pix(x, y, col)
		local new_col = get_pix(x, y)
		log("Pixel " .. tostring(x) .. " " .. tostring(y) .. " was " .. new_col)
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
