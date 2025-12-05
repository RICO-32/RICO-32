-- Platformer Game for RICO-32

-- Player settings
player = {
    x = 20,
    y = 80,
    w = 8,
    h = 8,
    vx = 0,
    vy = 0,
    on_ground = false,
    facing_right = true,
    jump_power = -0.8,
    move_speed = 0.025,
    anim_frame = 0,
    anim_timer = 0
}

-- Physics
gravity = 0.0015
max_fall_speed = 1.0
friction = 0.88
air_resistance = 0.94

-- Camera
camera = {
    x = 0,
    y = 0,
    target_x = 0,
    target_y = 0
}

-- Particles
particles = {}

-- Game state
game_state = "playing"
coins = 0
total_coins = 0
timer = 0
level = 1

-- Level data (platforms, coins, hazards, enemies)
platforms = {}
coins_list = {}
hazards = {}
enemies = {}
goal = {}

-- Level 1 layout
function load_level_1()
    platforms = {
        -- Starting area (low)
        {x=0, y=118, w=40, h=10},
        {x=50, y=112, w=28, h=16},
        {x=88, y=108, w=24, h=20},
        
        -- Climbing up
        {x=122, y=102, w=26, h=26},
        {x=158, y=96, w=24, h=32},
        {x=192, y=90, w=28, h=38},
        
        -- Mid-high section
        {x=230, y=84, w=24, h=44},
        {x=264, y=78, w=26, h=50},
        {x=300, y=72, w=24, h=56},
        
        -- Peak section (highest)
        {x=334, y=50, w=28, h=78},
        {x=372, y=45, w=26, h=83},
        {x=408, y=40, w=24, h=88},
        {x=442, y=45, w=28, h=83},
        {x=480, y=50, w=26, h=78},
        
        -- Descending
        {x=516, y=65, w=24, h=63},
        {x=550, y=75, w=28, h=53},
        {x=588, y=85, w=24, h=43},
        
        -- Low section
        {x=622, y=95, w=26, h=33},
        {x=658, y=105, w=24, h=23},
        {x=692, y=110, w=28, h=18},
        
        -- Final climb
        {x=730, y=100, w=26, h=28},
        {x=766, y=90, w=24, h=38},
        {x=800, y=80, w=35, h=48}
    }
    
    coins_list = {
        -- Lower section
        {x=60, y=102, collected=false},
        {x=98, y=98, collected=false},
        {x=132, y=92, collected=false},
        
        -- Climbing section
        {x=168, y=86, collected=false},
        {x=202, y=80, collected=false},
        {x=240, y=74, collected=false},
        {x=274, y=68, collected=false},
        {x=310, y=62, collected=false},
        
        -- Peak coins (high risk, high reward)
        {x=344, y=35, collected=false},
        {x=382, y=30, collected=false},
        {x=418, y=25, collected=false},
        {x=452, y=30, collected=false},
        {x=490, y=35, collected=false},
        
        -- Descending
        {x=526, y=55, collected=false},
        {x=560, y=65, collected=false},
        {x=598, y=75, collected=false},
        
        -- Lower section
        {x=632, y=85, collected=false},
        {x=668, y=95, collected=false},
        {x=702, y=100, collected=false},
        
        -- Final section
        {x=740, y=90, collected=false},
        {x=776, y=80, collected=false},
        {x=820, y=70, collected=false}
    }
    
    hazards = {
        {x=78, y=116, w=10, h=2},
        {x=112, y=118, w=8, h=2},
        {x=148, y=114, w=10, h=2},
        {x=182, y=108, w=8, h=2},
        {x=220, y=102, w=10, h=2},
        {x=254, y=96, w=8, h=2},
        {x=290, y=90, w=10, h=2},
        {x=506, y=83, w=10, h=2},
        {x=540, y=93, w=8, h=2},
        {x=578, y=103, w=10, h=2},
        {x=612, y=113, w=8, h=2},
        {x=648, y=123, w=10, h=2},
        {x=682, y=125, w=8, h=2},
        {x=720, y=118, w=10, h=2},
        {x=756, y=108, w=8, h=2},
        {x=790, y=98, w=10, h=2}
    }
    
    enemies = {
        -- Walking enemies on platforms (reduced from 13 to 8)
        {x=95, y=100, w=6, h=6, vx=0.02, start_x=88, end_x=110, type="walker"},
        {x=200, y=82, w=6, h=6, vx=0.025, start_x=192, end_x=218, type="walker"},
        {x=270, y=70, w=6, h=6, vx=0.02, start_x=264, end_x=288, type="walker"},
        {x=380, y=37, w=6, h=6, vx=0.03, start_x=372, end_x=396, type="walker"},
        {x=488, y=42, w=6, h=6, vx=0.025, start_x=480, end_x=504, type="walker"},
        {x=594, y=77, w=6, h=6, vx=0.02, start_x=588, end_x=610, type="walker"},
        {x=698, y=102, w=6, h=6, vx=0.025, start_x=692, end_x=718, type="walker"},
        {x=772, y=82, w=6, h=6, vx=0.02, start_x=766, end_x=788, type="walker"},
        
        -- Flying enemies (reduced from 7 to 5)
        {x=165, y=70, w=6, h=6, vx=0, vy=0.015, start_y=60, end_y=90, type="flyer"},
        {x=305, y=50, w=6, h=6, vx=0, vy=0.02, start_y=40, end_y=65, type="flyer"},
        {x=415, y=25, w=6, h=6, vx=0, vy=0.015, start_y=20, end_y=35, type="flyer"},
        {x=555, y=55, w=6, h=6, vx=0, vy=0.02, start_y=45, end_y=70, type="flyer"},
        {x=665, y=80, w=6, h=6, vx=0, vy=0.015, start_y=70, end_y=100, type="flyer"}
    }
    
    goal = {x=820, y=60, w=10, h=20}
    
    total_coins = #coins_list
end

-- Particle system
function create_particle(x, y, color, vx, vy)
    table.insert(particles, {
        x = x,
        y = y,
        vx = vx or (math.random() - 0.5) * 2,
        vy = vy or -math.random() * 2 - 1,
        life = 60,
        color = color
    })
end

function update_particles(dt)
    for i=#particles,1,-1 do
        local p = particles[i]
        p.x = p.x + p.vx * dt * 0.01
        p.y = p.y + p.vy * dt * 0.01
        p.vy = p.vy + 0.1 * dt * 0.01
        p.life = p.life - dt * 0.01
        if p.life <= 0 then
            table.remove(particles, i)
        end
    end
end

function draw_particles()
    for _,p in ipairs(particles) do
        local alpha = p.life / 60
        if alpha > 0.3 then
            rico:set_pix(math.floor(p.x - camera.x), math.floor(p.y - camera.y), p.color)
        end
    end
end

-- Collision detection
function aabb_collision(x1, y1, w1, h1, x2, y2, w2, h2)
    return x1 < x2 + w2 and
           x1 + w1 > x2 and
           y1 < y2 + h2 and
           y1 + h1 > y2
end

-- Check platform collision
function check_platform_collision(x, y, w, h)
    for _, plat in ipairs(platforms) do
        if aabb_collision(x, y, w, h, plat.x, plat.y, plat.w, plat.h) then
            return plat
        end
    end
    return nil
end

-- Reset player
function reset_player()
    player.x = 20
    player.y = 80
    player.vx = 0
    player.vy = 0
    timer = 0
    
    -- Reset coins
    for _, coin in ipairs(coins_list) do
        coin.collected = false
    end
    coins = 0
end

-- Start
function start()
    rico:set_frame_rate(60)
    load_level_1()
end

-- Update
function update(dt)
    -- ALWAYS clear the entire screen first
    rico:clear("BLACK")
    
    if game_state == "win" then
        -- Win screen
        rico:print_scr(30, 50, "YELLOW", "YOU WIN!")
        rico:print_scr_mid(20, 65, "WHITE", "Coins: "..coins.."/"..total_coins)
        rico:print_scr_mid(20, 75, "WHITE", "Time: "..math.floor(timer/1000).."s")
        rico:print_scr_mid(15, 95, "LIME", "Press ENTER to restart")
        
        if rico:key_just_pressed("Enter") then
            game_state = "playing"
            reset_player()
        end
        return
    end
    
    timer = timer + dt
    
    -- Player input
    local move_input = 0
    if rico:key_pressed("Left") then
        move_input = -1
        player.facing_right = false
    elseif rico:key_pressed("Right") then
        move_input = 1
        player.facing_right = true
    end
    
    -- Jump
    if rico:key_just_pressed("Up") and player.on_ground then
        player.vy = player.jump_power
        player.on_ground = false
        create_particle(player.x + player.w/2, player.y + player.h, "SILVER", -0.5, 0)
        create_particle(player.x + player.w/2, player.y + player.h, "SILVER", 0.5, 0)
    end
    
    -- Apply movement
    if player.on_ground then
        player.vx = player.vx * friction + move_input * player.move_speed * dt
    else
        player.vx = player.vx * air_resistance + move_input * player.move_speed * 0.3 * dt
    end
    
    -- Apply gravity
    player.vy = player.vy + gravity * dt
    if player.vy > max_fall_speed then
        player.vy = max_fall_speed
    end
    
    -- Update position
    player.x = player.x + player.vx
    player.y = player.y + player.vy
    
    -- Collision detection
    player.on_ground = false
    
    -- Platform collision
    local collided_plat = check_platform_collision(player.x, player.y, player.w, player.h)
    if collided_plat then
        -- Determine collision side
        local prev_y = player.y - player.vy
        
        if player.vy > 0 and prev_y + player.h <= collided_plat.y then
            -- Landing on top
            player.y = collided_plat.y - player.h
            player.vy = 0
            player.on_ground = true
        elseif player.vy < 0 and prev_y >= collided_plat.y + collided_plat.h then
            -- Hit from below
            player.y = collided_plat.y + collided_plat.h
            player.vy = 0
        else
            -- Side collision
            if player.vx > 0 then
                player.x = collided_plat.x - player.w
            else
                player.x = collided_plat.x + collided_plat.w
            end
            player.vx = 0
        end
    end
    
    -- Check if falling off screen
    if player.y > 130 then
        reset_player()
        create_particle(player.x + player.w/2, player.y, "RED", 0, -2)
    end
    
    -- Coin collection
    for _, coin in ipairs(coins_list) do
        if not coin.collected then
            if aabb_collision(player.x, player.y, player.w, player.h, coin.x-2, coin.y-2, 4, 4) then
                coin.collected = true
                coins = coins + 1
                for i=1,5 do
                    create_particle(coin.x, coin.y, "YELLOW", nil, nil)
                end
            end
        end
    end
    
    -- Hazard collision
    for _, hazard in ipairs(hazards) do
        if aabb_collision(player.x, player.y, player.w, player.h, hazard.x, hazard.y, hazard.w, hazard.h) then
            reset_player()
            for i=1,10 do
                create_particle(player.x + player.w/2, player.y + player.h/2, "RED", nil, nil)
            end
        end
    end
    
    -- Enemy collision and update
    for _, enemy in ipairs(enemies) do
        if enemy.type == "walker" then
            enemy.x = enemy.x + enemy.vx * dt
            if enemy.x <= enemy.start_x or enemy.x >= enemy.end_x then
                enemy.vx = -enemy.vx
            end
        elseif enemy.type == "flyer" then
            enemy.y = enemy.y + enemy.vy * dt
            if enemy.y <= enemy.start_y or enemy.y >= enemy.end_y then
                enemy.vy = -enemy.vy
            end
        end
        
        -- Check collision with player
        if aabb_collision(player.x, player.y, player.w, player.h, enemy.x, enemy.y, enemy.w, enemy.h) then
            reset_player()
            for i=1,10 do
                create_particle(player.x + player.w/2, player.y + player.h/2, "RED", nil, nil)
            end
        end
    end
    
    -- Goal collision
    if aabb_collision(player.x, player.y, player.w, player.h, goal.x, goal.y, goal.w, goal.h) then
        game_state = "win"
        for i=1,20 do
            create_particle(goal.x + goal.w/2, goal.y + goal.h/2, "LIME", nil, nil)
        end
    end
    
    -- Update camera
    camera.target_x = player.x - 50
    camera.target_y = player.y - 50
    
    if camera.target_x < 0 then camera.target_x = 0 end
    if camera.target_y < 0 then camera.target_y = 0 end
    
    camera.x = camera.x + (camera.target_x - camera.x) * 0.005 * dt
    camera.y = camera.y + (camera.target_y - camera.y) * 0.005 * dt
    
    -- Animation
    if math.abs(player.vx) > 0.1 and player.on_ground then
        player.anim_timer = player.anim_timer + dt
        if player.anim_timer > 150 then
            player.anim_frame = (player.anim_frame + 1) % 2
            player.anim_timer = 0
        end
    else
        player.anim_frame = 0
    end
    
    -- Update particles
    update_particles(dt)
    
    --------------------------
    -- DRAWING
    --------------------------
    
    -- Sky gradient
    for y=0,127 do
        local intensity = math.floor(y / 128 * 4)
        local colors = {"CYAN", "TEAL", "NAVY", "BLACK"}
        rico:rectfill(0, y, 128, 1, colors[intensity + 1])
    end
    
    -- Clouds
    local cloud_offset = (timer / 50) % 200
    for i=0,3 do
        local cx = (i * 50 - cloud_offset) % 200 - 20
        rico:circle(math.floor(cx), 15, 4, "WHITE")
        rico:circle(math.floor(cx + 5), 14, 5, "WHITE")
        rico:circle(math.floor(cx + 10), 15, 4, "WHITE")
    end
    
    -- Platforms
    for _, plat in ipairs(platforms) do
        local px = math.floor(plat.x - camera.x)
        local py = math.floor(plat.y - camera.y)
        if px > -plat.w and px < 128 and py > -plat.h and py < 128 then
            -- Grass top
            rico:rectfill(px, py, plat.w, 2, "LIME")
            -- Dirt body
            rico:rectfill(px, py + 2, plat.w, plat.h - 2, "MAROON")
            -- Details
            for x=0,plat.w,4 do
                rico:set_pix(px + x, py + 3, "OLIVE")
            end
        end
    end
    
    -- Coins
    local coin_frame = math.floor((timer / 100) % 4)
    for _, coin in ipairs(coins_list) do
        if not coin.collected then
            local cx = math.floor(coin.x - camera.x)
            local cy = math.floor(coin.y - camera.y)
            if cx > -5 and cx < 133 and cy > -5 and cy < 133 then
                local coin_colors = {"YELLOW", "YELLOW", "OLIVE", "YELLOW"}
                local offset = coin_frame == 2 and 1 or 0
                rico:circle(cx, cy, 2 - offset, coin_colors[coin_frame + 1])
            end
        end
    end
    
    -- Hazards (spikes)
    for _, hazard in ipairs(hazards) do
        local hx = math.floor(hazard.x - camera.x)
        local hy = math.floor(hazard.y - camera.y)
        if hx > -hazard.w and hx < 128 and hy > -hazard.h and hy < 128 then
            for x=0,hazard.w,3 do
                rico:rectfill(hx + x, hy, 2, hazard.h, "RED")
                rico:set_pix(hx + x + 1, hy - 1, "RED")
            end
        end
    end
    
    -- Enemies
    for _, enemy in ipairs(enemies) do
        local ex = math.floor(enemy.x - camera.x)
        local ey = math.floor(enemy.y - camera.y)
        if ex > -enemy.w and ex < 128 and ey > -enemy.h and ey < 128 then
            if enemy.type == "walker" then
                -- Body
                rico:rectfill(ex, ey + 2, enemy.w, enemy.h - 2, "RED")
                -- Eyes (facing direction)
                if enemy.vx > 0 then
                    rico:set_pix(ex + 4, ey + 3, "WHITE")
                else
                    rico:set_pix(ex + 1, ey + 3, "WHITE")
                end
                -- Legs
                local leg_offset = math.floor((timer / 150) % 2)
                if leg_offset == 0 then
                    rico:set_pix(ex + 1, ey + enemy.h, "MAROON")
                    rico:set_pix(ex + 4, ey + enemy.h, "MAROON")
                else
                    rico:set_pix(ex + 2, ey + enemy.h, "MAROON")
                    rico:set_pix(ex + 3, ey + enemy.h, "MAROON")
                end
            elseif enemy.type == "flyer" then
                -- Body
                rico:rectfill(ex + 1, ey + 1, enemy.w - 2, enemy.h - 2, "PURPLE")
                -- Wings (animated)
                local wing_offset = math.floor(math.sin(timer / 100) * 2)
                rico:set_pix(ex, ey + 2 + wing_offset, "MAGENTA")
                rico:set_pix(ex + enemy.w - 1, ey + 2 + wing_offset, "MAGENTA")
                -- Eyes
                rico:set_pix(ex + 2, ey + 2, "WHITE")
                rico:set_pix(ex + 3, ey + 2, "WHITE")
            end
        end
    end
    
    -- Goal flag
    local gx = math.floor(goal.x - camera.x)
    local gy = math.floor(goal.y - camera.y)
    if gx > -goal.w and gx < 128 and gy > -goal.h and gy < 128 then
        rico:rectfill(gx, gy, 2, goal.h, "WHITE")
        local flag_wave = math.floor(math.sin(timer / 100) * 2)
        rico:rectfill(gx + 2, gy + flag_wave, 6, 8, "LIME")
    end
    
    -- Draw particles
    draw_particles()
    
    -- Player
    local px = math.floor(player.x - camera.x)
    local py = math.floor(player.y - camera.y)
    
    -- Body
    rico:rectfill(px, py + 2, player.w, player.h - 2, "BLUE")
    -- Head
    rico:rectfill(px + 1, py, player.w - 2, 3, "CYAN")
    
    -- Eyes
    if player.facing_right then
        rico:set_pix(px + 5, py + 1, "WHITE")
    else
        rico:set_pix(px + 2, py + 1, "WHITE")
    end
    
    -- Legs (animation)
    if player.anim_frame == 1 and player.on_ground then
        rico:rectfill(px + 1, py + player.h, 2, 2, "BLUE")
        rico:rectfill(px + 5, py + player.h, 2, 2, "BLUE")
    else
        rico:rectfill(px + 2, py + player.h, 2, 2, "BLUE")
        rico:rectfill(px + 4, py + player.h, 2, 2, "BLUE")
    end
    
    -- UI
    rico:rectfill(0, 0, 128, 10, "BLACK")
    rico:print_scr_mini(2, 2, "YELLOW", "COINS: "..coins.."/"..total_coins)
    rico:print_scr_mini(70, 2, "WHITE", "TIME: "..math.floor(timer/1000).."s")
end

