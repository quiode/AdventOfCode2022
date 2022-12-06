use std::{
    io::{ BufReader, Lines as OtherLines },
    fs::File,
    collections::{ VecDeque, HashSet },
    hash::Hash,
};

pub type Lines = OtherLines<BufReader<File>>;

pub struct DupVecDeque<T>(VecDeque<T>) where T: Eq + Hash;

impl<T> DupVecDeque<T> where T: Eq + Hash {
    pub fn new() -> Self {
        Self {
            0: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            0: VecDeque::with_capacity(capacity),
        }
    }

    pub fn has_duplicates(&self) -> bool {
        let mut set = HashSet::new();
        for item in &self.0 {
            set.insert(item);
        }

        set.len() == self.0.len()
    }

    pub fn vec(&self) -> &VecDeque<T> {
        &self.0
    }

    pub fn vec_mut(&mut self) -> &mut VecDeque<T> {
        &mut self.0
    }
}