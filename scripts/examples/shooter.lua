-- RICO-32 Shooter Example

-- Player properties
player = {
    x = 64,
    y = 110,
    w = 8,
    h = 8,
    speed = 100,
    color = "CYAN",
    bullets = {}
}

-- Enemy properties
enemies = {}
enemy_timer = 0
enemy_spawn_rate = 1000  -- milliseconds
enemy_speed = 40

-- Game properties
score = 0
frame_rate = 60

-- Utility function to check collision
function collides(a, b)
    return a.x < b.x + b.w and a.x + a.w > b.x and
           a.y < b.y + b.h and a.y + a.h > b.y
end

-- Shoot bullet
function shoot()
    table.insert(player.bullets, {x = player.x + player.w/2 - 1, y = player.y, w = 2, h = 4, speed = 150, color = "YELLOW"})
end

-- Start function
function start()
    rico:set_frame_rate(frame_rate)
end

-- Update function
function update(dt)
    -- Clear screen
    rico:clear("BLACK")

    -- Player movement
    if rico:key_pressed("Left") then
        player.x = math.max(0, player.x - player.speed * dt / 1000)
    end
    if rico:key_pressed("Right") then
        player.x = math.min(128 - player.w, player.x + player.speed * dt / 1000)
    end
    if rico:key_pressed("Up") then
        player.y = math.max(0, player.y - player.speed * dt / 1000)
    end
    if rico:key_pressed("Down") then
        player.y = math.min(128 - player.h, player.y + player.speed * dt / 1000)
    end

    -- Shooting bullets
    if rico:key_pressed("Space") then
        if not player.shoot_cooldown or player.shoot_cooldown <= 0 then
            shoot()
            player.shoot_cooldown = 300 -- ms cooldown
        end
    end
    if player.shoot_cooldown then
        player.shoot_cooldown = player.shoot_cooldown - dt
    end

    -- Update bullets
    for i=#player.bullets,1,-1 do
        local b = player.bullets[i]
        b.y = b.y - b.speed * dt / 1000
        if b.y < -b.h then
            table.remove(player.bullets, i)
        else
            rico:rectfill(b.x, b.y, b.w, b.h, b.color)
        end
    end

    -- Spawn enemies
    enemy_timer = enemy_timer + dt
    if enemy_timer >= enemy_spawn_rate then
        enemy_timer = 0
        table.insert(enemies, {x = math.random(0, 120), y = -8, w = 8, h = 8, color = "RED"})
    end

    -- Update enemies
    for i=#enemies,1,-1 do
        local e = enemies[i]
        e.y = e.y + enemy_speed * dt / 1000
        if e.y > 128 then
            table.remove(enemies, i)
        else
            rico:rectfill(e.x, e.y, e.w, e.h, e.color)
        end
    end

    -- Bullet-Enemy collisions
    for i=#player.bullets,1,-1 do
        local b = player.bullets[i]
        for j=#enemies,1,-1 do
            local e = enemies[j]
            if collides(b, e) then
                table.remove(player.bullets, i)
                table.remove(enemies, j)
                score = score + 1
                break
            end
        end
    end

    -- Enemy-Player collisions
    for i=#enemies,1,-1 do
        local e = enemies[i]
        if collides(e, player) then
            -- Game over
            rico:clear("BLACK")
            rico:print_scr_mid(64, 64, "RED", "GAME OVER")
            rico:print_scr_mid(64, 74, "WHITE", "Score: "..score)
            return
        end
    end

    -- Draw player
    rico:rectfill(player.x, player.y, player.w, player.h, player.color)

    -- Draw score
    rico:print_scr(2, 2, "WHITE", "Score: "..score)
end

