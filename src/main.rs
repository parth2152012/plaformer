use macroquad::prelude::*;
use macroquad_tiled as tiled;
// 1. Bring in the platformer physics types
use macroquad_platformer::*;

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

    // 2. Build a physics world map grid based on your Tiled design
    let mut static_colliders = vec![];

    // We look through every row and column of your 40x20 tile layout
    for (_x, _y, tile) in tiled_map.tiles("world", None) {
        if tile.is_some() {
            static_colliders.push(Tile::Solid); // Land blocks block movement
        } else {
            static_colliders.push(Tile::Empty); // Sky lets you pass through
        }
    }

    let mut physics_world = World::new();
    // Parameters: (our array of tiles, tile_width, tile_height, map_width_in_tiles, room_index)
    physics_world.add_static_tiled_layer(static_colliders, 32.0, 32.0, 40, 1);

    // 3. Spawn our player body inside the physics engine world simulation
    // Arguments: (starting position vector, body width, body height)
    let player_actor = physics_world.add_actor(vec2(200.0, 100.0), 32, 32);

    // We'll separate horizontal and vertical speed values
    let mut player_speed_x = 0.0;
    let mut player_speed_y = 0.0;
    let max_horizontal_speed = 300.0;

    let game_camera = Camera2D::from_display_rect(Rect::new(0., 0., 1280., 640.));

    loop {
        clear_background(SKYBLUE);
        let dt = get_frame_time();

        // 4. Read the current position directly from the physics engine simulation
        let current_pos = physics_world.actor_pos(player_actor);

        // Check if the player box is standing on solid ground
        let is_on_ground = physics_world.collide_check(player_actor, current_pos + vec2(0.0, 1.0));

        // 5. Basic Gravity Simulation
        if !is_on_ground {
            player_speed_y += 1200.0 * dt; // Pull downward over time if in the air
        } else {
            player_speed_y = 0.0; // Stop accumulating downward velocity on the ground
        }

        // 6. Movement Inputs
        if is_key_down(KeyCode::D) {
            player_speed_x = max_horizontal_speed;
        } else if is_key_down(KeyCode::A) {
            player_speed_x = -max_horizontal_speed;
        } else {
            player_speed_x = 0.0; // Stop moving sideways instantly if keys are lifted
        }

        // Jump trigger inputs
        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Space) {
            if is_on_ground {
                player_speed_y = -450.0; // Instantly propel upwards
            }
        }

        // 7. Tell the physics system to move our actor box.
        // It will automatically stop you if a Solid tile gets in your way!
        physics_world.move_h(player_actor, player_speed_x * dt);
        physics_world.move_v(player_actor, player_speed_y * dt);

        // Fetch the newly updated position after collision checks calculated
        let updated_pos = physics_world.actor_pos(player_actor);

        set_camera(&game_camera);

        tiled_map.draw_tiles("world", Rect::new(0., 0., 1280., 640.), None);

        // 8. Draw our character block using the checked coordinates
        draw_rectangle(updated_pos.x, updated_pos.y, 32., 32., ORANGE);

        set_default_camera();

        next_frame().await
    }
}
