use std::fs::File;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use rodio::{OutputStream, Sink};

#[derive(Debug)]
pub struct CustomSound {
    path: String,
    start_tx: mpsc::Sender<()>,
    stop_tx: mpsc::Sender<()>,
}

impl CustomSound {
    pub fn new(path: String, looping: bool, volume: f32) -> CustomSound {
        let path_ref = path.clone();
        let (start_tx, start_rx) = mpsc::channel();
        let (stop_tx, stop_rx) = mpsc::channel();

        thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().expect("Impossible to open output stream");
            let sink = Sink::try_new(&stream_handle).expect("Impossible to create the sink");

            loop {
                //let sink = Sink::try_new(&stream_handle).expect("Impossible to create the sink");

                match start_rx.try_recv() {
                    Ok(_) => {
                        println!("sent1");
                        // Start playing the sound
                        println!("{}", path_ref);
                        let file = File::open(&path_ref).expect("Impossible to open audio file");
                        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
                        sink.set_volume(volume);

                        loop {
                            match stop_rx.try_recv() {
                                Ok(_) => {
                                    println!("sent2");
                                    // Stop playing the sound
                                    sink.stop();
                                    break;
                                    //sink.detach();
                                }
                                Err(_) => {
                                    if looping && sink.empty() {
                                        // If the sink is empty, append the audio file again to restart playback
                                        let file = File::open(&path_ref).expect("Impossible to open audio file");
                                        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).expect("Impossible to decode audio file"));
                                        sink.set_volume(volume);
                                    }
                                }
                            }
                            thread::sleep(Duration::from_millis(100));
                        }
                        //sink.sleep_until_end();
                    }
                    Err(_) => {}
                }

                // Sleep for a short duration to avoid high CPU usage
                thread::sleep(Duration::from_millis(100));
            }
        });

        CustomSound {
            path,
            start_tx,
            stop_tx,
        }
    }

    pub fn play(&self) {
        self.start_tx.send(()).expect("Failed to send start signal");
    }

    pub fn stop(&self) {
        self.stop_tx.send(()).expect("Failed to send stop signal");
    }
}


