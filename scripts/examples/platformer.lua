-- main.lua

-- Game State
game_state = "start" -- "start", "playing", "gameover", "levelcomplete"
score = 0
current_level = 1

-- Player
player = {x=10, y=100, w=8, h=8, dx=0, dy=0, speed=1.5, jump_force=-3.5, on_ground=false, health=3, color="CYAN"}

-- Physics
gravity = 0.18
tile_size = 8

-- Camera
camera_x = 0

-- Levels (tilemaps)
levels = {}
level_width = 64
level_height = 16

function generate_level(lv)
    local lvl = {}
    for y=1,level_height do
        lvl[y] = {}
        for x=1,level_width do
            if y==level_height then
                lvl[y][x] = 1 -- ground
            elseif y>level_height-3 and math.random() < 0.15 then
                lvl[y][x] = 1 -- platform
            elseif math.random() < 0.05 then
                lvl[y][x] = 2 -- collectible
            elseif math.random() < 0.03 then
                lvl[y][x] = 3 -- enemy spawn
            else
                lvl[y][x] = 0
            end
        end
    end
    return lvl
end

-- Pre-generate levels
for i=1,3 do
    levels[i] = generate_level(i)
end

-- Moving platforms
moving_platforms = {}
enemies = {}

-- Start
function start()
    rico:set_frame_rate(60)
end

-- Collision
function collide_tile(px, py)
    local tx = math.floor(px / tile_size) + 1
    local ty = math.floor(py / tile_size) + 1
    if ty < 1 or ty > level_height or tx < 1 or tx > level_width then return 0 end
    return levels[current_level][ty][tx]
end

-- Reset player for new level or restart
function reset_player()
    player.x = 10; player.y = 100; player.dx = 0; player.dy = 0; player.on_ground=false
end

-- Update
function update(dt)
    rico:clear("BLACK")
    
    local level = levels[current_level]
    
    -- Handle start/game over/level complete screens
    if game_state == "start" then
        rico:print_scr_mid(32,60,"WHITE","PRESS ENTER TO START")
        if rico:key_pressed("Enter") then game_state="playing"; reset_player() end
        return
    elseif game_state == "gameover" then
        rico:print_scr_mid(32,50,"RED","GAME OVER")
        rico:print_scr_mid(32,70,"WHITE","SCORE: "..score)
        rico:print_scr_mid(32,90,"WHITE","PRESS ENTER TO RESTART")
        if rico:key_pressed("Enter") then game_state="playing"; score=0; current_level=1; reset_player() end
        return
    elseif game_state == "levelcomplete" then
        rico:print_scr_mid(32,50,"LIME","LEVEL COMPLETE!")
        rico:print_scr_mid(32,70,"WHITE","SCORE: "..score)
        rico:print_scr_mid(32,90,"WHITE","PRESS ENTER TO CONTINUE")
        if rico:key_pressed("Enter") then
            current_level = current_level + 1
            if current_level > #levels then current_level = 1 end
            game_state="playing"; reset_player()
        end
        return
    end
    
    -- Player movement
    player.dx = 0
    if rico:key_pressed("Left") then player.dx = -player.speed end
    if rico:key_pressed("Right") then player.dx = player.speed end
    if rico:key_pressed("Up") and player.on_ground then
        player.dy = player.jump_force
        player.on_ground=false
    end
    
    -- Gravity
    player.dy = player.dy + gravity
    
    -- Horizontal collision
    local next_x = player.x + player.dx
    if not (collide_tile(next_x, player.y) == 1 or collide_tile(next_x+player.w-1, player.y) == 1 or
            collide_tile(next_x, player.y+player.h-1) == 1 or collide_tile(next_x+player.w-1, player.y+player.h-1) == 1) then
        player.x = next_x
    end
    
    -- Vertical collision
    local next_y = player.y + player.dy
    if player.dy > 0 then
        if (collide_tile(player.x, next_y+player.h-1) == 1 or collide_tile(player.x+player.w-1, next_y+player.h-1) == 1) then
            player.y = math.floor((next_y+player.h-1)/tile_size)*tile_size - player.h
            player.dy = 0
            player.on_ground=true
        else
            player.y = next_y
            player.on_ground=false
        end
    elseif player.dy < 0 then
        if (collide_tile(player.x, next_y) == 1 or collide_tile(player.x+player.w-1, next_y) == 1) then
            player.y = math.floor(next_y/tile_size +1)*tile_size
            player.dy=0
        else
            player.y = next_y
        end
    end
    
    -- Collectibles
    local tx = math.floor((player.x + player.w/2)/tile_size)+1
    local ty = math.floor((player.y + player.h/2)/tile_size)+1
    if level[ty][tx]==2 then
        score = score + 1
        level[ty][tx]=0
    end
    
    -- Enemies: spawn if tile=3
    for y=1,level_height do
        for x=1,level_width do
            if level[y][x]==3 then
                table.insert(enemies,{x=(x-1)*tile_size, y=(y-1)*tile_size, w=8,h=8, dx=1, color="RED"})
                level[y][x]=0
            end
        end
    end
    
    -- Update enemies
    for _,e in ipairs(enemies) do
        e.x = e.x + e.dx
        -- reverse at edges
        if collide_tile(e.x, e.y)==1 or collide_tile(e.x+e.w-1, e.y)==1 then e.dx = -e.dx end
        -- collision with player
        if player.x < e.x + e.w and player.x + player.w > e.x and player.y < e.y + e.h and player.y + player.h > e.y then
            player.health = player.health - 1
            player.x = 10; player.y = 100
            if player.health <= 0 then game_state="gameover" end
        end
    end
    
    -- Moving camera
    camera_x = math.max(0, math.min(player.x - 64, level_width*tile_size - 128))
    
    -- Draw level
    for y=1,level_height do
        for x=1,level_width do
            local tile = level[y][x]
            if tile==1 then rico:rectfill((x-1)*tile_size - camera_x,(y-1)*tile_size,tile_size,tile_size,"GRAY")
            elseif tile==2 then rico:rectfill((x-1)*tile_size - camera_x,(y-1)*tile_size,tile_size,tile_size,"YELLOW") end
        end
    end
    
    -- Draw player
    rico:rectfill(player.x - camera_x, player.y, player.w, player.h, player.color)
    
    -- Draw enemies
    for _,e in ipairs(enemies) do rico:rectfill(e.x - camera_x, e.y, e.w, e.h, e.color) end
    
    -- UI
    rico:print_scr(2,2,"WHITE","Score: "..score)
    rico:print_scr(2,10,"WHITE","Health: "..player.health)
    
    -- Level complete if reach right edge
    if player.x >= level_width*tile_size - player.w then game_state="levelcomplete" end
end

