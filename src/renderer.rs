use macroquad::prelude::*;
use crate::map::to_screen;

//draw hero and monsters
pub fn draw_stickman(x: usize, y: usize, cam: (f32, f32), enemy: bool) {
    let (sx, mut sy) = to_screen(x, y, cam);
    sy += 16.;

    //shadow
    draw_ellipse(sx, sy + 3., 10., 5., 0., Color::new(0., 0., 0., 0.2));

    //head
    if enemy {
        draw_line(sx - 5., sy - 32., sx, sy - 30., 2., BLACK);
        draw_line(sx + 5., sy - 32., sx, sy - 30., 2., BLACK);
    } else {
        //player
        draw_circle_lines(sx, sy - 32., 7., 2., BLACK);
    }

    //body and limbs
    for l in [
        [0., -25., 0., -8.],
        [0., -20., -8., -15.],
        [0., -20., 8., -15.],
        [0., -8., -6., 0.],
        [0., -8., 6., 0.],
    ] {
        draw_line(sx + l[0], sy + l[1], sx + l[2], sy + l[3], 2., BLACK);
    }
}
