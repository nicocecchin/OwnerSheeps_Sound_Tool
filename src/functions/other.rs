use std::fs::File;
use std::thread;
use rodio::{OutputStream, Sink};

pub fn play_sound_has_not_energy() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/energy/has_not_energy.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_crafting() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/craft/crafting_sound.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_teleport() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/teleport/teleport.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}