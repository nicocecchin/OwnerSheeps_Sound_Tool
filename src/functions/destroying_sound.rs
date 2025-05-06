use std::fs::File;
use std::thread;
use rodio::{OutputStream, Sink};

pub fn play_sound_mining_rock() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/destroying/mining_rock.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_chopping_wood() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/destroying/chopping_wood.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_destroying_garbage() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/destroying/throwing_garbage.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_lighting_off_fire() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/destroying/fire_extinguishing.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_collecting_coin() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/destroying/collecting_coin.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_picking_crate() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/destroying/picking_crate.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_collecting_water() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/destroying/collecting_water.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_picking_fish() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/destroying/picking_fish.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_chopping_bush() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/destroying/chopping_bush.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_destroying_jollyblock() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/destroying/destroying_jollyblock.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}
