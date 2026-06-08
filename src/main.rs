use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;

// Making window conf
fn window_conf() -> Conf {
    Conf {
        window_title: "My first platformer".to_owned(),
        window_width: 1280,
        window_height: 640,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");

    let tileset_png = load_texture("Tileset.png").await.unwrap();
    let cave_entrance_png = load_texture("cave_entrance.png").await.unwrap();

    tileset_png.set_filter(FilterMode::Nearest);
    cave_entrance_png.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("map.json").await.unwrap();

    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[
            ("Tileset.png", tileset_png),
            ("cave_entrance.png", cave_entrance_png),
        ],
        &[],
    )
    .unwrap();

    let mut static_colider = vec![];

    for (_x, _y, tile) in tiled_map.tiles("world", None) {
        if tile.is_some() {
            static_colider.push(Tile::Solid);
        } else {
            static_colider.push(Tile::Empty);
        }
    }

    let mut physics_world = World::new();
    physics_world.add_static_tiled_layer(static_colider, 32., 32., 40, 1);

    let player_actor = physics_world.add_actor(vec2(200., 100.), 32, 32);

    let mut player_speed_x: f32 = 0.;
    let mut player_speed_y: f32 = 0.;
    let max_horizontal_speed = 300.;

    let game_camera = Camera2D::from_display_rect(Rect::new(0., 0., 1280., 640.));

    loop {
        clear_background(SKYBLUE);

        let dt = get_frame_time();

        let current_pos = physics_world.actor_pos(player_actor);
        let is_on_ground = physics_world.collide_check(player_actor, current_pos + vec2(0., 1.));

        if !is_on_ground {
            player_speed_y *= 1200. * dt;
        } else {
            player_speed_y = 0.;
        }

        //Moving to right
        if is_key_down(KeyCode::D) {
            player_speed_x = max_horizontal_speed;
        }
        //Moving to left
        else if is_key_down(KeyCode::A) {
            player_speed_x = -max_horizontal_speed;
        } else {
            player_speed_x = 0.; // Stop moving sideways instantly if keys are lifted
        }

        // Jump trigger inputs
        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Space) {
            if is_on_ground {
                player_speed_y = -450.0; // Instantly propel upwards
            }
        }

        physics_world.move_h(player_actor, player_speed_x * dt);
        physics_world.move_v(player_actor, player_speed_y * dt);

        let updated_pos = physics_world.actor_pos(player_actor);

        set_camera(&game_camera);

        tiled_map.draw_tiles("world", Rect::new(0., 0., 1280., 640.), None);
        draw_rectangle(updated_pos.x, updated_pos.y, 32., 32., ORANGE);

        set_default_camera();
        next_frame().await
    }
}
