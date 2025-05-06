///This tool allows you to sounds in this project!
///
/// We already added a lot of sounds, but you also have the possibility to add some.
///
/// Also, this tool gives you the possibility to use some of the functions of the WG library but they produce sounds.
///
/// All the sounds that we added have their own public functions, so you can use them also outside of the other function where they are used.
/// The functions are:
///
/// walking sounds
/// ```rust
/// pub fn play_sound_walking_grass();
///
/// pub fn play_sound_walking_sand();
///
/// pub fn play_sound_walking_water();
///
/// pub fn play_sound_entering_water();
///
/// pub fn play_sound_walking_street();
///
/// pub fn play_sound_walking_hill();
///
/// pub fn play_sound_walking_mountain();
///
/// pub fn play_sound_walking_snow();
///
/// pub fn play_sound_walking_teleport();
/// ```

/// destroying sounds
/// ```rust
/// pub fn play_sound_mining_rock();
///
/// pub fn play_sound_chopping_wood();
///
/// pub fn play_sound_destroying_garbage();
///
/// pub fn play_sound_lighting_off_fire();
///
/// pub fn play_sound_collecting_coin();
///
/// pub fn play_sound_picking_crate();
///
/// pub fn play_sound_collecting_water();
///
/// pub fn play_sound_picking_fish();
///
/// pub fn play_sound_chopping_bush();
///
/// pub fn play_sound_destroying_jollyblock();
/// ```
///
/// put sounds
/// ```rust
/// pub fn play_sound_coin_in_bank();
///
/// pub fn play_sound_garbage_in_bin();
///
/// pub fn play_sound_tree_in_crate();
///
/// pub fn play_sound_fire_in_tree_empty();
///
/// pub fn play_sound_water_in_fire();
///
/// pub fn play_sound_rock_in_g_h_s_s();
///
/// pub fn play_sound_rock_in_water();
///
/// pub fn play_sound_rock_in_lava();
///
/// pub fn play_sound_empty_in_mountain();
///
/// pub fn play_sound_content_in_market();
///
/// pub fn play_sound_content_in_tile();
/// ```
///
/// other sounds
/// ```rust
/// pub fn play_sound_has_not_energy();
///
/// pub fn play_sound_crafting();
///
/// pub fn play_sound_teleport();
/// ```


/// We added the possibility to play a backgound sound based on the weather
///
/// in order to do it, in the method
/// ```rust
/// use robotics_lib::world::World;
/// fn process_tick(&mut self, world: &mut World);
/// ```
/// you need to put the following function:
/// ```rust
/// use robotics_lib::world::World;
/// use OwnerSheeps_Sound_Tool::functions::weather_sounds::weather_sound;
/// fn process_tick(&mut self, world: &mut World){
/// 	//your code
/// 	weather_sound(world);
/// }


///We propose some alternatives to the already present functions:
///```rust
/// use robotics_lib::interface::Direction;
/// use robotics_lib::runner::Runnable;use robotics_lib::runner::Runnable;
/// use robotics_lib::utils::LibError;
/// use robotics_lib::world::tile::Content;
/// use robotics_lib::world::World;
/// pub fn put(
///     robot: &mut impl Runnable,
///     world: &mut World,
///     content_in: Content,
///     quantity: usize,
///     direction: Direction,
/// ) -> Result<usize, LibError>;
///
/// pub fn go(
/// 	robot: &mut impl Runnable,
/// 	world: &mut World,
/// 	direction: Direction
/// ) -> Result<TileMatrix, LibError>;
///
/// pub fn destroy(
/// 	robot: &mut impl Runnable,
/// 	world: &mut World,
/// 	direction: Direction
/// ) -> Result<usize, LibError>;
///
/// pub fn craft(
/// 	robot: &mut impl Runnable,
/// 	content: Content
/// ) -> Result<Content, LibError>;
///
/// pub fn teleport(
///     robot: &mut impl Runnable,
///     world: &mut World,
///     coordinates: (usize, usize),
/// ) -> Result<TileMatrix, LibError>;
/// ```
///
/// these alternatives are:
///
///```rust
/// use robotics_lib::interface::Direction;
/// use robotics_lib::runner::Runnable;use robotics_lib::runner::Runnable;
/// use robotics_lib::utils::LibError;
/// use robotics_lib::world::tile::Content;
/// use robotics_lib::world::World;
/// pub fn put_with_sound(
///     robot: &mut impl Runnable,
///     world: &mut World,
///     content_in: Content,
///     quantity: usize,
///     direction: Direction,
/// ) -> Result<usize, LibError>;
///
/// pub fn go_with_sound(
/// 	robot: &mut impl Runnable,
/// 	world: &mut World,
/// 	direction: Direction
/// ) -> Result<TileMatrix, LibError>;
///
/// pub fn destroy_with_sound(
/// 	robot: &mut impl Runnable,
/// 	world: &mut World,
/// 	direction: Direction
/// ) -> Result<usize, LibError>;
///
/// pub fn craft_with_sound(
/// 	robot: &mut impl Runnable,
/// 	content: Content
/// ) -> Result<Content, LibError>;
///
/// pub fn teleport_with_sound(
///     robot: &mut impl Runnable,
///     world: &mut World,
///     coordinates: (usize, usize),
/// ) -> Result<TileMatrix, LibError>;
/// ```
///
/// they work the same as the original one, exept that they (you wouldn't guess) play the sounds.


///In order to play custom sounds, we implemented a struct and its methods:
///```rust
/// use std::sync::mpsc;use std::sync::mpsc;
/// pub struct CustomSound {
///     path: String,
///     start_tx: mpsc::Sender<()>,
///     stop_tx: mpsc::Sender<()>,
/// }
/// impl CustomSound {
///     pub fn new(path: String, looping: bool, volume: f32) -> CustomSound;
///     pub fn play(&self);
/// 	pub fn stop(&self);
/// }
/// ```
///
/// if you want to play a custom sound you need to declare a variable of type CustomSound, and then use the method play(). if you want to stop the sound you need to use the method .stop().


pub mod functions;