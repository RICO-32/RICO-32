-- main.lua

-- Grid settings
grid_width = 10
grid_height = 16
block_size = 8
margin_x = 16

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
colors = {"CYAN","YELLOW","MAGENTA","LIME","RED","BLUE","ORANGE"}

-- Game state
game_state = "start"
score = 0
current_piece = nil
piece_x, piece_y = 4,1
drop_timer = 0
drop_speed = 500 -- ms per drop
soft_drop_timer = 0
soft_drop_speed = 50
move_timer = 0
move_speed = 120

-- Start
function start()
    set_frame_rate(60)
end

-- Spawn a new piece
function spawn_piece()
    current_piece = tetrominoes[math.random(1,#tetrominoes)]
    piece_x = 4
    piece_y = 1
    drop_timer = 0
    soft_drop_timer = 0
    move_timer = 0
    if check_collision(piece_x, piece_y, current_piece) then
        game_state = "gameover"
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
    for y=grid_height,1,-1 do
        local full = true
        for x=1,grid_width do
            if grid[y][x] == 0 then
                full = false
                break
            end
        end
        if full then
            score = score + 100
            for yy=y,2,-1 do
                for x=1,grid_width do
                    grid[yy][x] = grid[yy-1][x]
                end
            end
            for x=1,grid_width do grid[1][x] = 0 end
        end
    end
end

-- Restart game
function restart_game()
    score = 0
    grid = {}
    for y=1,grid_height do
        grid[y] = {}
        for x=1,grid_width do grid[y][x] = 0 end
    end
    spawn_piece()
end

-- Update
function update(dt)
    -- Background
    clear("DARKGRAY")
    rectfill(0,0,margin_x,grid_height*block_size,"GRAY")
    rectfill(margin_x + grid_width*block_size,0,margin_x + grid_width*block_size+2,grid_height*block_size,"GRAY")
    rectfill(margin_x,0,grid_width*block_size,grid_height*block_size,"BLACK")

    -- Start menu
    if game_state == "start" then
        print_scr_mid(32,60,"WHITE","PRESS ENTER TO START")
        if key_just_pressed("Enter") then
            game_state = "playing"
            restart_game()
        end
        return
    end

    -- Game over
    if game_state == "gameover" then
        print_scr_mid(32,50,"RED","GAME OVER")
        print_scr_mid(32,70,"WHITE","SCORE: "..score)
        print_scr_mid(32,90,"WHITE","PRESS ENTER TO RESTART")
        if key_just_pressed("Enter") then
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
        if key_pressed("Left") and not check_collision(piece_x-1, piece_y, current_piece) then
            piece_x = piece_x - 1
            move_timer = 0
        elseif key_pressed("Right") and not check_collision(piece_x+1, piece_y, current_piece) then
            piece_x = piece_x + 1
            move_timer = 0
        end
    end

    -- Rotation (JUST PRESSED)
    if key_just_pressed("Up") then
        local rotated = rotate(current_piece)
        if not check_collision(piece_x, piece_y, rotated) then
            current_piece = rotated
        elseif not check_collision(piece_x-1, piece_y, rotated) then
            piece_x = piece_x -1; current_piece = rotated
        elseif not check_collision(piece_x+1, piece_y, rotated) then
            piece_x = piece_x +1; current_piece = rotated
        end
    end

    -- Soft drop (held)
    if key_pressed("Down") then
        soft_drop_timer = soft_drop_timer + dt
        if soft_drop_timer >= soft_drop_speed then
            soft_drop_timer = 0
            if not check_collision(piece_x, piece_y+1, current_piece) then
                piece_y = piece_y + 1
            else
                lock_piece()
            end
        end
    else
        soft_drop_timer = 0
        drop_timer = drop_timer + dt
        if drop_timer >= drop_speed then
            drop_timer = 0
            if not check_collision(piece_x, piece_y+1, current_piece) then
                piece_y = piece_y + 1
            else
                lock_piece()
            end
        end
    end

    --------------------------
    -- DRAW GRID + PIECE
    --------------------------

    -- Draw locked blocks
    for y=1,grid_height do
        for x=1,grid_width do
            local val = grid[y][x]
            if val ~= 0 then
                rectfill(
                    margin_x + (x-1)*block_size,
                    (y-1)*block_size,
                    block_size,
                    block_size,
                    colors[val]
                )
            end
        end
    end

    -- Draw current falling piece
    for y=1,#current_piece do
        for x=1,#current_piece[y] do
            local val = current_piece[y][x]
            if val ~= 0 then
                rectfill(
                    margin_x + (piece_x + x - 2)*block_size,
                    (piece_y + y - 2)*block_size,
                    block_size,
                    block_size,
                    colors[val]
                )
            end
        end
    end

    -- UI
    print_scr(2,2,"WHITE","Score: "..score)
end

