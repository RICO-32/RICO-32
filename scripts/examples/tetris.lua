-- main.lua

-- Grid settings
grid_width = 10
grid_height = 20
block_size = 6
margin_x = 20

-- Game grid
grid = {}
for y=1,grid_height do
    grid[y] = {}
    for x=1,grid_width do grid[y][x] = 0 end
end

-- Tetrominoes
tetrominoes = {
    {{1,1,1,1}},        -- I
    {{2,2},{2,2}},      -- O
    {{0,3,0},{3,3,3}},  -- T
    {{0,4,4},{4,4,0}},  -- S
    {{5,5,0},{0,5,5}},  -- Z
    {{6,0,0},{6,6,6}},  -- J
    {{0,0,7},{7,7,7}}   -- L
}
colors = {"TEAL","YELLOW","PINK","GREEN","RED","BLUE","ORANGE"}

-- Game state
game_state = "start"
score = 0
level = 1
lines_cleared = 0
current_piece = nil
next_piece = nil
piece_x, piece_y = 4,1
drop_timer = 0
drop_speed = 800
soft_drop_timer = 0
soft_drop_speed = 40
move_timer = 0
move_speed = 120
lock_delay = 0
lock_delay_max = 500
particles = {}
shake_timer = 0
combo = 0
high_score = 0
ghost_y = 1

-- Start
function start()
    rico:set_frame_rate(60)
end

-- Particle system
function create_particle(x, y, color_idx)
    for i=1,3 do
        table.insert(particles, {
            x = x,
            y = y,
            vx = (math.random() - 0.5) * 0.3,
            vy = -math.random() * 0.5 - 0.2,
            life = 500,
            color = colors[color_idx]
        })
    end
end

function update_particles(dt)
    for i=#particles,1,-1 do
        local p = particles[i]
        p.x = p.x + p.vx * dt
        p.y = p.y + p.vy * dt
        p.vy = p.vy + 0.001 * dt
        p.life = p.life - dt
        if p.life <= 0 then
            table.remove(particles, i)
        end
    end
end

function draw_particles()
    for _,p in ipairs(particles) do
        local alpha = p.life / 500
        if alpha > 0.5 then
            rico:rectfill(math.floor(p.x), math.floor(p.y), 2, 2, p.color)
        end
    end
end

-- Spawn a new piece
function spawn_piece()
    current_piece = next_piece or tetrominoes[math.random(1,#tetrominoes)]
    next_piece = tetrominoes[math.random(1,#tetrominoes)]
    piece_x = 4
    piece_y = 1
    drop_timer = 0
    soft_drop_timer = 0
    move_timer = 0
    lock_delay = 0
    calculate_ghost()
    if check_collision(piece_x, piece_y, current_piece) then
        game_state = "gameover"
        if score > high_score then high_score = score end
    end
end

-- Calculate ghost piece position
function calculate_ghost()
    ghost_y = piece_y
    while not check_collision(piece_x, ghost_y + 1, current_piece) do
        ghost_y = ghost_y + 1
    end
end

-- Rotate piece clockwise
function rotate(piece)
    local h = #piece
    local w = #piece[1]
    local new_piece = {}
    for y=1,w do
        new_piece[y] = {}
        for x=1,h do
            new_piece[y][x] = piece[h - x + 1][y]
        end
    end
    return new_piece
end

-- Collision check
function check_collision(px, py, piece)
    for y=1,#piece do
        for x=1,#piece[y] do
            if piece[y][x] ~= 0 then
                local gx = px + x - 1
                local gy = py + y - 1
                if gx < 1 or gx > grid_width or gy > grid_height then
                    return true
                end
                if gy >= 1 and grid[gy][gx] ~= 0 then
                    return true
                end
            end
        end
    end
    return false
end

-- Lock piece into grid
function lock_piece()
    for y=1,#current_piece do
        for x=1,#current_piece[y] do
            if current_piece[y][x] ~= 0 then
                local gx = piece_x + x - 1
                local gy = piece_y + y - 1
                if gy >= 1 and gy <= grid_height then
                    grid[gy][gx] = current_piece[y][x]
                end
            end
        end
    end
    clear_lines()
    spawn_piece()
end

-- Clear full lines
function clear_lines()
    local cleared = 0
    for y=grid_height,1,-1 do
        local full = true
        for x=1,grid_width do
            if grid[y][x] == 0 then
                full = false
                break
            end
        end
        if full then
            cleared = cleared + 1
            -- Create particles
            for x=1,grid_width do
                create_particle(margin_x + (x-1)*block_size + block_size/2, (y-1)*block_size + block_size/2, grid[y][x])
            end
            -- Shift down
            for yy=y,2,-1 do
                for x=1,grid_width do
                    grid[yy][x] = grid[yy-1][x]
                end
            end
            for x=1,grid_width do grid[1][x] = 0 end
            y = y + 1
        end
    end
    if cleared > 0 then
        lines_cleared = lines_cleared + cleared
        local points = {0, 100, 300, 500, 800}
        score = score + points[cleared+1] * level
        combo = combo + 1
        score = score + combo * 50
        shake_timer = 150
        -- Level up every 10 lines
        level = math.floor(lines_cleared / 10) + 1
        drop_speed = math.max(100, 800 - (level-1) * 50)
    else
        combo = 0
    end
end

-- Restart game
function restart_game()
    score = 0
    level = 1
    lines_cleared = 0
    combo = 0
    grid = {}
    for y=1,grid_height do
        grid[y] = {}
        for x=1,grid_width do grid[y][x] = 0 end
    end
    particles = {}
    next_piece = nil
    spawn_piece()
end

-- Hard drop
function hard_drop()
    while not check_collision(piece_x, piece_y+1, current_piece) do
        piece_y = piece_y + 1
        score = score + 2
    end
    lock_piece()
end

-- Update
function update(dt)
    -- Screen shake
    local shake_x, shake_y = 0, 0
    if shake_timer > 0 then
        shake_timer = shake_timer - dt
        shake_x = (math.random() - 0.5) * 2
        shake_y = (math.random() - 0.5) * 2
    end
    
    -- Background
    rico:clear("BLACK")
    
    -- Draw border with gradient effect
    for i=0,3 do
        local c = {"NAVY","TEAL","NAVY","BLACK"}
        rico:rect(margin_x-4+i, -4+i, grid_width*block_size+8-i*2, grid_height*block_size+8-i*2, c[i+1])
    end
    
    -- Draw playfield background
    for y=0,grid_height-1 do
        for x=0,grid_width-1 do
            if (x+y) % 2 == 0 then
                rico:rectfill(margin_x + x*block_size + shake_x, y*block_size + shake_y, block_size, block_size, "NAVY")
            else
                rico:rectfill(margin_x + x*block_size + shake_x, y*block_size + shake_y, block_size, block_size, "BLACK")
            end
        end
    end

    -- Start menu
    if game_state == "start" then
        rico:print_scr(28, 20, "CYAN", "TETRIS")
        rico:print_scr_mid(10, 50, "WHITE", "ARROW KEYS - MOVE")
        rico:print_scr_mid(10, 60, "WHITE", "UP - ROTATE")
        rico:print_scr_mid(10, 70, "WHITE", "DOWN - SOFT DROP")
        rico:print_scr_mid(10, 80, "WHITE", "SPACE - HARD DROP")
        rico:print_scr_mid(6, 100, "YELLOW", "PRESS ENTER TO START")
        if high_score > 0 then
            rico:print_scr_mid(10, 115, "LIME", "HIGH SCORE: "..high_score)
        end
        if rico:key_just_pressed("Enter") then
            game_state = "playing"
            restart_game()
        end
        return
    end

    -- Game over
    if game_state == "gameover" then
        rico:print_scr(20, 40, "RED", "GAME OVER")
        rico:print_scr_mid(20, 60, "WHITE", "SCORE: "..score)
        rico:print_scr_mid(20, 70, "WHITE", "LEVEL: "..level)
        rico:print_scr_mid(20, 80, "WHITE", "LINES: "..lines_cleared)
        if score == high_score and score > 0 then
            rico:print_scr_mid(10, 95, "YELLOW", "NEW HIGH SCORE!")
        end
        rico:print_scr_mid(4, 110, "LIME", "PRESS ENTER TO RESTART")
        if rico:key_just_pressed("Enter") then
            game_state = "playing"
            restart_game()
        end
        return
    end

    --------------------------
    -- PLAYER INPUT
    --------------------------

    -- Horizontal movement (held)
    move_timer = move_timer + dt
    if move_timer >= move_speed then
        if rico:key_pressed("Left") and not check_collision(piece_x-1, piece_y, current_piece) then
            piece_x = piece_x - 1
            move_timer = 0
            lock_delay = 0
            calculate_ghost()
        elseif rico:key_pressed("Right") and not check_collision(piece_x+1, piece_y, current_piece) then
            piece_x = piece_x + 1
            move_timer = 0
            lock_delay = 0
            calculate_ghost()
        end
    end

    -- Rotation (JUST PRESSED)
    if rico:key_just_pressed("Up") then
        local rotated = rotate(current_piece)
        if not check_collision(piece_x, piece_y, rotated) then
            current_piece = rotated
            lock_delay = 0
            calculate_ghost()
        elseif not check_collision(piece_x-1, piece_y, rotated) then
            piece_x = piece_x -1; current_piece = rotated
            lock_delay = 0
            calculate_ghost()
        elseif not check_collision(piece_x+1, piece_y, rotated) then
            piece_x = piece_x +1; current_piece = rotated
            lock_delay = 0
            calculate_ghost()
        end
    end
    
    -- Hard drop
    if rico:key_just_pressed("Space") then
        hard_drop()
        return
    end

    -- Soft drop (held)
    if rico:key_pressed("Down") then
        soft_drop_timer = soft_drop_timer + dt
        if soft_drop_timer >= soft_drop_speed then
            soft_drop_timer = 0
            if not check_collision(piece_x, piece_y+1, current_piece) then
                piece_y = piece_y + 1
                score = score + 1
                calculate_ghost()
            end
        end
    else
        soft_drop_timer = 0
        drop_timer = drop_timer + dt
        if drop_timer >= drop_speed then
            drop_timer = 0
            if not check_collision(piece_x, piece_y+1, current_piece) then
                piece_y = piece_y + 1
                calculate_ghost()
            end
        end
    end
    
    -- Manual lock when piece hits bottom
    if check_collision(piece_x, piece_y+1, current_piece) and rico:key_just_pressed("Down") then
        lock_piece()
    end

    --------------------------
    -- DRAW GRID + PIECE
    --------------------------

    -- Draw locked blocks with 3D effect
    for y=1,grid_height do
        for x=1,grid_width do
            local val = grid[y][x]
            if val ~= 0 then
                local bx = margin_x + (x-1)*block_size + shake_x
                local by = (y-1)*block_size + shake_y
                -- Shadow
                rico:rectfill(bx+1, by+1, block_size-1, block_size-1, "NAVY")
                -- Main block
                rico:rectfill(bx, by, block_size-1, block_size-1, colors[val])
                -- Highlight
                rico:rectfill(bx, by, block_size-2, 1, "WHITE")
                rico:rectfill(bx, by, 1, block_size-2, "WHITE")
            end
        end
    end

    -- Draw ghost piece
    if ghost_y ~= piece_y then
        for y=1,#current_piece do
            for x=1,#current_piece[y] do
                local val = current_piece[y][x]
                if val ~= 0 then
                    local bx = margin_x + (piece_x + x - 2)*block_size + shake_x
                    local by = (ghost_y + y - 2)*block_size + shake_y
                    rico:rect(bx, by, block_size-1, block_size-1, "GRAY")
                end
            end
        end
    end

    -- Draw current falling piece with 3D effect
    for y=1,#current_piece do
        for x=1,#current_piece[y] do
            local val = current_piece[y][x]
            if val ~= 0 then
                local bx = margin_x + (piece_x + x - 2)*block_size + shake_x
                local by = (piece_y + y - 2)*block_size + shake_y
                -- Shadow
                rico:rectfill(bx+1, by+1, block_size-1, block_size-1, "NAVY")
                -- Main block
                rico:rectfill(bx, by, block_size-1, block_size-1, colors[val])
                -- Highlight
                rico:rectfill(bx, by, block_size-2, 1, "WHITE")
                rico:rectfill(bx, by, 1, block_size-2, "WHITE")
            end
        end
    end

    -- Draw particles
    update_particles(dt)
    draw_particles()

    -- UI Panel
    local ui_x = margin_x + grid_width*block_size + 10
    rico:print_scr_mini(ui_x, 5, "SILVER", "SCORE")
    rico:print_scr_mid(ui_x, 12, "WHITE", tostring(score))
    
    rico:print_scr_mini(ui_x, 25, "SILVER", "LEVEL")
    rico:print_scr_mid(ui_x, 32, "YELLOW", tostring(level))
    
    rico:print_scr_mini(ui_x, 45, "SILVER", "LINES")
    rico:print_scr_mid(ui_x, 52, "CYAN", tostring(lines_cleared))
    
    if combo > 1 then
        rico:print_scr_mini(ui_x, 65, "LIME", "COMBO")
        rico:print_scr_mid(ui_x, 72, "LIME", "x"..combo)
    end
    
    -- Next piece preview
    rico:print_scr_mini(ui_x, 90, "SILVER", "NEXT")
    if next_piece then
        for y=1,#next_piece do
            for x=1,#next_piece[y] do
                local val = next_piece[y][x]
                if val ~= 0 then
                    rico:rectfill(ui_x + (x-1)*4, 96 + (y-1)*4, 3, 3, colors[val])
                end
            end
        end
    end
    
    -- High score
    if high_score > 0 then
        rico:print_scr_mini(2, 123, "OLIVE", "HI:"..high_score)
    end
end
