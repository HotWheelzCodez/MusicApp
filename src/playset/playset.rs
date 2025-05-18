use core::panic;
use std::{cell::RefCell, collections::{HashMap, HashSet}, fs, io, path::Path, rc::Rc, time::Duration};
use audiotags::Tag;

use super::pset_format;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
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

#[derive(Debug, Clone)]
pub enum SongSet {
    Terminal(HashSet<Song>),
    NonTerminal(String)
}

impl SongSet {
    pub fn flatten(&self, non_term_map: &HashMap<String, Playset>) -> HashSet<Song> {
        match self {
            SongSet::Terminal(set) => set.clone(),
            SongSet::NonTerminal(name) => {
                non_term_map.get(name).unwrap().songs.borrow().flatten(non_term_map)
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

#[derive(Debug, Clone)]
pub enum SongTree {
    Operation(char, SongTreeNode),
    Set(SongSet),
}

#[derive(Debug, Clone)]
pub struct SongTreeNode {
    lhs: RefCell<Rc<SongTree>>,
    rhs: RefCell<Rc<SongTree>>,
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
                op(song_tree_node.lhs.borrow().flatten(non_term_map), &song_tree_node.rhs.borrow().flatten(non_term_map))
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
                    song_tree_node.lhs.borrow().to_pset_string(),
                    song_tree_node.rhs.borrow().to_pset_string(),
                    op
                )
            },
            SongTree::Set(song_set) => {
                song_set.to_pset_string()
            },
        }
    }
    pub fn from_pset_string(s: &str) -> Self {
        println!("from_pset:\n{}", s);
        let mut parse_stack: Vec<SongTree> = vec![];
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
                    parse_stack.push(SongTree::Set(SongSet::NonTerminal(name_buffer)));
                    name_buffer = String::new();
                }

                pset_format::SET_START => {
                    collecting_set = true;
                }
                pset_format::SET_END => {
                    collecting_set = false;
                    parse_stack.push(SongTree::Set(SongSet::Terminal(set_buffer)));
                    set_buffer = HashSet::new();
                }

                pset_format::UNION..=pset_format::DIFFERENCE => {
                    let right = parse_stack.pop().unwrap();
                    let left = parse_stack.pop().unwrap();
                    parse_stack.push(SongTree::Operation(c, SongTreeNode {
                        lhs: RefCell::new(Rc::new(left)),
                        rhs: RefCell::new(Rc::new(right))
                    }));
                }

                c => {
                    name_buffer.push(c);
                }
            }
        }
        println!("p_stack:\n{:#?}", parse_stack);
        println!("nb:\n{}", name_buffer);
        println!("cs:\n{}", collecting_set);
        parse_stack.pop().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Playset {
    pub name: String,
    pub songs: RefCell<Rc<SongTree>>,
}
impl Playset {
    /// just put "song_library/" as the param
    pub fn write_to_file<P: AsRef<Path>>(&self, song_library: P) -> io::Result<()> {
        let mut output_path = song_library.as_ref().to_str().unwrap().to_owned();
        output_path.push_str("subsets/");
        output_path.push_str(&self.name);

        let out = self.songs.borrow().to_pset_string();

        fs::write(output_path, out)?;

        Ok(())
    }

    pub fn empty_terminal(name: String) -> Self {
        Self {
            name,
            songs: RefCell::new(Rc::new(SongTree::Set(SongSet::Terminal(HashSet::new())))),
        }
    }
    pub fn from_pset_string(s: &str, name: String) -> Self {
        Self {
            name,
            songs: RefCell::new(Rc::new(SongTree::from_pset_string(s)))
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
            songs: RefCell::new(Rc::new(SongTree::Set(SongSet::Terminal(universal_set))))
        };

        let mut sets = HashMap::new();
        
        for (name, file) in subset_dir
            .filter_map(|f| f.ok())
            .filter(|f| f.file_type().unwrap().is_file())
            .map(|f| (f.file_name().into_string().unwrap(), fs::read_to_string(f.path()).unwrap()))
        {
            sets.insert(name.clone(), Playset::from_pset_string(&file, name));
        }

        println!("Sets:\n{:#?}", sets);

        Ok(Self {
            universal_set,
            sets
        })
    }

    pub fn push_empty_set(&mut self, name: String) {
        self.sets.insert(name.clone(), Playset::empty_terminal(name));
    }
}


