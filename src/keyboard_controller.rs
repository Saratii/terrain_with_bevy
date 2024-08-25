use bevy::{input::ButtonInput, prelude::{KeyCode, Query, Res, Transform, Visibility, With, Without}, time::Time};

use crate::{components::{ContentList, CurrentTool, Grid, PickaxeTag, PlayerTag, ShovelTag, TerrainGridTag, Tool, Velocity}, constants::{FRICTION, MAX_PLAYER_SPEED, PLAYER_HEIGHT, PLAYER_WIDTH}, player::apply_velocity, world_generation::does_gravity_apply_to_entity};

pub fn process_key_event(
    mut grid_query: Query<&mut Grid, (With<TerrainGridTag>, Without<ShovelTag>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Velocity), (With<PlayerTag>, Without<ShovelTag>)>,
    time: Res<Time>,
    mut current_tool: Query<&mut CurrentTool>,
    shovel_contents_query: Query<&mut ContentList, With<ShovelTag>>,
    mut shovel_visability_query: Query<&mut Visibility, (With<ShovelTag>, Without<PickaxeTag>)>,
    mut pickaxe_visability_query: Query<&mut Visibility, (With<PickaxeTag>, Without<ShovelTag>)>,
){
    let shovel_contents = shovel_contents_query.get_single().unwrap();
    let grid = grid_query.get_single_mut().unwrap();
    let mut player = player_query.get_single_mut().unwrap();
    let does_gravity_apply = does_gravity_apply_to_entity(player.0.translation.x as i32 - PLAYER_WIDTH as i32/2,  player.0.translation.y as i32, PLAYER_WIDTH as i32, PLAYER_HEIGHT as i32, &grid.data);
    if does_gravity_apply {
        player.1.vy -= 1. * time.delta_seconds();
    } else {
        player.1.vy = 0.;
        if player.1.vx > 0. {
            player.1.vx -= FRICTION * time.delta_seconds();
        } else if player.1.vx < 0. {
            player.1.vx += FRICTION * time.delta_seconds();
        }
    }
    if keys.pressed(KeyCode::Digit1){
        let mut shovel_visability = shovel_visability_query.get_single_mut().unwrap();
        let mut pickaxe_visability = pickaxe_visability_query.get_single_mut().unwrap();
        *shovel_visability = Visibility::Visible;
        *pickaxe_visability = Visibility::Hidden;
        let mut current_tool = current_tool.get_single_mut().unwrap();
        current_tool.tool = Tool::Shovel;
    } else if keys.pressed(KeyCode::Digit2) && shovel_contents.contents.len() == 0{
        let mut shovel_visability = shovel_visability_query.get_single_mut().unwrap();
        let mut pickaxe_visability = pickaxe_visability_query.get_single_mut().unwrap();
        *shovel_visability = Visibility::Hidden;
        *pickaxe_visability = Visibility::Visible;
        let mut current_tool = current_tool.get_single_mut().unwrap();
        current_tool.tool = Tool::Pickaxe;
    }
    if keys.pressed(KeyCode::KeyA) {
        if player.1.vx * -1. < MAX_PLAYER_SPEED {
            player.1.vx -= 1. * time.delta_seconds();
        }
    } else if keys.pressed(KeyCode::KeyD) {
         if player.1.vx < MAX_PLAYER_SPEED {
            player.1.vx += 1. * time.delta_seconds();
        }
    }
    if keys.pressed(KeyCode::Space){
        if !does_gravity_apply{
            player.1.vy += 150. * time.delta_seconds();
        }
    }
    apply_velocity(&mut player.0.translation, &mut player.1, &grid.data);
}