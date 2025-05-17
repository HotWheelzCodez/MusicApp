use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use youtube_dl::{YoutubeDl, YoutubeDlOutput, SingleVideo};
use std::process;

pub fn play_music(file_path: &str, stream_handle: &Option<OutputStreamHandle>, sink: &Sink) {
    let file = BufReader::new(File::open(file_path).unwrap());
    let source = Decoder::new(file).unwrap();

    sink.append(source);
}

pub fn get_youtube_music(url: &str) {
    let output = YoutubeDl::new(url).socket_timeout("15").extract_audio(true).format("mp3").output_template("/song_library/U/%(title)s.%(ext)s").run();
}
