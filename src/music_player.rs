use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;

pub fn play_music(file_path: &str, stream_handle: &Option<OutputStreamHandle>, sink: &Sink) {
    let file = BufReader::new(File::open(file_path).unwrap());
    let source = Decoder::new(file).unwrap();

    sink.append(source);
}
