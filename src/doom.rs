use crate::{
    eadk::{
        display::{SCREEN_HEIGHT, SCREEN_WIDTH},
        Color, Rect,
    },
    graphics::Buffer,
    math::{
        large_step,
        trigo::{atan, cos, inv_tan, sin, tan},
        Vec2, Vec2i,
    },
};
use core::f32;

const MAX_COLLISION_DEPTH: u16 = 100;
const PLAYER_HALF_THICKNESS: f32 = 0.42;

/// the player
pub struct Player {
    /// player position
    pub pos: Vec2,
    /// camera angle
    pub theta: f32,
    /// camera half fov
    fov: f32,
    /// vertical fov
    vfov: f32,
}

impl Player {
    /// new player
    pub fn new(pos: Vec2, theta: f32, fov: f32) -> Self {
        let mut p = Player {
            pos,
            theta,
            fov: fov.clamp(0.1, f32::consts::FRAC_PI_2 - 0.1),
            vfov: 0.,
        };
        p.recalc_vfov();
        p
    }

    /// returns the (horizontal) fov
    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    /// set the fov
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov.clamp(0.1, f32::consts::FRAC_PI_2 - 0.1);
        self.recalc_vfov();
    }

    /// get the vertical fov
    pub fn get_vfov(&self) -> f32 {
        self.vfov
    }

    /// re-compute vertical fov
    fn recalc_vfov(&mut self) {
        self.vfov = self.fov * SCREEN_HEIGHT as f32 / SCREEN_WIDTH as f32;
    }

    /// get the current tile
    pub fn get_tile(&self) -> Vec2i {
        self.pos.floor()
    }

    /// X-aligned rtx
    fn trace_x<'a>(&self, theta: f32, map: &'a Map) -> Option<(f32, (&'a CubeFace, f32))> {
        let theta = self.theta + theta;
        let y;
        if sin(theta) > 0. {
            y = (self.pos.y as i32 + 1) as f32 - self.pos.y;
        } else if sin(theta) < 0. {
            y = (self.pos.y as i32) as f32 - self.pos.y;
        } else {
            return None;
        }
        let ini_x = Vec2 {
            x: y * inv_tan(theta),
            y,
        };
        let delta_x = Vec2 {
            x: large_step(sin(theta), 0.) * inv_tan(theta),
            // -1 or 1, depending of the sign of sin(theta)
            y: large_step(sin(theta), 0.),
        };
        let phi = Vec2 { x: 1., y: 0. }.rotated(theta) * 0.001;
        for i in 0..MAX_COLLISION_DEPTH {
            let v = self.pos + ini_x + delta_x * i as f32;
            if let Some(c) = map.get_at((v + phi).floor()) {
                return Some((
                    (v - self.pos).norm(),
                    (
                        &c.faces[if sin(theta) > 0. { 0 } else { 2 }],
                        v.x - v.x as i16 as f32,
                    ),
                ));
            }
        }
        None
    }

    /// Y-aligned rtx
    fn trace_y<'a>(&self, theta: f32, map: &'a Map) -> Option<(f32, (&'a CubeFace, f32))> {
        let theta = self.theta + theta;
        let x;
        if cos(theta) > 0. {
            x = (self.pos.x as i32 + 1) as f32 - self.pos.x;
        } else if cos(theta) < 0. {
            x = (self.pos.x as i32) as f32 - self.pos.x;
        } else {
            return None;
        }
        let ini_y = Vec2 {
            x,
            y: x * tan(theta),
        };
        let delta_y = Vec2 {
            x: large_step(cos(theta), 0.),
            // -1 or 1, depending of the sign of cos(theta)
            y: large_step(cos(theta), 0.) * tan(theta),
        };
        let phi = Vec2 { x: 1., y: 0. }.rotated(theta) * 0.001;
        for i in 0..MAX_COLLISION_DEPTH {
            let v = self.pos + ini_y + delta_y * i as f32;
            if let Some(c) = map.get_at((v + phi).floor()) {
                return Some((
                    (v - self.pos).norm(),
                    (
                        &c.faces[if cos(theta) > 0. { 3 } else { 1 }],
                        v.y - v.y as i16 as f32,
                    ),
                ));
            }
        }
        None
    }

    /// check collision at a deviated angle theta from the pointing direction
    fn ray_trace<'a>(&self, theta: f32, map: &'a Map) -> Option<(f32, (&'a CubeFace, f32))> {
        let trace_x = self.trace_x(theta, map);
        let trace_y = self.trace_y(theta, map);
        match (trace_x, trace_y) {
            (None, None) => None,
            (Some(t1), None) => Some(t1),
            (None, Some(t2)) => Some(t2),
            (Some(t1), Some(t2)) => {
                if t1.0 < t2.0 {
                    Some(t1)
                } else {
                    Some(t2)
                }
            }
        }
    }

    /// draw the player view to a graphical buffer
    pub fn render_view(&self, buff: &mut Buffer, map: &Map) {
        let precalc = (2. * self.get_fov()) / SCREEN_WIDTH as f32;
        for i in 0..SCREEN_WIDTH {
            let ray = self.ray_trace((i as i32 - SCREEN_WIDTH as i32 / 2) as f32 * precalc, map);
            if let Some(r) = ray {
                let rect_height = ((SCREEN_HEIGHT) as f32 * atan(0.5 / r.0) / self.get_vfov())
                    .clamp(0., (SCREEN_HEIGHT) as f32) as u16;
                buff.push_rect_uniform(
                    Rect {
                        x: i,
                        y: (SCREEN_HEIGHT - rect_height) / 2,
                        width: 1,
                        height: rect_height,
                    },
                    Color { rgb565: 0xFFFF },
                    // &[Color { rgb565: 0xFFFF }; SCREEN_HEIGHT as usize],
                    // &r.1 .0.sample(r.1 .1, rect_height),
                );
            }
        }
    }

    /// handle player movment and collisions
    pub fn move_player(&mut self, direction: Vec2, angle: f32, map: &Map) {
        // +x is forward
        // +y is left
        let movment = direction.rotated(self.theta);
        // x move & collisions
        self.pos.x += movment.x;
        for wall in map.wall_iter() {
            // check collision for every block
            if !(self.pos.x + PLAYER_HALF_THICKNESS <= wall.0 as f32
                || self.pos.x - PLAYER_HALF_THICKNESS >= (wall.0 + 1) as f32
                || self.pos.y + PLAYER_HALF_THICKNESS <= wall.1 as f32
                || self.pos.y - PLAYER_HALF_THICKNESS >= (wall.1 + 1) as f32)
            {
                // if so, place player accordingly
                if movment.x > 0. {
                    self.pos.x = wall.0 as f32 - PLAYER_HALF_THICKNESS;
                } else {
                    self.pos.x = (wall.0 + 1) as f32 + PLAYER_HALF_THICKNESS;
                }
            }
        }
        // y move & collisions
        self.pos.y += movment.y;
        for wall in map.wall_iter() {
            // check collision for every block
            if !(self.pos.x + PLAYER_HALF_THICKNESS <= wall.0 as f32
                || self.pos.x - PLAYER_HALF_THICKNESS >= (wall.0 + 1) as f32
                || self.pos.y + PLAYER_HALF_THICKNESS <= wall.1 as f32
                || self.pos.y - PLAYER_HALF_THICKNESS >= (wall.1 + 1) as f32)
            {
                // if so, place player accordingly
                if movment.y > 0. {
                    self.pos.y = wall.1 as f32 - PLAYER_HALF_THICKNESS;
                } else {
                    self.pos.y = (wall.1 + 1) as f32 + PLAYER_HALF_THICKNESS;
                }
            }
        }
        // rotate cam
        self.theta += angle;
    }
}

const MAP_SIZE: (usize, usize) = (2, 2);

/// a map
pub struct Map {
    pub data: [[Option<Cube>; MAP_SIZE.1]; MAP_SIZE.0],
}

impl Map {
    /// get if there is a wall at x y
    pub fn get_at(&self, pos: Vec2i) -> Option<&Cube> {
        if pos.x < 0 || pos.y < 0 || pos.x >= MAP_SIZE.0 as i32 || pos.y >= MAP_SIZE.1 as i32 {
            return None;
        }
        self.data[pos.y as usize][pos.x as usize].as_ref()
    }

    /// iterate over the walls positions
    pub fn wall_iter(&self) -> MapWallIterator {
        MapWallIterator {
            map: &self,
            x: 0,
            y: 0,
        }
    }

    /// draws the map to a buffer
    pub fn draw(
        &self,
        buff: &mut Buffer,
        tile_size: u16,
        draw_empty: bool,
        player: Option<&Player>,
    ) {
        for (y, row) in self.data.iter().enumerate() {
            for (x, e) in row.iter().enumerate() {
                if let Some(_) = e {
                    buff.push_rect_uniform(
                        Rect {
                            x: x as u16 * tile_size,
                            y: y as u16 * tile_size,
                            width: tile_size,
                            height: tile_size,
                        },
                        Color {
                            rgb565: 0b1111100000011111,
                        },
                    );
                } else {
                    if draw_empty {
                        buff.push_rect_uniform(
                            Rect {
                                x: x as u16 * tile_size,
                                y: y as u16 * tile_size,
                                width: tile_size,
                                height: tile_size,
                            },
                            Color {
                                rgb565: 0b0011100011100111,
                            },
                        );
                    }
                }
            }
        }
        if let Some(p) = player {
            buff.circle(
                p.pos * tile_size as f32,
                tile_size as f32 / 4.,
                Color {
                    rgb565: 0b11111100000,
                },
            );
            // fov
            buff.line(
                p.pos * tile_size as f32,
                (p.pos + Vec2 { x: 2., y: 0. }.rotated(p.theta + p.get_fov())) * tile_size as f32,
                Color { rgb565: 0b11111 },
            );
            buff.line(
                p.pos * tile_size as f32,
                (p.pos + Vec2 { x: 2., y: 0. }.rotated(p.theta - p.get_fov())) * tile_size as f32,
                Color { rgb565: 0b11111 },
            );
            // direction
            buff.line(
                p.pos * tile_size as f32,
                (p.pos + Vec2 { x: 2., y: 0. }.rotated(p.theta)) * tile_size as f32,
                Color {
                    rgb565: 0b11111111111,
                },
            );
        }
    }
}

/// an object ti iterate over the walls of a map
pub struct MapWallIterator<'a> {
    map: &'a Map,
    x: usize,
    y: usize,
}

/// the iterator implementation
impl<'a> Iterator for MapWallIterator<'a> {
    type Item = (usize, usize, &'a Cube);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.x >= MAP_SIZE.0 {
                self.x = 0;
                self.y += 1;
            }
            if self.y >= MAP_SIZE.1 {
                return None;
            }
            // return position and add 1 if there is a wall
            if let Some(c) = &self.map.data[self.y][self.x] {
                let pos = (self.x, self.y, c);
                self.x += 1;
                return Some(pos);
            }
            // move next otherwise
            self.x += 1;
        }
    }
}

pub struct Cube {
    pub faces: [CubeFace; 4],
}

pub struct CubeFace {
    pub data: [[u16; 16]; 16],
}

impl CubeFace {
    pub fn sample<'a>(&self, pos: f32, size: u16) -> [Color; SCREEN_HEIGHT as usize] {
        let row = ((pos * 16.) as usize).clamp(0, 15);
        let mut colors = [Color { rgb565: 0 }; SCREEN_HEIGHT as usize];
        for i in 0..(usize::max(size as usize, SCREEN_HEIGHT as usize)) {
            colors[i].rgb565 = self.data[row][16 * i / size as usize];
        }
        colors
    }
}
