local player = require("player")

function start()
    log("Start function")
end

function button(x, y, w, h, label)
    rectfill(x, y, w, h, "CYAN")
    rect(x, y, w, h, "MAGENTA")
    print_scr_mini(x+4, y+4, "PURPLE", label)
		local mouse = mouse()
		if mouse.pressed then
			if mouse.x >= x and mouse.x <= x + y and mouse.y >= y and mouse.y <= y+h then
				log("Button pressed")
			end
		end
end

math.randomseed(os.time())
function update(dt)
	player.upd()
	clear("BLACK")
	print_scr(1, 50, "GREEN", "WOWAZA")
	draw(1, 45, "correct.png")
	button(60, 50, 40, 10, "BUTTON!")
	
	if player.x % 10 == 0 then
		log("Frame rate: " .. tostring(math.floor(1000/dt)))

		local x = math.random(0, 127)
		local y = math.random(0, 127)
		local cols = {"GREEN", "CYAN", "BLUE", "RED", "TEAL"}
		local col = cols[math.random(1, #cols)]
		if key_pressed("Enter") then
			log("Pressed enter")
		end
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
