In order to play custom sounds, we implemented a struct and its methods:
```rust  
pub struct CustomSound {  
    path: String,  
    start_tx: mpsc::Sender<()>,  
    stop_tx: mpsc::Sender<()>,  
}
impl CustomSound {  
    pub fn new(path: String, looping: bool, volume: f32) -> CustomSound;
    pub fn play(&self);
	pub fn stop(&self);
}
```

if you want to play a custom sound you need to declare a variable of type CustomSound, and then use the method play(). if you want to stop the sound you need to use the method .stop().