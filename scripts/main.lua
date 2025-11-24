-- Constants
local SCREEN_W, SCREEN_H = 128, 128
local BALL_RADIUS = 4
local FRICTION = 0.98
local POCKET_RADIUS = 6

-- Colors
local TABLE_COLOR = "GREEN"
local BALL_COLOR = "WHITE"
local TARGET_COLOR = "RED"
local HOLE_COLOR = "BLACK"

-- Score
local score = 0

-- Ball objects
local balls = {
    {x = 64, y = 100, vx = 0, vy = 0, color = BALL_COLOR}, -- cue ball
    {x = 40, y = 50, vx = 0, vy = 0, color = TARGET_COLOR},
    {x = 80, y = 50, vx = 0, vy = 0, color = "BLUE"},
    {x = 64, y = 30, vx = 0, vy = 0, color = "YELLOW"}
}

-- Pockets (corners + middle of long sides)
local pockets = {
    {x = 0, y = 0}, {x = SCREEN_W/2, y = 0}, {x = SCREEN_W, y = 0},
    {x = 0, y = SCREEN_H}, {x = SCREEN_W/2, y = SCREEN_H}, {x = SCREEN_W, y = SCREEN_H}
}

-- Draw functions
local function draw_ball(ball)
    circle(ball.x, ball.y, BALL_RADIUS, ball.color)
end

local function draw_table()
    clear(TABLE_COLOR)
    for _, p in ipairs(pockets) do
        circle(p.x, p.y, POCKET_RADIUS, HOLE_COLOR)
    end
end

-- Ball physics
local function check_collision(b1, b2)
    local dx = b2.x - b1.x
    local dy = b2.y - b1.y
    local dist = math.sqrt(dx*dx + dy*dy)
    return dist < BALL_RADIUS*2
end

local function update_balls(dt)
    for i = #balls, 1, -1 do
        local b = balls[i]

        -- Move ball
        b.x = b.x + b.vx * dt/16
        b.y = b.y + b.vy * dt/16

        -- Friction
        b.vx = b.vx * FRICTION
        b.vy = b.vy * FRICTION

        -- Bounce off walls
        if b.x < BALL_RADIUS then b.x = BALL_RADIUS b.vx = -b.vx end
        if b.x > SCREEN_W - BALL_RADIUS then b.x = SCREEN_W - BALL_RADIUS b.vx = -b.vx end
        if b.y < BALL_RADIUS then b.y = BALL_RADIUS b.vy = -b.vy end
        if b.y > SCREEN_H - BALL_RADIUS then b.y = SCREEN_H - BALL_RADIUS b.vy = -b.vy end

        -- Check pockets
        for _, p in ipairs(pockets) do
            local dx = b.x - p.x
            local dy = b.y - p.y
            local dist = math.sqrt(dx*dx + dy*dy)
            if dist < POCKET_RADIUS then
                if b.color ~= BALL_COLOR then
                    score = score + 1 -- score only for target balls
                end
                table.remove(balls, i)
                break
            end
        end
    end

    -- Ball collisions
    for i = 1, #balls do
        for j = i+1, #balls do
            local b1, b2 = balls[i], balls[j]
            if check_collision(b1, b2) then
                local dx = b2.x - b1.x
                local dy = b2.y - b1.y
                local dist = math.sqrt(dx*dx + dy*dy)
                local nx, ny = dx/dist, dy/dist
                local p = 2 * (b1.vx*nx + b1.vy*ny - b2.vx*nx - b2.vy*ny) / 2
                b1.vx = b1.vx - p*nx
                b1.vy = b1.vy - p*ny
                b2.vx = b2.vx + p*nx
                b2.vy = b2.vy + p*ny
            end
        end
    end
end

-- Launch cue ball with mouse
local function handle_input()
    local mouse_obj = mouse()
    if mouse_obj.just_pressed then
        local cue = balls[1] -- always first
        local dx = mouse_obj.x - cue.x
        local dy = mouse_obj.y - cue.y
        cue.vx = dx / 5
        cue.vy = dy / 5
    end
end

-- Draw score
local function draw_score()
    print_scr(2, 2, "WHITE", "Score: " .. score)
end

-- RICO-32 start
function start()
    log("Pool game started!")
    set_frame_rate(60)
end

-- RICO-32 update
function update(dt)
    draw_table()
    handle_input()
    update_balls(dt)
    for _, ball in ipairs(balls) do
        draw_ball(ball)
    end
    draw_score()
end

