#![no_std]
#![no_main]

pub mod doom;
pub mod eadk;
pub mod graphics;
pub mod math;

use core::f32;

use doom::{Cube, CubeFace, Map, Player};
use eadk::{
    input::{Key, KeyboardState},
    Color,
};
use graphics::Buffer;
use math::Vec2;

#[used]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 12] = *b"Wolfenstein\0";

#[used]
#[link_section = ".rodata.eadk_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

const MAP_SIZE: (usize, usize) = (2, 2);

const WALL_TEX: [[u16; 16]; 16] = [
    [
        0xFFFF, 0xFFFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFFFF, 0xFFFF,
    ],
    [
        0xFFFF, 0xFFFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFFFF, 0xFFFF,
    ],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [
        0xFFFF, 0xFFFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFFFF, 0xFFFF,
    ],
    [
        0xFFFF, 0xFFFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFFFF, 0xFFFF,
    ],
];

#[no_mangle]
pub fn main() {
    let mut buff = Buffer::new();
    const TILE_SIZE: u16 = 16;
    const W: Option<Cube> = Some(Cube {
        faces: [
            CubeFace { data: WALL_TEX },
            CubeFace { data: WALL_TEX },
            CubeFace { data: WALL_TEX },
            CubeFace { data: WALL_TEX },
        ],
    });
    const L: Option<Cube> = None;
    let map = Map {
        data: [[L, W], [W, L]],
    };
    let mut player = Player::new(
        Vec2 { x: 6., y: 6. },
        -f32::consts::FRAC_PI_8,
        f32::consts::FRAC_PI_6,
    );
    loop {
        let mut mvmnt_vec = Vec2::default();
        let mut cam_mvmnt = 0.;
        let keys = KeyboardState::scan();
        if keys.key_down(Key::Up) {
            mvmnt_vec.x += 1.;
        }
        if keys.key_down(Key::Down) {
            mvmnt_vec.x -= 1.;
        }
        if keys.key_down(Key::Left) {
            mvmnt_vec.y -= 1.;
        }
        if keys.key_down(Key::Right) {
            mvmnt_vec.y += 1.;
        }
        // cam rotation
        if keys.key_down(Key::Ok) {
            cam_mvmnt -= 1.;
        }
        if keys.key_down(Key::Back) {
            cam_mvmnt += 1.;
        }
        player.move_player(mvmnt_vec * 0.08, cam_mvmnt * 0.04, &map);
        // fov manipulation
        let mut dfov = 0.;
        if keys.key_down(Key::Plus) {
            dfov += 1.;
        }
        if keys.key_down(Key::Minus) {
            dfov -= 1.;
        }
        player.set_fov(player.get_fov() + dfov * 0.02);
        //
        buff.clear(Color { rgb565: 0 });
        player.render_view(&mut buff, &map);
        map.draw(&mut buff, TILE_SIZE, false, Some(&player));
        buff.render();
    }
}
