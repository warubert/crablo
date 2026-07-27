use macroquad::prelude::*;
use crate::entities::{DmgText, Monster};
use crate::map::{bfs, dist, draw_walls, to_screen, to_tile, Tile, MAP};
use crate::renderer::draw_stickman;

pub struct Game {
    map: [[Tile; MAP]; MAP],
    cam: (f32, f32),
    px: usize,
    py: usize,
    path: Vec<(usize, usize)>,
    player_cd: f32,
    monsters: Vec<Monster>,
    texts: Vec<DmgText>,
    pub hp: i32,
    gold: Vec<(usize, usize)>,
    pub score: i32,
}

impl Game {
    pub fn new() -> Self {
        let mut map = [[Tile::Floor; MAP]; MAP];

        for i in 0..MAP {
            map[0][i] = Tile::Wall;
            map[MAP - 1][i] = Tile::Wall;
            map[i][0] = Tile::Wall;
            map[i][MAP - 1] = Tile::Wall;
        }

        //add obstacles
        for (x, y) in [(5, 5), (6, 5), (12, 10)] {
            map[y][x] = Tile::Wall;
        }

        Game {
            map,
            cam: (screen_width() / 2.0, 50.0),
            px: 2,
            py: 2,
            path: vec![],
            player_cd: 0.0,
            monsters: vec![
                Monster {
                    x: 8,
                    y: 8,
                    hp: 30,
                    cd: 0.,
                },
                Monster {
                    x: 12,
                    y: 4,
                    hp: 30,
                    cd: 0.,
                },
                Monster {
                    x: 15,
                    y: 12,
                    hp: 30,
                    cd: 0.,
                },
            ],
            texts: vec![],
            hp: 100,
            score: 0,
            gold: vec![(3, 3), (10, 2), (16, 5), (6, 14), (17, 17)],
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        //victory/loss logic
        if self.hp <= 0 || self.monsters.is_empty() {
            return true;
        }

        //update text animations
        self.texts.retain_mut(|t| {
            t.life -= dt;
            t.y -= 20. * dt;
            t.life > 0.
        });

        //mouse input logic
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let (tx, ty) = to_tile(mx, my, self.cam);

            //check if the click is inside the map bounds
            if tx < MAP && ty < MAP && self.map[ty][tx] == Tile::Floor {
                self.path = bfs(&self.map, (self.px, self.py), (tx, ty));
            }
        }

        //handle movement of the player
        if !self.path.is_empty() {
            self.player_cd -= dt;

            //time to move?
            if self.player_cd <= 0. {
                self.player_cd = 0.15;

                let (nx, ny) = self.path[0];

                //combat logic for player
                if let Some(i) = self.monsters.iter().position(|m| m.x == nx && m.y == ny) {
                    //attack monster
                    self.damage_monster(i, 10);
                    //stop moving
                    self.path.clear();
                } else {
                    //move player
                    self.path.remove(0);
                    self.px = nx;
                    self.py = ny;

                    //collect gold
                    if let Some(i) = self.gold.iter().position(|&g| g == (self.px, self.py)) {
                        self.gold.remove(i);
                        self.score += 100;

                        //spawn green text
                        let (sx, sy) = to_screen(self.px, self.py, self.cam);
                        self.texts.push(DmgText {
                            x: sx,
                            y: sy - 40.,
                            dmg: -100,
                            life: 1.0,
                        });
                    }
                }
            }
        }

        //Monster Logic
        //check occupied tiles to monsters dont stack
        let occupied: Vec<_> = self
            .monsters
            .iter()
            .map(|m| (m.x, m.y))
            .chain(std::iter::once((self.px, self.py)))
            .collect();

        for i in 0..self.monsters.len() {
            self.monsters[i].cd -= dt;
            if self.monsters[i].cd <= 0. {
                self.monsters[i].cd = 1.; //slow

                let (mx, my) = (self.monsters[i].x, self.monsters[i].y);
                let d = dist((mx, my), (self.px, self.py));

                if d == 1 {
                    self.hp -= 5;
                    let (sx, sy) = to_screen(self.px, self.py, self.cam);
                    self.texts.push(DmgText {
                        x: sx,
                        y: sy - 40.,
                        dmg: 5,
                        life: 1.0,
                    });
                } else {
                    //chase player
                    let path = bfs(&self.map, (mx, my), (self.px, self.py));
                    if path.len() > 1 && !occupied.contains(&path[0]) {
                        self.monsters[i].x = path[0].0;
                        self.monsters[i].y = path[0].1;
                    }
                }
            }
        }
        false
    }

    //helper function to damage a monster
    fn damage_monster(&mut self, idx: usize, dmg: i32) {
        self.monsters[idx].hp -= dmg;

        //spawn text
        let (sx, sy) = to_screen(self.monsters[idx].x, self.monsters[idx].y, self.cam);
        self.texts.push(DmgText {
            x: sx,
            y: sy,
            dmg,
            life: 1.0,
        });

        //kill logic
        if self.monsters[idx].hp <= 0 {
            self.monsters.remove(idx);
            self.score += 50;
        }
    }

    pub fn draw(&self) {
        for y in 0..MAP {
            for x in 0..MAP {
                if self.map[y][x] == Tile::Wall {
                    draw_walls(x, y, self.cam);
                } else {
                    let (sx, sy) = to_screen(x, y, self.cam);

                    if self.gold.contains(&(x, y)) {
                        draw_circle(sx, sy + 16., 6., GOLD);
                    } else {
                        draw_circle(sx, sy + 16., 2., LIGHTGRAY);
                    }
                }
            }
        }

        //draw path
        for (px, py) in &self.path {
            let (sx, sy) = to_screen(*px, *py, self.cam);
            draw_circle(sx, sy + 16., 4., GOLD);
        }

        //draw player
        draw_stickman(self.px, self.py, self.cam, false);

        //draw monsters
        for monster in &self.monsters {
            draw_stickman(monster.x, monster.y, self.cam, true);
        }

        //draw floating texts
        for text in &self.texts {
            if text.dmg < 0 {
                draw_text(format!("+{}", -text.dmg), text.x, text.y, 20., GREEN);
            } else {
                draw_text(format!("-{}", text.dmg), text.x, text.y, 20., RED);
            }
        }

        //HUD
        draw_text(
            format!("HP: {}", self.hp),
            20.,
            screen_height() - 40.,
            30.,
            BLACK,
        );
        draw_text(
            format!("Score: {}", self.score),
            20.,
            screen_height() - 70.,
            30.,
            BLACK,
        );
    }
}
