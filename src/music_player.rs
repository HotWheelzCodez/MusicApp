use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;

pub fn play_music(file_path: &str, stream_handle: &Option<OutputStreamHandle>) {
    let file = BufReader::new(File::open(file_path).unwrap());
    let source = Decoder::new(file).unwrap();

    let sink = Sink::try_new(stream_handle.as_ref().unwrap()).unwrap();
    sink.append(source);
    sink.detach();
}
