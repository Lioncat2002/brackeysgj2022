use std::vec;
use macroquad::ui::widgets::Window;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::Group;
use macroquad::{prelude::*, hash};

use macroquad_platformer::*;
use macroquad_tiled as tiled;
use tiled::Map;

#[derive(Clone, Copy)]
enum CurrentLevel {
    Menu,
    Level1,
    Level2,
    Level3,
    End,
}
#[derive(Clone, Copy)]
struct Player {
    sprite: Texture2D,
    collider: Actor,
    speed: Vec2,
}
#[derive(Clone, Copy)]
struct Enemy {
    sprite: Texture2D,
    collider: Actor,
}
struct Level {
    tilemap: Map,
    player: Player,
    enemies: Vec<Enemy>,
    world: World,
    game_endpoint: Rect,
    current_level: CurrentLevel,
    next_level: CurrentLevel,
}

impl Level {
    fn update(&mut self) -> CurrentLevel {
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

        //Collision checking and stuff
        let player_rect = Rect::new(pos.x, pos.y, 32., 32.);
        for enemy in &self.enemies {
            let enemy_pos = self.world.actor_pos(enemy.collider);
            let enemy_rect = Rect::new(enemy_pos.x, enemy_pos.y, 32., 32.);
            if player_rect.intersect(enemy_rect).is_some() {
                println!("Game Over!");
            }
        }

        //Check for game end
        if player_rect.intersect(self.game_endpoint).is_some() {
            println!("You Win!");
            self.next_level
        } else {
            self.current_level
        }
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
        for enemy in &self.enemies {
            let enemy_pos = self.world.actor_pos(enemy.collider);
            draw_texture_ex(
                enemy.sprite,
                enemy_pos.x,
                enemy_pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(32., 32.)),
                    ..Default::default()
                },
            );
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Brackeys Game Jam 2022".to_owned(),
        fullscreen: false,
        window_height: 512,
        window_width: 1024,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    //Player stuff
    let player_tex = load_texture("assets/player.png").await.unwrap();
    player_tex.set_filter(FilterMode::Nearest);

    //Enemy stuff
    let enemy_tex = load_texture("assets/enemy.png").await.unwrap();
    enemy_tex.set_filter(FilterMode::Nearest);
    //Levels
    //1st level
    let mut tileset = load_texture("assets/tileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);
    let mut tiledmap_json = load_string("assets/level1.json").await.unwrap();
    let mut tilemap = tiled::load_map(&tiledmap_json, &[("tileset.png", tileset)], &[]).unwrap();

    let mut static_colliders = vec![];
    let mut world1 = World::new();
    for (_x, _y, _tile) in tilemap.tiles("Platforms", None) {
        if _tile.is_some() {
            static_colliders.push(Tile::Collider);
        } else {
            static_colliders.push(Tile::Empty);
        }
    }
    world1.add_static_tiled_layer(
        static_colliders,
        tilemap.raw_tiled_map.tilewidth as f32 * 2.,
        tilemap.raw_tiled_map.tileheight as f32 * 2.,
        tilemap.raw_tiled_map.width as _,
        1,
    );

    let mut player = Player {
        collider: world1.add_actor(Vec2::new(48., 48.), 32, 32),
        sprite: player_tex,
        speed: Vec2::new(0., 0.),
    };

    let mut enemy1 = Enemy {
        collider: world1.add_actor(Vec2::new(200., 130.), 32, 32),
        sprite: enemy_tex,
    };

    let mut level1 = Level {
        tilemap: tilemap,
        player: player,
        enemies: [enemy1].to_vec(),
        world: world1,
        game_endpoint: Rect::new(32. * 30., 4. * 32., 32., 32.),
        current_level: CurrentLevel::Level1,
        next_level: CurrentLevel::Level2,
    };
    
    //2nd level
     tiledmap_json = load_string("assets/level2.json").await.unwrap();
    tilemap = tiled::load_map(&tiledmap_json, &[("tileset.png", tileset)], &[]).unwrap();

    let mut static_colliders = vec![];
    let mut world2 = World::new();
    for (_x, _y, _tile) in tilemap.tiles("Platforms", None) {
        if _tile.is_some() {
            static_colliders.push(Tile::Collider);
        } else {
            static_colliders.push(Tile::Empty);
        }
    }
    world2.add_static_tiled_layer(
        static_colliders,
        tilemap.raw_tiled_map.tilewidth as f32 * 2.,
        tilemap.raw_tiled_map.tileheight as f32 * 2.,
        tilemap.raw_tiled_map.width as _,
        1,
    );
    //finally understood the need of that #[derive(Clone,Copy)]
    player.collider = world2.add_actor(Vec2::new(48., 48.), 32, 32);

    enemy1.collider = world2.add_actor(Vec2::new(290., 195.), 32, 32);

    let mut level2 = Level {
        tilemap: tilemap,
        player: player,
        enemies: [enemy1].to_vec(),
        world: world2,
        game_endpoint: Rect::new(32. * 30., 13. * 32., 32., 32.),
        current_level: CurrentLevel::Level2,
        next_level: CurrentLevel::End,
    };
    //Level selection n stuff
    let mut current_level = CurrentLevel::Menu;
    
    loop {
        clear_background(WHITE);
    
        match current_level {
            CurrentLevel::Menu => {
                Window::new(hash!(), vec2(400., 100.), vec2(320., 320.))
                .label("In a Coma")
                .titlebar(true)
                .ui(&mut *root_ui(), |ui| {
                    
                        Group::new(hash!("In a Coma"), Vec2::new(300., 80.)).ui(ui, |ui| {
                            ui.label(Vec2::new(140., 10.), "Pussy");
        
                            if ui.button(Vec2::new(140.,40.), "Start") {
                                current_level=CurrentLevel::Level1;
                            }
                        });
                    
                });
            },
            CurrentLevel::Level1 => {
                current_level = level1.update();
                level1.draw();
            }
            CurrentLevel::Level2 => {
                current_level = level2.update();
                level2.draw();
            }
            CurrentLevel::Level3 => todo!(),
            CurrentLevel::End=>{
                Window::new(hash!(), vec2(400., 100.), vec2(320., 320.))
                .label("You have done it!")
                .titlebar(true)
                .ui(&mut *root_ui(), |ui| {
                    
                        Group::new(hash!("You have done it!"), Vec2::new(300., 80.)).ui(ui, |ui| {
                            ui.label(Vec2::new(140., 10.), "Look! he woke up!");
                        });
                    
                });
            },
        }

        next_frame().await;
    }
}
