use std::ffi::c_uchar;
use std::fs::File;
use std::ops::Deref;
use std::sync::{Arc, mpsc, MutexGuard};
use std::sync::mpsc::{Receiver, RecvError};
use std::thread;
use std::time::Duration;
use lazy_static::lazy_static;
use robotics_lib::interface::look_at_sky;
use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
use robotics_lib::world::World;
use rodio::{OutputStream, Sink};
use std::sync::Mutex;


lazy_static! {
    static ref GLOBAL_VARIABLE: Mutex<Option<WeatherType>> = Mutex::new(None);
}

fn play_weather_sound(weather_type: WeatherType) -> mpsc::Sender<()> {
    let (tx, rx) = mpsc::channel();
    let mut volume : f32 = 1.0;

    thread::spawn(move || {
        let mut path: String = "".to_string();
        match weather_type {
            WeatherType::Sunny => {path = "src/sounds/weather/sunny_sound.mp3".to_string(); volume = 0.05}
            WeatherType::Rainy => {path = "src/sounds/weather/rainy_sound.mp3".to_string(); volume = 0.5}
            WeatherType::Foggy => {path = "src/sounds/weather/foggy_sound.mp3".to_string(); volume = 0.01}
            WeatherType::TropicalMonsoon => {path = "src/sounds/weather/tropical_monsoon_sound.mp3".to_string(); volume = 0.01}
            WeatherType::TrentinoSnow => {path = "src/sounds/weather/trentino_snow_sound.mp3".to_string(); volume = 0.01}
        }
        println!("{}", path);

        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to create the sink");
        let file = File::open(&path).expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.set_volume(volume);

        // Play the sound in a loop until the stop signal is received
        while rx.try_recv().is_err() {
            if sink.empty() {
                // If the sink is empty, append the audio file again to restart playback
                let file = File::open(&path).expect("Impossible to open audio file");
                sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
            }

            // Sleep for a short duration to avoid high CPU usage
            thread::sleep(Duration::from_millis(100));
        }

        sink.stop(); // Stop the playback
        sink.detach(); // Detach the sink
    });

    tx
}

pub fn weather_sound_init() {
    thread::spawn(move || {
        let mut previous_weather = GLOBAL_VARIABLE.lock().unwrap().clone().unwrap();
        let mut change_signal = play_weather_sound(previous_weather);

        loop{
            let weather= GLOBAL_VARIABLE.lock().unwrap().clone().unwrap();
            //println!("{:?}", weather);
            if weather != previous_weather{
                previous_weather = weather;
                println!("{:?},{:?}", previous_weather, change_signal);
                change_signal.send(()).expect("impossible to send");
                change_signal = play_weather_sound(previous_weather)
            }

            thread::sleep(Duration::from_millis(100))
        }
    });
}


pub fn weather_sound(world: &World){
    let weather = look_at_sky(world).get_weather_condition();
    let mut global_var = GLOBAL_VARIABLE.lock().unwrap();
    match global_var.deref() {
        None => {
            let _ = global_var.insert(weather);
            weather_sound_init()
        }
        Some(_) => {let _ = global_var.insert(weather);}
    }
}