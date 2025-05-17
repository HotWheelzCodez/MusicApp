use core::panic;
use std::{collections::{HashMap, HashSet}, fs, io, path::Path, time::Duration};
use audiotags::Tag;

use super::pset_format;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Song {
    pub name: String,
    pub genre: String,
    pub artist: String,
    pub album: String,
    pub duration: u64,
}

impl Song {
    pub fn from_path<P: AsRef<Path>>(p: P, name: String) -> audiotags::Result<Self> {
        let path = p.as_ref().to_str().unwrap();
        let path = format!("{}/{}", path, name);
        println!("Reading song data from: {}", path);
        let meta = Tag::new().read_from_path(path)?;
        
        Ok(Self {
            name,
            genre: meta.genre().unwrap_or("").to_owned(),
            artist: meta.artist().unwrap_or("").to_owned(),
            album: meta.album().map(|a| a.title).unwrap_or("").to_owned(),
            duration: Duration::from_secs_f64(meta.duration().unwrap_or(0.0)).as_secs(),
        })
    }
}

pub enum SongSet {
    Terminal(HashSet<Song>),
    NonTerminal(String)
}

impl SongSet {
    pub fn flatten(&self, non_term_map: &HashMap<String, Playset>) -> HashSet<Song> {
        match self {
            SongSet::Terminal(set) => set.clone(),
            SongSet::NonTerminal(name) => {
                non_term_map.get(name).unwrap().songs.flatten(non_term_map)
            },
        }
    }
    pub fn to_pset_string(&self) -> String {
        let mut out = String::new();
        match self {
            SongSet::Terminal(set) => {
                out.push(pset_format::SET_START);
                for song in set {
                    out.push_str(&song.name);
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
    pub fn flatten(&self, non_term_map: &HashMap<String, Playset>) -> HashSet<Song> {
        match self {
            SongTree::Operation(op, song_tree_node) => {
                let op = match *op {
                    pset_format::UNION => |l: HashSet<Song>, r| l.union(r).map(|s| s.to_owned()).collect(),
                    pset_format::INTERSECTION => |l: HashSet<Song>, r| l.intersection(r).map(|s| s.to_owned()).collect(),
                    pset_format::DIFFERENCE => |l: HashSet<Song>, r| l.difference(r).map(|s| s.to_owned()).collect(),
                    _ => unreachable!()
                };
                op(song_tree_node.lhs.flatten(non_term_map), &song_tree_node.rhs.flatten(non_term_map))
            },
            SongTree::Set(song_set) => {
                song_set.flatten(non_term_map)
            },
        }
    }

    pub fn to_pset_string(&self) -> String {
        match self {
            SongTree::Operation(op, song_tree_node) => {
                format!(
                    "{}{}{}",
                    song_tree_node.lhs.to_pset_string(),
                    song_tree_node.rhs.to_pset_string(),
                    op
                )
            },
            SongTree::Set(song_set) => {
                song_set.to_pset_string()
            },
        }
    }
    pub fn from_pset_string(s: &str) -> Self {
        let mut parse_stack = vec![];
        let mut set_buffer = HashSet::<Song>::new();
        let mut name_buffer = String::new();

        let mut collecting_set = false;

        for c in s.chars() {
            match c {
                pset_format::SEPERATOR if collecting_set => {
                    set_buffer.insert(Song::from_path("song_library/U/", name_buffer).unwrap());
                    name_buffer = String::new();
                }
                pset_format::SEPERATOR => {
                    parse_stack.push(SongSet::NonTerminal(name_buffer));
                    name_buffer = String::new();
                }

                pset_format::SET_START => {
                    collecting_set = true;
                }
                pset_format::SET_END => {
                    collecting_set = false;
                    parse_stack.push(SongSet::Terminal(set_buffer));
                    set_buffer = HashSet::new();
                }

                pset_format::UNION => {
                    
                }
                pset_format::INTERSECTION => {
                    
                }
                pset_format::DIFFERENCE => {
                    
                }

                c => {
                    name_buffer.push(c);
                }
            }
        }
        todo!()
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

        let out = self.songs.to_pset_string();

        fs::write(output_path, out)?;

        Ok(())
    }

    pub fn empty_terminal(name: String) -> Self {
        Self {
            name,
            songs: SongTree::Set(SongSet::Terminal(HashSet::new())),
        }
    }
    pub fn from_pset_string(s: &str, name: String) -> Self {
        Self {
            name,
            songs: SongTree::from_pset_string(s)
        }
    }
}

pub struct Library {
    pub universal_set: Playset,
    pub sets: HashMap<String, Playset>
}
impl Library {
    pub fn initialize<P: AsRef<Path>>(universal_set: P, subsets: P) -> io::Result<Self> {
        let universal_dir = fs::read_dir(universal_set)?;
        let subset_dir = fs::read_dir(subsets)?;

        let universal_set = universal_dir
            .map(|f| f.unwrap().path())
            .map(|p| {
                let dir = p.parent().unwrap();
                let f_name = p.file_name().unwrap().to_str().unwrap();
                Song::from_path(dir, f_name.to_owned()).unwrap()
            }).collect::<HashSet<Song>>();
        let universal_set = Playset {
            name: "U".to_owned(),
            songs: SongTree::Set(SongSet::Terminal(universal_set))
        };
        
        for file in subset_dir
            .filter_map(|f| f.ok())
            .filter(|f| f.file_type().unwrap().is_file())
            .map(|f| fs::read_to_string(f.path()).unwrap())
        {
            
        }

        Ok(Self {
            universal_set,
            sets: HashMap::new()
        })
    }

    pub fn push_empty_set(&mut self, name: String) {
        self.sets.insert(name.clone(), Playset::empty_terminal(name));
    }
}


