-- RICO-32 Gorgeous Shooter Example
-- Drop this into scripts/main.lua

------------------------------------------------------------
-- CONFIG
------------------------------------------------------------
local W, H = 128, 128

local CFG = {
    -- player
    speed = 90,
    radius = 6,
    fire_rate = 120,        -- ms cooldown
    bullet_speed = 160,
    bullet_life = 800,
    player_hp = 5,

    -- enemies
    spawn_ms = 900,
    e_speed_min = 28,
    e_speed_max = 46,
    e_radius = 5,

    -- visuals
    stars_far = 42,
    stars_mid = 28,
    stars_near = 18,
    shake_decay = 0.92,
    shake_floor = 0.18,
    particle_life = 500,
    particle_speed = 90,
    max_particles = 180,

    -- colors
    c_bg = "BLACK",
    c_star_far = "GRAY",
    c_star_mid = "SILVER",
    c_star_near = "WHITE",
    c_player = "LIME",
    c_bullet = "MAGENTA",
    c_enemy1 = "RED",
    c_enemy2 = "YELLOW",
    c_enemy3 = "PURPLE",
    c_ui = "WHITE",
    c_warn = "YELLOW",
    c_hit = "RED",
}

------------------------------------------------------------
-- STATE
------------------------------------------------------------
local G = {
    time = 0,
    dt = 0,
    gameover = false,

    player = {
        x = 64, y = 104,
        fire_cd = 0,
        hp = CFG.player_hp,
        inv = 0,
    },

    bullets = {},
    enemies = {},
    particles = {},
    shake = 0,

    score = 0,
    spawn_t = 0,
}

local BG = { far = {}, mid = {}, near = {} }

------------------------------------------------------------
-- UTIL
------------------------------------------------------------
local function clamp(a, lo, hi) if a < lo then return lo elseif a > hi then return hi else return a end end
local function rnd(n) return math.random() * (n or 1) end
local function irnd(n) return math.floor(math.random() * n) end
local function angle_to(ax, ay, bx, by) return math.atan2((by - ay), (bx - ax)) end

local function within(ax, ay, bx, by, r)
    local dx, dy = ax - bx, ay - by
    return dx*dx + dy*dy <= r*r
end

local function screen_shake(p) G.shake = G.shake + (p or 1) end
local function apply_shake(x, y)
    if G.shake > 0.01 then
        return x + (math.random()*2-1)*G.shake, y + (math.random()*2-1)*G.shake
    end
    return x, y
end

local function draw_bar(x, y, w, h, pct, bg, fg)
    rico:rectfill(x, y, w, h, bg) -- background
    local ww = math.floor(w * clamp(pct, 0, 1))
    rico:rectfill(x, y, ww, h, fg) -- foreground
end


------------------------------------------------------------
-- BACKGROUND
------------------------------------------------------------
local function init_bg()
    BG.far, BG.mid, BG.near = {}, {}, {}
    for i=1, CFG.stars_far do BG.far[i] = {x=rnd(W), y=rnd(H), sp=10+rnd(6), c=CFG.c_star_far} end
    for i=1, CFG.stars_mid do BG.mid[i] = {x=rnd(W), y=rnd(H), sp=18+rnd(10), c=CFG.c_star_mid} end
    for i=1, CFG.stars_near do BG.near[i] = {x=rnd(W), y=rnd(H), sp=28+rnd(16), c=CFG.c_star_near} end
end

local function update_bg(dt)
    local function step(layer)
        for _,s in ipairs(layer) do
            s.y = s.y + s.sp*(dt/1000)
            if s.y > H then s.y = -2; s.x = rnd(W) end
        end
    end
    step(BG.far); step(BG.mid); step(BG.near)
end

local function draw_bg()
    rico:clear(CFG.c_bg)
    for _,s in ipairs(BG.far) do rico:rectfill(s.x, s.y, 1, 1, s.c) end
    for _,s in ipairs(BG.mid) do rico:rectfill(s.x, s.y, 1, 1, s.c) end
    for _,s in ipairs(BG.near) do rico:rectfill(s.x, s.y, 1, 1, s.c) end
end

------------------------------------------------------------
-- PARTICLES
------------------------------------------------------------
local function add_particles(x, y, n, col)
    col = col or CFG.c_warn
    for i=1,n do
        if #G.particles < CFG.max_particles then
            local a = rnd(2*math.pi)
            local sp = rnd(CFG.particle_speed)
            table.insert(G.particles, {
                x=x, y=y,
                vx=math.cos(a)*sp*0.02, vy=math.sin(a)*sp*0.02,
                life=CFG.particle_life*(0.6+rnd(0.4)),
                c=col
            })
        end
    end
end

local function update_particles(dt)
    for i=#G.particles,1,-1 do
        local p = G.particles[i]
        p.life = p.life - dt
        p.x = p.x + p.vx*dt
        p.y = p.y + p.vy*dt
        if p.life <= 0 then table.remove(G.particles,i) end
    end
end

local function draw_particles()
    for _,p in ipairs(G.particles) do rico:circle(p.x, p.y, 1, p.c) end
end

------------------------------------------------------------
-- PLAYER
------------------------------------------------------------
local function damage_player()
    if G.player.inv > 0 then return end
    G.player.hp = G.player.hp - 1
    G.player.inv = 400
    add_particles(G.player.x, G.player.y, 16, CFG.c_hit)
    screen_shake(1.2)
    if G.player.hp <= 0 then
        G.gameover = true
        add_particles(G.player.x, G.player.y, 40, CFG.c_hit)
        screen_shake(2.0)
    end
end

local function update_player(dt)
    local p = G.player
    p.inv = math.max(0, p.inv - dt)
    p.fire_cd = math.max(0, p.fire_cd - dt)

    local ax, ay = 0, 0
    if rico:key_pressed("A") or rico:key_pressed("Left") then ax = ax - 1 end
    if rico:key_pressed("D") or rico:key_pressed("Right") then ax = ax + 1 end
    if rico:key_pressed("W") or rico:key_pressed("Up") then ay = ay - 1 end
    if rico:key_pressed("S") or rico:key_pressed("Down") then ay = ay + 1 end
    if ax~=0 or ay~=0 then local m=math.sqrt(ax*ax+ay*ay); ax,ay=ax/m,ay/m end

    p.x = clamp(p.x + ax*CFG.speed*(dt/1000), 8, W-8)
    p.y = clamp(p.y + ay*CFG.speed*(dt/1000), 12, H-12)

    local mouse = rico:mouse()
    if (mouse.pressed or rico:key_pressed("Enter")) and p.fire_cd<=0 then
        p.fire_cd = CFG.fire_rate
        local tx, ty = mouse.x, mouse.y
        if tx<0 then tx,ty=p.x,p.y-40 end
        local ang = angle_to(p.x,p.y,tx,ty)
        local vx,vy = math.cos(ang)*CFG.bullet_speed, math.sin(ang)*CFG.bullet_speed
        table.insert(G.bullets,{x=p.x,y=p.y,vx=vx,vy=vy,life=CFG.bullet_life})
        add_particles(p.x,p.y,6,CFG.c_bullet)
    end
end

local function draw_player()
    local x,y = apply_shake(G.player.x,G.player.y)
    rico:rectfill(x-3,y-6,6,12,CFG.c_player) -- body
    rico:rectfill(x-7,y-2,4,4,CFG.c_player)  -- left wing
    rico:rectfill(x+3,y-2,4,4,CFG.c_player)  -- right wing
    rico:circle(x,y+6,3,"YELLOW")            -- thruster glow
    rico:circle(x,y+8,2,"RED")
    if G.player.inv>0 and (math.floor(G.time/120)%2==0) then
        rico:circle(x,y,CFG.radius+4,"CYAN")
    end
end
------------------------------------------------------------
-- BULLETS
------------------------------------------------------------
local function update_bullets(dt)
    for i=#G.bullets,1,-1 do
        local b = G.bullets[i]
        b.life = b.life - dt
        b.x = b.x + b.vx*(dt/1000)
        b.y = b.y + b.vy*(dt/1000)
        if b.life <= 0 or b.x < -4 or b.x > W+4 or b.y < -4 or b.y > H+4 then
            table.remove(G.bullets,i)
        end
    end
end

local function draw_bullets()
    for _, b in ipairs(G.bullets) do
        rico:circle(b.x, b.y, 2.5, CFG.c_bullet) -- outer glow
        rico:circle(b.x, b.y, 1.2, "WHITE")      -- inner core
        rico:circle(b.x - b.vx*0.02, b.y - b.vy*0.02, 1, "YELLOW") -- trail spark
    end
end

------------------------------------------------------------
-- ENEMIES
------------------------------------------------------------
local function spawn_enemy()
    local kind = (math.random() < 0.5) and 1 or 2
    local x = irnd(W-20)+10
    local y = -10
    local vy = CFG.e_speed_min + rnd(CFG.e_speed_max-CFG.e_speed_min)
    table.insert(G.enemies,{kind=kind,x=x,y=y,vx=0,vy=vy,hp=1})
end

local function update_enemies(dt)
    G.spawn_t = G.spawn_t - dt
    if G.spawn_t <= 0 then
        G.spawn_t = CFG.spawn_ms
        spawn_enemy()
    end

    for i=#G.enemies,1,-1 do
        local e = G.enemies[i]
        if e.kind==2 then
            e.x = e.x + math.sin(G.time/300)*20*(dt/1000)
        end
        e.y = e.y + e.vy*(dt/1000)

        -- offscreen cleanup
        if e.y > H+10 then table.remove(G.enemies,i) end
    end
end

local function draw_enemies()
    for _, e in ipairs(G.enemies) do
        local x,y = apply_shake(e.x,e.y)
        if e.kind==1 then
            rico:circle(x,y,CFG.e_radius+3,"GRAY")
            rico:circle(x,y,CFG.e_radius,CFG.c_enemy1)
            rico:circle(x,y-2,2,"WHITE")
        else
            rico:circle(x,y,CFG.e_radius+4,"GRAY")
            rico:rectfill(x-6,y-3,12,6,CFG.c_enemy2)
            rico:circle(x,y,3,"CYAN")
        end
    end
end

------------------------------------------------------------
-- COLLISIONS
------------------------------------------------------------
local function resolve_collisions()
    -- bullets vs enemies
    for bi=#G.bullets,1,-1 do
        local b=G.bullets[bi]
        local hit=nil
        for ei=#G.enemies,1,-1 do
            local e=G.enemies[ei]
            if within(b.x,b.y,e.x,e.y,CFG.e_radius+2) then hit=ei break end
        end
        if hit then
            add_particles(b.x,b.y,12,CFG.c_warn)
            screen_shake(0.6)
            table.remove(G.bullets,bi)
            table.remove(G.enemies,hit)
            G.score=G.score+10
        end
    end

    -- enemies vs player
    for ei=#G.enemies,1,-1 do
        local e=G.enemies[ei]
        if within(e.x,e.y,G.player.x,G.player.y,CFG.radius+CFG.e_radius) then
            damage_player()
            add_particles(e.x,e.y,12,CFG.c_hit)
            table.remove(G.enemies,ei)
        end
    end
end

------------------------------------------------------------
-- HUD
------------------------------------------------------------
local function draw_hud()
    -- score
    rico:print_scr_mid(4, 4, CFG.c_ui, "Score:" .. G.score)

    -- health bar
    draw_bar(4, H-8, 60, 5, G.player.hp / CFG.player_hp, "GRAY", CFG.c_hit)
    rico:print_scr_mid(68, H-9, CFG.c_ui, "HP:" .. G.player.hp)

    if G.gameover then
        rico:print_scr(W/2-44, H/2-10, "RED", "GAME OVER")
        rico:print_scr_mid(W/2-50, H/2+6, "YELLOW", "Enter: Restart")
    end
end

------------------------------------------------------------
-- FLOW
------------------------------------------------------------
local function update_shake()
    if G.shake>0 then
        G.shake=G.shake*CFG.shake_decay
        if G.shake<CFG.shake_floor then G.shake=0 end
    end
end

local function reset_game()
    G.time=0; G.dt=0; G.gameover=false
    G.player.x,G.player.y=64,104
    G.player.hp=CFG.player_hp
    G.player.fire_cd=0; G.player.inv=0
    G.bullets={}; G.enemies={}; G.particles={}
    G.score=0; G.spawn_t=CFG.spawn_ms; G.shake=0
end

------------------------------------------------------------
-- ENTRY POINTS
------------------------------------------------------------
function start()
    rico:set_frame_rate(60)
    init_bg()
    reset_game()
    rico:log("Gorgeous Shooter Ready")
end

function update(dt)
    G.dt=dt; G.time=G.time+dt
    if G.gameover and rico:key_just_pressed("Enter") then reset_game() end

    update_bg(dt)
    update_shake()
    update_particles(dt)

    if not G.gameover then
        update_player(dt)
        update_bullets(dt)
        update_enemies(dt)
        resolve_collisions()
    end

    draw_bg()
    draw_bullets()
    draw_enemies()
    draw_player()
    draw_particles()
    draw_hud()
end
