use std::{collections::{HashMap, HashSet}, fs, io, path::Path, time::Duration};
use audiotags::Tag;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Song {
    pub genre: String,
    pub artist: String,
    pub album: Option<String>,
    pub duration: Duration,

    pub path: String,
}

impl Song {
    pub fn from_path<P: AsRef<Path>>(p: P) -> audiotags::Result<Self> {
        let meta = Tag::new().read_from_path(&p)?;
        
        Ok(Self {
            genre: meta.genre().unwrap_or("").to_owned(),
            artist: meta.artist().unwrap_or("").to_owned(),
            album: meta.album().map(|a| a.title.to_owned()),
            duration: Duration::from_secs_f64(meta.duration().unwrap_or(0.0)),
            path: p.as_ref().to_str().unwrap().to_owned()
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
    pub universal_set: Playset,
    pub sets: Vec<Playset>
}
impl Library {
    pub fn initialize<P: AsRef<Path>>(universal_set: P, subsets: P) -> io::Result<Self> {
        let universal_dir = fs::read_dir(universal_set)?;
        let subset_dir = fs::read_dir(subsets)?;

        let universal_set = universal_dir
            .map(|f| f.unwrap().file_name())
            .map(|f| Song::from_path(f).unwrap())
            .collect::<HashSet<Song>>();
        let universal_set = Playset {
            name: "U".to_owned(),
            set: Songset::Primitive(universal_set)
        };

        Ok(Self {
            universal_set,
            sets: vec![],
        })
    }
}


