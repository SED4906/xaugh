use std::{collections::BTreeMap, sync::Mutex};

pub static ATOMS: Mutex<BTreeMap<u32,String>> = Mutex::new(BTreeMap::new());
pub static NEXT_ATOM: Mutex<u32> = Mutex::new(128);

pub fn init_atoms() {
    let mut atoms = ATOMS.lock().unwrap();
    atoms.insert(23, "RESOURCE_MANAGER".to_string());
}

pub fn register_atom(atom: String) {
    let mut next_atom = NEXT_ATOM.lock().unwrap();
    let mut atoms = ATOMS.lock().unwrap();
    atoms.insert(*next_atom, atom);
    *next_atom += 1;
}

pub fn get_atom(only_if_exists: bool, name: String) -> u32 {
    let mut atoms = ATOMS.lock().unwrap();
    match atoms.iter().filter(|(_, v)| **v == name).next() {
        Some((k, _)) => *k,
        None if only_if_exists => 0,
        _ => {
            let mut next_atom = NEXT_ATOM.lock().unwrap();
            atoms.insert(*next_atom, name);
            *next_atom += 1;
            *next_atom - 1
        }
    }
}