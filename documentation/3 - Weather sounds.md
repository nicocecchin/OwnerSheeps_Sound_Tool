We added the possibility to play a backgound sound based on the weather

in order to do it, in the method 
```rust  
fn process_tick(&mut self, world: &mut World);
```
you need to put the following function:
```rust  
fn process_tick(&mut self, world: &mut World){
	//your code
	weather_sound(world);
}
```
