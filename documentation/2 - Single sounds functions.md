
All the sounds that we added have their own public functions, so you can use them also outside of the other function where they are used.
The functions are:

1. walking sounds:

```rust  
pub fn play_sound_walking_grass();
  
pub fn play_sound_walking_sand();
  
pub fn play_sound_walking_water();
  
pub fn play_sound_entering_water();
  
pub fn play_sound_walking_street(); 
  
pub fn play_sound_walking_hill();
  
pub fn play_sound_walking_mountain();
  
pub fn play_sound_walking_snow();
  
pub fn play_sound_walking_teleport();
```

2. destroying sounds:

```rust
pub fn play_sound_mining_rock();
  
pub fn play_sound_chopping_wood();  
  
pub fn play_sound_destroying_garbage();
  
pub fn play_sound_lighting_off_fire();
  
pub fn play_sound_collecting_coin();
  
pub fn play_sound_picking_crate();  
  
pub fn play_sound_collecting_water();
  
pub fn play_sound_picking_fish(); 
  
pub fn play_sound_chopping_bush();
  
pub fn play_sound_destroying_jollyblock();
```

3. put sounds:

```rust
pub fn play_sound_coin_in_bank();
  
pub fn play_sound_garbage_in_bin();
  
pub fn play_sound_tree_in_crate();
  
pub fn play_sound_fire_in_tree_empty();
  
pub fn play_sound_water_in_fire();
  
pub fn play_sound_rock_in_g_h_s_s();
  
pub fn play_sound_rock_in_water();
  
pub fn play_sound_rock_in_lava();
  
pub fn play_sound_empty_in_mountain(); 
  
pub fn play_sound_content_in_market();
  
pub fn play_sound_content_in_tile();
```

4. other sounds:

```rust
pub fn play_sound_has_not_energy();
  
pub fn play_sound_crafting();
  
pub fn play_sound_teleport();
```