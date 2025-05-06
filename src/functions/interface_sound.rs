use robotics_lib::interface::*;
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::World;
use crate::functions::destroying_sound::{play_sound_chopping_bush, play_sound_chopping_wood, play_sound_collecting_coin, play_sound_collecting_water, play_sound_destroying_garbage, play_sound_destroying_jollyblock, play_sound_lighting_off_fire, play_sound_mining_rock, play_sound_picking_fish};
use crate::functions::other::{play_sound_crafting, play_sound_teleport};
use crate::functions::put_sounds::{play_sound_coin_in_bank, play_sound_content_in_market, play_sound_empty_in_mountain, play_sound_fire_in_tree_empty, play_sound_garbage_in_bin, play_sound_rock_in_g_h_s_s, play_sound_rock_in_lava, play_sound_rock_in_water, play_sound_tree_in_crate, play_sound_water_in_fire};
pub(crate) use crate::functions::walking_sounds::*;
type TileMatrix = (Vec<Vec<Option<Tile>>>, (usize, usize));

fn give_tile(robot: &mut impl Runnable, world: &mut World, direction: Direction) -> Option<Tile>{
    let view = robot_view(robot, world);
    let mut tile;
    match &direction {
        Direction::Up => {tile = view[0][1].clone()}
        Direction::Down => {tile = view[2][1].clone()}
        Direction::Left => {tile = view[1][0].clone()}
        Direction::Right => {tile = view[1][2].clone()}
    }
    tile
}

pub fn put_with_sound(
    robot: &mut impl Runnable,
    world: &mut World,
    content_in: Content,
    quantity: usize,
    direction: Direction,
) -> Result<usize, LibError> {
    let tile = give_tile(robot, world, direction.clone());
    let result = put(robot, world, content_in.clone(), quantity, direction.clone());
    match &result {
        Ok(_) => {
            match (tile.clone().unwrap().tile_type , tile.clone().unwrap().content , &content_in) {
                | (_, Content::Bank(range), Content::Coin(_)) => {
                    play_sound_coin_in_bank()
                }
                | (_, Content::Bin(range), Content::Garbage(_)) => {
                    play_sound_garbage_in_bin()
                }
                | (_, Content::Crate(range), Content::Tree(_)) => {
                    play_sound_tree_in_crate()
                }
                | (_, Content::Tree(_), Content::Fire) | (_, Content::None, Content::Fire) => {
                    play_sound_fire_in_tree_empty()
                }
                | (_, Content::Market(remaining_op), to_sell) => {
                    play_sound_content_in_market()
                }
                | (TileType::Grass | TileType::Hill | TileType::Sand | TileType::Snow, _, Content::Rock(_)) => {
                    play_sound_rock_in_g_h_s_s()
                }
                | (TileType::ShallowWater, _, Content::Rock(_)) => {
                    play_sound_rock_in_water()
                }
                | (TileType::DeepWater, _, Content::Rock(_)) => {
                    play_sound_rock_in_water()
                }
                | (TileType::Lava, _, Content::Rock(_)) => {
                    play_sound_rock_in_lava()
                }
                | (_, Content::None, Content::Rock(_)) => {
                    play_sound_rock_in_g_h_s_s()
                }
                | (TileType::Mountain, _, Content::None) => {
                    play_sound_empty_in_mountain()
                }
                | (_, Content::Fire, Content::Water(_)) => {
                    play_sound_water_in_fire()
                }
                | (_, Content::Fire, _) => {
                    play_sound_water_in_fire()
                }
                | (_, Content::None, _) => {

                }
                | (_, a, b) => {

                }
            }
        }
        Err(_) => {}
    }

    result

}

// DeepWater,
// ShallowWater,
// Sand,
// Grass,
// Street,
// Hill,
// Mountain,
// Snow,
// Lava,
// Teleport(bool),
// Wall,

pub fn go_with_sound(robot: &mut impl Runnable, world: &mut World, direction: Direction) -> Result<TileMatrix, LibError>{
    let end_tile = give_tile(robot, world, direction.clone());
    let view = robot_view(robot, world);
    let start_tile = &view[1][1];
    let result = go(robot, world, direction.clone());
    match result.clone() {
        Ok(_) => {
            match (&start_tile, &end_tile){
                (Some(a), Some(b)) => {
                    match &a.tile_type {
                        TileType::DeepWater | TileType::ShallowWater => {play_sound_walking_water()}
                        _ => {
                            match (a.tile_type, b.tile_type) {
                                // (TileType::Sand, TileType::DeepWater | TileType::ShallowWater) => {}
                                // (TileType::Grass, TileType::DeepWater | TileType::ShallowWater) => {play_sound_walking_grass()}
                                // (TileType::Street, TileType::DeepWater | TileType::ShallowWater) => {play_sound_walking_street()}
                                // (TileType::Hill, TileType::DeepWater | TileType::ShallowWater) => {play_sound_walking_hill()}
                                // (TileType::Mountain, TileType::DeepWater | TileType::ShallowWater) => {play_sound_walking_mountain()}
                                // (TileType::Snow, TileType::DeepWater | TileType::ShallowWater) => {}
                                // (TileType::Teleport(_), TileType::DeepWater | TileType::ShallowWater) => {}
                                (TileType::Sand, _) => {play_sound_walking_sand()}
                                (TileType::Grass, _) => {play_sound_walking_grass()}
                                (TileType::Street, _) => {play_sound_walking_street()}
                                (TileType::Hill, _) => {play_sound_walking_hill()}
                                (TileType::Mountain, _) => {play_sound_walking_mountain()}
                                (TileType::Snow, _) => {play_sound_walking_snow()}
                                (TileType::Teleport(_), _) => {play_sound_walking_teleport()}
                                (_, _) => {}
                            }
                        }
                    }
                }
                (_, _) => {}
            }
        }
        Err(_) => {}
    }
    result
}

pub fn destroy_with_sound(robot: &mut impl Runnable, world: &mut World, direction: Direction) -> Result<usize, LibError>{
    let end_tile = give_tile(robot, world, direction.clone());
    let result = destroy(robot, world, direction.clone());
    match result.clone() {
        Ok(_) => {
            match end_tile.clone() {
                None => {}
                Some(a) => {
                    match a.content {
                        Content::Rock(_) => {play_sound_mining_rock()}
                        Content::Tree(_) => {play_sound_chopping_wood()}
                        Content::Garbage(_) => {play_sound_destroying_garbage()}
                        Content::Fire => {play_sound_lighting_off_fire()}
                        Content::Coin(_) => {play_sound_collecting_coin()}
                        Content::Water(_) => {play_sound_collecting_water()}
                        Content::Fish(_) => {play_sound_picking_fish()}
                        Content::Bush(_) => {play_sound_chopping_bush()}
                        Content::JollyBlock(_) => {play_sound_destroying_jollyblock()}
                        _ => {}
                    }
                }
            }
        }
        Err(_) => {}
    }
    result
}

pub fn craft_with_sound(robot: &mut impl Runnable, content: Content) -> Result<Content, LibError> {
    let result = craft(robot, content);
    match result.clone() {
        Ok(_) => {play_sound_crafting()}
        Err(_) => {}
    }
    result
}

pub fn teleport_with_sound(
    robot: &mut impl Runnable,
    world: &mut World,
    coordinates: (usize, usize),
) -> Result<TileMatrix, LibError> {
    let result = teleport(robot, world, coordinates);
    match result.clone() {
        Ok(_) => {play_sound_teleport()}
        Err(_) => {}
    }
    result
}