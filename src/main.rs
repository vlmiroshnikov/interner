use std::{collections::HashMap, mem};

pub struct Interner {
    map: HashMap<&'static str, StrId>,
    vec: Vec<&'static str>,
    buf: String,
    full: Vec<String>,
}

#[derive(Debug)]
pub struct Stats {
    pub uniq_count: usize,
    pub total_size: usize,
}

impl Interner {
    pub fn stats(self) -> Stats {
        Stats {
            uniq_count: self.map.len(),
            total_size: self.full.iter().fold(0, |acc, it| acc + it.len()),
        }
    }

    pub fn with_capacity(cap: usize) -> Interner {
        let cap = cap.next_power_of_two();
        Interner {
            map: HashMap::default(),
            vec: Vec::new(),
            buf: String::with_capacity(cap),
            full: Vec::new(),
        }
    }

    pub fn intern(&mut self, name: &str) -> StrId {
        if let Some(&id) = self.map.get(name) {
            return id;
        }
        let name = unsafe { self.alloc(name) };
        let id = StrId(self.map.len() as u32);
        self.map.insert(name, id);
        self.vec.push(name);

        debug_assert!(self.lookup(id) == name);
        debug_assert!(self.intern(name) == id);

        id
    }

    pub fn lookup(&self, id: StrId) -> &str {
        self.vec[id.0 as usize]
    }

    unsafe fn alloc(&mut self, name: &str) -> &'static str {
        let cap = self.buf.capacity();
        if cap < self.buf.len() + name.len() {
            let new_cap = (cap.max(name.len()) + 1).next_power_of_two();
            let new_buf = String::with_capacity(new_cap);
            let old_buf = mem::replace(&mut self.buf, new_buf);
            self.full.push(old_buf);
        }

        let interned = {
            let start = self.buf.len();
            self.buf.push_str(name);
            &self.buf[start..]
        };

        &*(interned as *const str)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct StrId(u32);

fn main() {
    let mut interner = Interner::with_capacity(10);
    for i in 0..100000 {
        interner.intern(format!("string_{}", i % 1000).as_str());
    }

    println!("{:?}", interner.stats())
}
