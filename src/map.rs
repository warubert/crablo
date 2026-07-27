use macroquad::prelude::*;
use std::collections::VecDeque;

pub const MAP: usize = 20;
pub const TILE_SIZE: (f32, f32) = (32.0, 16.0);

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

pub fn to_screen(x: usize, y: usize, cam: (f32, f32)) -> (f32, f32) {
    (
        (x as f32 - y as f32) * TILE_SIZE.0 + cam.0,
        (x as f32 + y as f32) * TILE_SIZE.1 + cam.1,
    )
}

pub fn to_tile(sx: f32, sy: f32, cam: (f32, f32)) -> (usize, usize) {
    let (ax, ay) = (sx - cam.0, sy - cam.1);

    (
        ((ax / TILE_SIZE.0 + ay / TILE_SIZE.1) / 2.) as usize,
        ((ay / TILE_SIZE.1 - ax / TILE_SIZE.0) / 2.) as usize,
    )
}

//calculate Manhattan distance between two points
pub fn dist(p1: (usize, usize), p2: (usize, usize)) -> i32 {
    (p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs()
}

//Pathfinder algorithm
pub fn bfs(
    map: &[[Tile; MAP]; MAP],
    start: (usize, usize),
    goal: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut q = VecDeque::from([start]);
    let mut visited = [[false; MAP]; MAP];
    visited[start.1][start.0] = true;

    let mut parent = [[None; MAP]; MAP];

    while let Some(curr) = q.pop_front() {
        if curr == goal {
            let mut path = vec![];
            let mut c = goal;
            while c != start {
                path.push(c);
                c = parent[c.1][c.0].unwrap();
            }
            path.reverse();
            return path;
        }

        //check close
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let (nx, ny) = ((curr.0 as i32 + dx) as usize, (curr.1 as i32 + dy) as usize);

            if nx < MAP && ny < MAP && !visited[ny][nx] && map[ny][nx] == Tile::Floor {
                visited[ny][nx] = true;
                parent[ny][nx] = Some(curr);
                q.push_back((nx, ny));
            }
        }
    }

    vec![]
}

pub fn draw_walls(x: usize, y: usize, cam: (f32, f32)) {
    let (sx, sy) = to_screen(x, y, cam);

    let v = [
        vec2(sx, sy - 40.),
        vec2(sx + 32., sy - 24.),
        vec2(sx, sy - 8.),
        vec2(sx - 32., sy - 24.),
        vec2(sx + 32., sy),
        vec2(sx, sy + 16.),
        vec2(sx - 32., sy),
    ];

    let colors = [
        Color::new(0.8, 0.8, 0.8, 1.0),
        Color::new(0.5, 0.5, 0.5, 1.0),
        Color::new(0.6, 0.6, 0.6, 1.0),
    ];

    //draw faces
    draw_triangle(v[0], v[1], v[2], colors[0]);
    draw_triangle(v[0], v[2], v[3], colors[0]);
    draw_triangle(v[1], v[4], v[5], colors[1]);
    draw_triangle(v[1], v[5], v[2], colors[1]);
    draw_triangle(v[3], v[2], v[5], colors[2]);
    draw_triangle(v[3], v[5], v[6], colors[2]);

    //draw outlines
    for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 0), (1, 4), (2, 5), (3, 6)] {
        draw_line(v[a].x, v[a].y, v[b].x, v[b].y, 1.0, BLACK);
    }
}
