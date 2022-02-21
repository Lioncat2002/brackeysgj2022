use std::vec;

use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;
use tiled::Map;

struct Player {
    sprite: Texture2D,
    collider: Actor,
    speed: Vec2,
}

struct Level {
    tilemap: Map,
    player: Player,
    world: World,
}

impl Level {
    fn update(&mut self) {
        //player movement

        if is_key_down(KeyCode::A) {
            self.player.speed.x = -200.;
        } else if is_key_down(KeyCode::D) {
            self.player.speed.x = 200.;
        } else {
            self.player.speed.x = 0.;
        }

        let pos = self.world.actor_pos(self.player.collider);
        let on_ground = self
            .world
            .collide_check(self.player.collider, pos + vec2(0., 1.));

        if on_ground == false {
            self.player.speed.y += 2000. * get_frame_time();
        } else {
            self.player.speed.y = 0.;
        }

        if is_key_down(KeyCode::Space) {
            if on_ground {
                self.player.speed.y = -700.;
            }
        }

        self.world
            .move_h(self.player.collider, self.player.speed.x * get_frame_time());
        self.world
            .move_v(self.player.collider, self.player.speed.y * get_frame_time());
    }

    fn draw(&self) {
        self.tilemap
            .draw_tiles("Background", Rect::new(0.0, 0.0, 1024., 1024.), None);
        self.tilemap
            .draw_tiles("Platforms", Rect::new(0.0, 0.0, 1024., 1024.), None);
        self.tilemap
            .draw_tiles("cloudsnprops", Rect::new(0.0, 0.0, 1024., 1024.), None);
        let position = self.world.actor_pos(self.player.collider);

        draw_texture_ex(
            self.player.sprite,
            position.x,
            position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(32., 32.)),
                ..Default::default()
            },
        );
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        fullscreen: false,
        window_height: 512,
        window_width: 1024,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    //Tilemap stuff
    let tileset = load_texture("assets/tileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);
    let tiledmap_json = load_string("assets/map.json").await.unwrap();
    let tilemap = tiled::load_map(&tiledmap_json, &[("tileset.png", tileset)], &[]).unwrap();
    //Player stuff
    let player = load_texture("assets/player.png").await.unwrap();
    player.set_filter(FilterMode::Nearest);

    let mut static_colliders = vec![];
    let mut world = World::new();
    for (_x, _y, _tile) in tilemap.tiles("Platforms", None) {
        if _tile.is_some() {
            static_colliders.push(Tile::Collider);
        } else {
            static_colliders.push(Tile::Empty);
        }
    }
    world.add_static_tiled_layer(
        static_colliders,
        tilemap.raw_tiled_map.tilewidth as f32 * 2.,
        tilemap.raw_tiled_map.tileheight as f32 * 2.,
        tilemap.raw_tiled_map.width as _,
        1,
    );
    let player = Player {
        collider: world.add_actor(Vec2::new(48., 48.), 32, 32),
        sprite: player,
        speed: Vec2::new(0., 0.),
    };
    let mut level1 = Level {
        tilemap: tilemap,
        player: player,
        world: world,
    };

    loop {
        clear_background(WHITE);
        level1.update();
        level1.draw();
        next_frame().await;
    }
}
