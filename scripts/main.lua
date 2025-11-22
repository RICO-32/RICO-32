local player = require("player")

function start()
    log("Start function")
end

math.randomseed(os.time())
function update(dt)
	player.upd()
	clear("BLACK")
	draw(1, 2, "main.png")
	print_scr(1, 50, "GREEN", "WOWAZA")
	draw(1, 45, "correct.png")
	if player.x % 10 == 0 then
		log("Player's X is: " .. tostring(player.x) .. " and " .. tostring(dt) .. " time has passed")

		local x = math.random(0, 127)
		local y = math.random(0, 127)
		local cols = {"GREEN", "CYAN", "BLUE", "RED", "TEAL"}
		local col = cols[math.random(1, #cols)]
		log("Pixel " .. tostring(x) .. " " .. tostring(y) .. " was " .. get_pix(x,y))
		set_pix(x, y, col)
		log("Pixel " .. tostring(x) .. " " .. tostring(y) .. " is " .. get_pix(x,y))
	end
	if player.x % 20 == 0 then
		clear("GREEN")
	end
	if player.x % 100 == 0 then
		local new_frame_rate = math.floor(player.x / 10)
		log("Changing framerate to " .. tostring(new_frame_rate))
		set_frame_rate(new_frame_rate)
	end
end
