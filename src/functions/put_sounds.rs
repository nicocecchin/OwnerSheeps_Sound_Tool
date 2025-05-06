use std::fs::File;
use std::thread;
use rodio::{OutputStream, Sink};

pub fn play_sound_coin_in_bank() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/put/coin_in_bank.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_garbage_in_bin() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/put/garbage_in_bin.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_tree_in_crate() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/put/tree_in_crate.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_fire_in_tree_empty() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/put/fire_in_tree_empty.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_water_in_fire() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/put/water_in_fire.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_rock_in_g_h_s_s() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/put/rock_in_g_h_s_s.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_rock_in_water() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/put/rock_in_water.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_rock_in_lava() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/put/rock_in_lava.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_empty_in_mountain() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/put/empty_in_mountain.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_content_in_market() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/put/content_in_market.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}

pub fn play_sound_content_in_tile() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
        let sink = Sink::try_new(&stream_handle).expect("Impossible to creat the sink");
        let file = File::open("src/sounds/put/content_in_tile.mp3").expect("Impossible to open audio file");

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
        sink.sleep_until_end();
        sink.detach();
    });}
