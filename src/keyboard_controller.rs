use bevy::{ecs::event::EventWriter, input::ButtonInput, prelude::{KeyCode, Query, Res, Transform, Visibility, With, Without}, time::Time};

use crate::{chunk_generator::NewChunkEvent, components::{Bool, ChunkMap, ContentList, PlayerTag, Velocity}, constants::{FRICTION, MAX_PLAYER_SPEED, PLAYER_ACCELERATION, PLAYER_HEIGHT, PLAYER_WIDTH}, player::apply_velocity, tools::{CurrentTool, HoeTag, PickaxeTag, ShovelTag, Tool}, world_generation::does_gravity_apply_to_entity};

pub fn process_key_event(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<PlayerTag>>,
    time: Res<Time>,
    shovel_contents_query: Query<&mut ContentList, With<ShovelTag>>,
    mut current_tool_query: Query<&mut CurrentTool>,
    mut shovel_visability_query: Query<&mut Visibility, (With<ShovelTag>, Without<PickaxeTag>, Without<HoeTag>)>,
    mut pickaxe_visability_query: Query<&mut Visibility, (With<PickaxeTag>, Without<ShovelTag>, Without<HoeTag>)>,
    mut hoe_visability_query: Query<&mut Visibility, (With<HoeTag>, Without<PickaxeTag>, Without<ShovelTag>)>,
    mut hoe_is_locked_query: Query<&mut Bool, With<HoeTag>>,
    mut chunk_map_query: Query<&mut ChunkMap>,
    mut chunk_event_writer: EventWriter<NewChunkEvent>,
) {
    let shovel_contents = shovel_contents_query.get_single().unwrap();
    let chunk_map = &mut chunk_map_query.get_single_mut().unwrap();
    let mut hoe_is_locked = hoe_is_locked_query.get_single_mut().unwrap();
    let mut player = player_query.get_single_mut().unwrap();
    let does_gravity_apply = does_gravity_apply_to_entity(player.0.translation, PLAYER_WIDTH as i32, PLAYER_HEIGHT as i32, &mut chunk_map.map, &mut chunk_event_writer);
    if does_gravity_apply{
        player.1.vy -= 300. * time.delta_seconds();
    } else {
        player.1.vy = 0.;
        if player.1.vx > 0. {
            player.1.vx = (player.1.vx - FRICTION * time.delta_seconds()).max(0.);
        } else if player.1.vx < 0. {
            player.1.vx = (player.1.vx + FRICTION * time.delta_seconds()).min(0.);
        }
    }
    if keys.pressed(KeyCode::Digit1) {
        hoe_is_locked.bool = false;
        let mut shovel_visability = shovel_visability_query.get_single_mut().unwrap();
        let mut pickaxe_visability = pickaxe_visability_query.get_single_mut().unwrap();
        let mut hoe_visability = hoe_visability_query.get_single_mut().unwrap();
        *shovel_visability = Visibility::Visible;
        *pickaxe_visability = Visibility::Hidden;
        *hoe_visability = Visibility::Hidden;
        let mut current_tool = current_tool_query.get_single_mut().unwrap();
        current_tool.tool = Tool::Shovel;
    } else if keys.pressed(KeyCode::Digit2) && shovel_contents.contents.len() == 0 {
        hoe_is_locked.bool = false;
        let mut shovel_visability = shovel_visability_query.get_single_mut().unwrap();
        let mut pickaxe_visability = pickaxe_visability_query.get_single_mut().unwrap();
        let mut hoe_visability = hoe_visability_query.get_single_mut().unwrap();
        *shovel_visability = Visibility::Hidden;
        *pickaxe_visability = Visibility::Visible;
        *hoe_visability = Visibility::Hidden;
        let mut current_tool = current_tool_query.get_single_mut().unwrap();
        current_tool.tool = Tool::Pickaxe;
    } else if keys.pressed(KeyCode::Digit3) && shovel_contents.contents.len() == 0 {
        let mut shovel_visability = shovel_visability_query.get_single_mut().unwrap();
        let mut pickaxe_visability = pickaxe_visability_query.get_single_mut().unwrap();
        let mut hoe_visability = hoe_visability_query.get_single_mut().unwrap();
        *shovel_visability = Visibility::Hidden;
        *pickaxe_visability = Visibility::Hidden;
        *hoe_visability = Visibility::Visible;
        let mut current_tool = current_tool_query.get_single_mut().unwrap();
        current_tool.tool = Tool::Hoe;
    } else if keys.pressed(KeyCode::Digit4) && shovel_contents.contents.len() == 0 {
        let mut current_tool = current_tool_query.get_single_mut().unwrap();
        let mut shovel_visability = shovel_visability_query.get_single_mut().unwrap();
        let mut pickaxe_visability = pickaxe_visability_query.get_single_mut().unwrap();
        let mut hoe_visability = hoe_visability_query.get_single_mut().unwrap();
        *shovel_visability = Visibility::Hidden;
        *pickaxe_visability = Visibility::Hidden;
        *hoe_visability = Visibility::Hidden;
        current_tool.tool = Tool::SpawnDrill;
    }
    if keys.pressed(KeyCode::KeyA) {
        player.1.vx = (player.1.vx - PLAYER_ACCELERATION * time.delta_seconds())
            .max(-MAX_PLAYER_SPEED);
    }
    if keys.pressed(KeyCode::KeyD) {
        player.1.vx = (player.1.vx + PLAYER_ACCELERATION * time.delta_seconds())
            .min(MAX_PLAYER_SPEED);
    }
    if keys.pressed(KeyCode::Space) && !does_gravity_apply {
        player.1.vy += 150.;
    }
    apply_velocity(&mut player.0.translation, &mut player.1, &chunk_map.map, &time);
}