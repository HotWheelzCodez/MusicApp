use std::{collections::HashSet, time::Duration};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Song {
    pub genre: String,
    pub artist: String,
    pub album: String,
    pub duration: Duration,
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
