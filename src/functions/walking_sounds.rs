use std::fs::File;
use std::thread;
use rodio::{OutputStream, Sink};

pub fn play_sound_walking_grass() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/walking/walking_grass.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_walking_sand() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/walking/walking_sand.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_walking_water() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/walking/walking_water.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_entering_water() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/walking/entering_water.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_walking_street() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/walking/walking_asphalt.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_walking_hill() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/walking/walking_hill.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_walking_mountain() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/walking/walking_mountain.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_walking_snow() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/walking/walking_snow.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_walking_teleport() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/walking/walking_teleport.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}