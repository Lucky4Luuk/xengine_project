use std::collections::HashMap;

pub struct ResourceStorage<T>
{
    map: HashMap<usize, T>,
    pub empty_id: usize,
}

impl<T> ResourceStorage<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            empty_id: 0
        }
    }

    fn next_empty_id(&mut self) {
        let mut found = false;
        let mut id = self.empty_id;
        while !found {
            if self.map.get(&id).is_none() {
                debug!("{}", id);
                self.empty_id = id;
                found = true;
            }
            id += 1;
        }
    }

    pub fn push(&mut self, resource: T) -> usize {
        self.map.insert(self.empty_id, resource);
        let id = self.empty_id;
        debug!("Finding next free id...");
        self.next_empty_id();
        debug!("Done!");
        return id;
    }

    pub fn insert(&mut self, id: usize, resource: T) {
        self.map.insert(id, resource);
        if id == self.empty_id {
            self.next_empty_id();
        }
    }

    pub fn set(&mut self, id: usize, resource: T) {
        match self.map.get(&id) {
            Some(x) => {
                *self.map.get_mut(&id).unwrap() = resource;
            },
            None => {
                self.map.insert(id, resource);
            }
        }
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.map.get(&id)
    }

    pub fn remove(&mut self, id: usize) -> Result<(), &'static str> {
        match self.map.get(&id) {
            Some(x) => {
                self.map.remove(&id);
                if id < self.empty_id {
                    self.empty_id = id;
                }
                return Ok(());
            },
            None => return Err("No mesh found at that ID!")
        }
    }
}
