use std::{collections::{HashMap, HashSet}, fs, io, path::Path, time::Duration};
use audiotags::Tag;

use super::pset_format;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Song {
    pub name: String,
    pub genre: String,
    pub artist: String,
    pub album: Option<String>,
    pub duration: Duration,

    pub path: String,
}

impl Song {
    pub fn from_path<P: AsRef<Path>>(p: P, name: String) -> audiotags::Result<Self> {
        let meta = Tag::new().read_from_path(&p)?;
        
        Ok(Self {
            name,
            genre: meta.genre().unwrap_or("").to_owned(),
            artist: meta.artist().unwrap_or("").to_owned(),
            album: meta.album().map(|a| a.title.to_owned()),
            duration: Duration::from_secs_f64(meta.duration().unwrap_or(0.0)),
            path: p.as_ref().to_str().unwrap().to_owned()
        })
    }
}

pub struct TerminalSongset {
    
}

pub struct NonTerminalSongset {
    
}

pub enum SongSet {
    Terminal(HashSet<Song>),
    NonTerminal(String)
}

impl SongSet {
    pub fn flatten(&self) -> HashSet<Song> {
        match self {
            SongSet::Terminal(set) => set.clone(),
            SongSet::NonTerminal(name) => {
                todo!()
            },
        }
    }
    pub fn to_pset_string(&self) -> String {
        let mut out = String::new();
        match self {
            SongSet::Terminal(set) => {
                out.push(pset_format::SET_START);
                for song in set {
                    out.push_str(&song.path);
                    out.push(pset_format::SEPERATOR)
                }
                out.pop();
                out.push(pset_format::SET_END);
            },
            SongSet::NonTerminal(name) => {
                out = name.clone();
                out.push(pset_format::SEPERATOR);
            },
        }
        out
    }
    pub fn from_pset_string(s: String) -> Self {
        for c in s.chars() {
            
        }

        todo!()
    }
}

pub enum SongTree {
    Operation(char, SongTreeNode),
    Set(SongSet),
}

pub struct SongTreeNode {
    lhs: Box<SongTree>,
    rhs: Box<SongTree>,
}

impl SongTree {
    pub fn to_pset_string(&self) -> String {
        match self {
            SongTree::Operation(op, song_tree_node) => {
                todo!()
            },
            SongTree::Set(song_set) => {
                song_set.to_pset_string()
            },
        }
    }
}

pub struct Playset {
    pub name: String,
    pub songs: SongTree,
}
impl Playset {
    pub fn write_to_file<P: AsRef<Path>>(&self, song_library: P) -> io::Result<()> {
        let mut output_path = song_library.as_ref().to_str().unwrap().to_owned();
        output_path.push_str(&self.name);

        let mut out = self.songs.to_pset_string();

        fs::write(output_path, out)?;

        Ok(())
    }

    pub fn empty_terminal(name: String) -> Self {
        Self {
            name,
            set: SongSet::Terminal(HashSet::new())
        }
    }
}

pub struct Library {
    pub universal_set: Playset,
    pub sets: Vec<Playset>
}
impl Library {
    pub fn initialize<P: AsRef<Path>>(universal_set: P, subsets: P) -> io::Result<Self> {
        let universal_dir = fs::read_dir(universal_set)?;
        let subset_dir = fs::read_dir(subsets)?;

        let universal_set = universal_dir
            .map(|f| f.unwrap().path())
            .map(|p| {
                let p2 = p.clone();
                let f_name = p.file_name().unwrap().to_str().unwrap();
                Song::from_path(p2, f_name.to_owned()).unwrap()
            }).collect::<HashSet<Song>>();
        let universal_set = Playset {
            name: "U".to_owned(),
            set: SongSet::Terminal(universal_set)
        };

        for file in subset_dir
            .filter_map(|f| f.ok())
            .filter(|f| f.file_type().unwrap().is_file())
            .map(|f| fs::read_to_string(f.path()).unwrap())
        {
            
        }

        Ok(Self {
            universal_set,
            sets: vec![],
        })
    }

    pub fn push_empty_set(&mut self, name: String) {
        self.sets.push(Playset::empty_terminal(name));
    }
}


