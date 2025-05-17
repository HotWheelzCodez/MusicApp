use std::{collections::HashSet, io, path::Path, time::Duration};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Song {
    pub genre: String,
    pub artist: String,
    pub album: String,
    pub duration: Duration,
}

impl Song {
    pub fn from_path<P: AsRef<Path>>(p: P) -> io::Result<Self> {
        let meta = std::fs::metadata(p)?;
        
        Ok(Self {
            genre: todo!(),
            artist: todo!(),
            album: todo!(),
            duration: todo!(),
        })
    }
}


#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Transformation {
    Union(String),
    Intersection(String)
}


#[derive(Serialize, Deserialize)]
pub enum Songset {
    Primitive(HashSet<Song>),
    Algrebaic((HashSet<Song>, Vec<Transformation>))
}


#[derive(Serialize, Deserialize)]
pub struct Playset {
    pub name: String,
    pub set: Songset
}


#[derive(Serialize, Deserialize)]
pub struct Library {
    pub sets: Vec<Playset>
}

