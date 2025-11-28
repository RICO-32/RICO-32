use crate::{rico_engine::{PixelsType, ScreenEngine, SCREEN_SIZE}, utils::{colors::{ALL_COLORS, COLORS}, keyboard::Keyboard, mouse::MousePress, pixels::{clear, rect, rect_fill, set_pix}}};

#[derive(Copy, Clone, PartialEq)]
pub enum Tools{
    Pencil,
    Fill,
    Eraser,
    Select
}

const BUTTON_WIDTH: i32 = 12;
const SPRITE_SIZE: usize = 32;
const DRAW_Y: i32 = 44;

fn image_from_tool(tool: Tools) -> [[COLORS; BUTTON_WIDTH as usize - 2]; BUTTON_WIDTH as usize - 2] {
    let ye = COLORS::YELLOW;
    let br = COLORS::BROWN;
    let bl = COLORS::BLANK;
    let re = COLORS::RED;
    let db = COLORS::BLUE;
    let si = COLORS::SILVER;
    let gr = COLORS::GRAY;
    let pi = COLORS::PINK;
    match tool {
        Tools::Pencil => {
            [
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, bl, ye, br, br, bl],
                [bl, bl, bl, bl, bl, ye, ye, ye, br, bl],
                [bl, bl, bl, bl, ye, ye, ye, ye, ye, bl],
                [bl, bl, bl, ye, ye, ye, ye, ye, bl, bl],
                [bl, bl, ye, ye, ye, ye, ye, bl, bl, bl],
                [bl, bl, re, ye, ye, ye, bl, bl, bl, bl],
                [bl, re, re, re, ye, bl, bl, bl, bl, bl],
                [bl, re, re, bl, bl, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
            ]
        },
        Tools::Fill => {
            [
                [bl, bl, bl, bl, si, gr, gr, bl, bl, bl],
                [bl, bl, bl, si, si, gr, gr, bl, bl, bl],
                [bl, db, db, si, gr, gr, gr, si, bl, bl],
                [db, db, si, si, gr, gr, si, si, si, bl],
                [db, si, si, si, gr, gr, si, si, si, si],
                [db, si, si, gr, gr, gr, si, si, si, si],
                [db, si, si, si, si, si, si, si, si, si],
                [db, bl, si, si, si, si, si, si, si, si],
                [db, bl, bl, si, si, si, si, si, si, si],
                [bl, bl, bl, bl, bl, si, si, si, si, bl],
            ]
        },
        Tools::Eraser => {
            [
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, re, re, bl, bl, bl],
                [bl, bl, bl, bl, re, pi, re, re, bl, bl],
                [bl, bl, bl, re, re, re, pi, re, re, bl],
                [bl, bl, re, re, re, re, re, pi, re, bl],
                [bl, re, pi, re, re, re, re, re, bl, bl],
                [bl, re, re, pi, re, re, re, bl, bl, bl],
                [bl, bl, re, re, pi, re, bl, bl, bl, bl],
                [bl, bl, bl, re, re, bl, bl, bl, bl, bl],
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
            ]
        },
        Tools::Select => {
            [
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
                [bl, si, si, bl, si, si, bl, si, si, bl],
                [bl, si, bl, bl, bl, bl, bl, bl, si, bl],
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
                [bl, si, bl, bl, bl, bl, bl, bl, si, bl],
                [bl, si, bl, bl, bl, bl, bl, bl, si, bl],
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
                [bl, si, bl, bl, bl, bl, bl, bl, si, bl],
                [bl, si, si, bl, si, si, bl, si, si, bl],
                [bl, bl, bl, bl, bl, bl, bl, bl, bl, bl],
            ]
        },
    }
}

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub struct SpriteEngine{
    pixels: PixelsType,
    selected_color: COLORS,
    pub mouse: MousePress,
    pub sprite_pixs: PixelsType,
    pub tool: Tools,
    pub keyboard: Keyboard

    selection: Option<(i32, i32, i32, i32)>, 
    selection_start_pos: Option<(i32, i32)>,
    moving_selection_content: Option<PixelsType>,
    move_start_info: Option<((i32, i32), (i32, i32, i32, i32))>, 
}

impl SpriteEngine{
    pub fn new() -> Self{
        SpriteEngine { 
            pixels: vec![vec![COLORS::BLACK; SCREEN_SIZE]; SCREEN_SIZE * 2],
            mouse: MousePress::default(),
            selected_color: COLORS::BLACK,
            sprite_pixs: vec![vec![COLORS::BLANK; SPRITE_SIZE]; SPRITE_SIZE],
            tool: Tools::Pencil,
            selection: None,
            selection_start_pos: None,
            moving_selection_content: None,
            move_start_info: None,
            keyboard: Keyboard::default(),
        }
    }

    fn stamp_selection(&mut self) {
        if let (Some(content), Some((x1, y1, _, _))) = (self.moving_selection_content.take(), self.selection) {
            let h = content.len();
            if h == 0 { return; }
            let w = content[0].len();
            if w == 0 { return; }
    
            for r in 0..h {
                for c in 0..w {
                    let target_y = y1 as isize + r as isize;
                    let target_x = x1 as isize + c as isize;
                    if target_y >= 0 && target_y < SPRITE_SIZE as isize && target_x >= 0 && target_x < SPRITE_SIZE as isize {
                        if content[r][c] != COLORS::BLANK { 
                            self.sprite_pixs[target_y as usize][target_x as usize] = content[r][c];
                        }
                    }
                }
            }
        }
        self.moving_selection_content = None;
    }

    fn handle_click(&mut self, y: usize, x: usize){
        match self.tool{
            Tools::Pencil => {
                self.sprite_pixs[y][x] = self.selected_color;
            },
            Tools::Fill => {
                let col = self.sprite_pixs[y][x];
                let mut q: Vec<(i32, i32)> = vec![(y as i32, x as i32)];
                let mut visited: [[bool; SPRITE_SIZE]; SPRITE_SIZE] = [[false; SPRITE_SIZE]; SPRITE_SIZE];
                while q.len() > 0 {
                    let t = q.pop().unwrap();

                    if visited[t.0 as usize][t.1 as usize] {
                        continue;
                    }
                    visited[t.0 as usize][t.1 as usize] = true;
                    self.sprite_pixs[t.0 as usize][t.1 as usize] = self.selected_color;

                    for dir in DIRS {
                        let ny = t.0 + dir.0;
                        let nx = t.1 + dir.1;
                        if ny >= 0 && ny < SPRITE_SIZE as i32 && nx >= 0 && nx < SPRITE_SIZE as i32 && col == self.sprite_pixs[ny as usize][nx as usize] {
                            q.push((ny, nx));
                        }
                    }
                }
            },
            Tools::Eraser => {
                self.sprite_pixs[y][x] = COLORS::BLANK;
            },
            Tools::Select => {},
        };
    }

    fn draw_canvas(&mut self) {
        for y in 0..SPRITE_SIZE as i32{
            for x in 0..SPRITE_SIZE as i32{
                let mut col = self.sprite_pixs[y as usize][x as usize];
                if col == COLORS::BLANK {
                    col = if (y + x) % 2 == 0 { COLORS::SILVER } else { COLORS::WHITE };
                }
                rect_fill(&mut self.pixels, 16+x*3, DRAW_Y+y*3, 3, 3, col);

            }
        }
        
        let on_canvas = self.mouse.x >= 16 && self.mouse.x < 16+(SPRITE_SIZE as i32*3) && self.mouse.y >= DRAW_Y && self.mouse.y < DRAW_Y+(SPRITE_SIZE as i32*3);
        let grid_x = ((self.mouse.x - 16) / 3).max(0).min(SPRITE_SIZE as i32 - 1);
        let grid_y = ((self.mouse.y - DRAW_Y) / 3).max(0).min(SPRITE_SIZE as i32 - 1);

        if self.tool == Tools::Select {
            if self.mouse.just_pressed && on_canvas {
                if let Some((x1, y1, x2, y2)) = self.selection {
                    if grid_x >= x1 && grid_x <= x2 && grid_y >= y1 && grid_y <= y2 {
                        self.move_start_info = Some(((grid_x, grid_y), self.selection.unwrap()));
        
                        if self.moving_selection_content.is_none() {
                            let w = (x2 - x1 + 1) as usize;
                            let h = (y2 - y1 + 1) as usize;
                            let mut content = vec![vec![COLORS::BLANK; w]; h];
                            for r in 0..h {
                                for c in 0..w {
                                    content[r][c] = self.sprite_pixs[y1 as usize + r][x1 as usize + c];
                                    self.sprite_pixs[y1 as usize + r][x1 as usize + c] = COLORS::BLANK;
                                }
                            }
                            self.moving_selection_content = Some(content);
                        }
                    } else {
                        self.stamp_selection();
                        self.selection = None;
                        self.selection_start_pos = Some((grid_x, grid_y));
                    }
                } else {
                    self.stamp_selection();
                    self.selection = None;
                    self.selection_start_pos = Some((grid_x, grid_y));
                }
            }
        
            if self.mouse.pressed {
                if let Some((start_grid, start_rect)) = self.move_start_info {
                    let dx = grid_x - start_grid.0;
                    let dy = grid_y - start_grid.1;
                    self.selection = Some((start_rect.0 + dx, start_rect.1 + dy, start_rect.2 + dx, start_rect.3 + dy));
                } else if let Some(start_pos) = self.selection_start_pos {
                    let x1 = start_pos.0.min(grid_x);
                    let y1 = start_pos.1.min(grid_y);
                    let x2 = start_pos.0.max(grid_x);
                    let y2 = start_pos.1.max(grid_y);
                    self.selection = Some((x1, y1, x2, y2));
                }
            }
            if !self.mouse.pressed {
                self.move_start_info = None;
                self.selection_start_pos = None;
            }
            if let (Some(content), Some((x1, y1, _, _))) = (self.moving_selection_content.as_ref(), self.selection) {
                let h = content.len();
                let w = content[0].len();
                for r in 0..h {
                    for c in 0..w {
                        let col = content[r][c];
                        if col != COLORS::BLANK {
                            let draw_x = 16 + (x1 + c as i32) * 3;
                            let draw_y = DRAW_Y + (y1 + r as i32) * 3;
                            rect_fill(&mut self.pixels, draw_x, draw_y, 3, 3, col);
                        }
                    }
                }
            }
            if let Some((x1, y1, x2, y2)) = self.selection {
                let x = 16 + x1 * 3;
                let y = DRAW_Y + y1 * 3;
                let w = (x2 - x1 + 1) * 3;
                let h = (y2 - y1 + 1) * 3;
                rect(&mut self.pixels, x, y, w, h, COLORS::WHITE);
                rect(&mut self.pixels, x-1, y-1, w+2, h+2, COLORS::BLACK);
            }

        } else {
            if self.mouse.pressed && on_canvas {
                self.handle_click(grid_y as usize, grid_x as usize);
            }
            self.stamp_selection();
            self.selection = None;
            self.selection_start_pos = None;
            self.move_start_info = None;
        }
    }

    fn tool_button(&mut self, x: i32, y: i32, tool: Tools){
        for dy in 1..BUTTON_WIDTH-1{
            for dx in 1..BUTTON_WIDTH-1{
                set_pix(&mut self.pixels, y + dy, x + dx, image_from_tool(tool)[dy as usize - 1][dx as usize - 1]);
            }
        }

        if self.mouse.just_pressed {
            if self.mouse.x != -1 {
                if self.mouse.x >= x && self.mouse.x < x + BUTTON_WIDTH && self.mouse.y >= y && self.mouse.y < y + BUTTON_WIDTH {
                    self.tool = tool;
                }
            }
        }

        if self.tool == tool {
            rect(&mut self.pixels, x, y, BUTTON_WIDTH-1, BUTTON_WIDTH-1, COLORS::WHITE);
        }
    }

    fn color_button(&mut self, x: i32, y: i32, col: COLORS){
        for dy in 1..BUTTON_WIDTH-1{
            for dx in 1..BUTTON_WIDTH-1{
                set_pix(&mut self.pixels, y + dy, x + dx, col);
            }
        }

        if self.mouse.just_pressed {
            if self.mouse.x != -1 {
                if self.mouse.x >= x && self.mouse.x < x + BUTTON_WIDTH && self.mouse.y >= y && self.mouse.y < y + BUTTON_WIDTH {
                    self.selected_color = col;
                }
            }
        }

        if self.selected_color == col {
            rect(&mut self.pixels, x, y, BUTTON_WIDTH-1, BUTTON_WIDTH-1, COLORS::WHITE);
        }
    }

    pub fn update(&mut self) {
        clear(&mut self.pixels, COLORS(30, 30, 30, 255));
        //clear(&mut self.pixels, COLORS::GRAY);

        // rect(&mut self.pixels, 14, 8, COLOR_BUTTON_WIDTH*8 + 3, COLOR_BUTTON_WIDTH * 2 + 3, COLORS::WHITE);
        for (i, col) in ALL_COLORS.iter().enumerate(){
            if *col == COLORS::BLANK { continue; }
            let idx = i as i32 - 1;
            self.color_button(16 + (idx as i32 % 8) * BUTTON_WIDTH, 10 + BUTTON_WIDTH * (idx >= 8) as i32, *col);
        }

        for (i, tool) in [Tools::Pencil, Tools::Eraser, Tools::Fill, Tools::Select].iter().enumerate(){
            let idx = i as i32;
            self.tool_button(40 + (idx as i32 % 8) * BUTTON_WIDTH, 148+ BUTTON_WIDTH * (idx >= 8) as i32, *tool);
        }

        self.draw_canvas();

        if self.mouse.just_pressed {
            self.mouse.just_pressed = false;
        };
    }
}
impl ScreenEngine for SpriteEngine{
    type Pixels<'a> = &'a PixelsType;
    fn pixels(&self) -> Self::Pixels<'_> {
        &self.pixels
    }
}
