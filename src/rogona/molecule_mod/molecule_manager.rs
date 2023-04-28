/* MOLECULE MANAGER */

// TODO [Future Work] Every Modulation should be able to be changed to an modulation trait obj

use crate::rogona::{
    molecule_mod::molecule::Molecule,
    rg_attributes::pgtraits::{KernelCompNames, PGComponent, PGObj, RManager}, // ~~ used in later versions with more abstraction
};
use std::{collections::{hash_map::ValuesMut, HashMap}};

use std::fmt;

#[derive(Debug)]
pub struct MoleculeManager {
    mol_arr: HashMap<usize, Box<Molecule>>, // find Molecules by ID - store as single copy on the heap
    total_counter: usize,                   // determines Molecule ID
    mol_to_add_arr: MoleculeArray,          // Vec with Box<Molecule>
    mol_to_des_arr: Vec<usize>,             // Vec with the id's ! currently not in use
}

impl PGComponent for MoleculeManager {
    type Comp = Molecule;

    fn get_name(&self) -> KernelCompNames {
        KernelCompNames::MoleculeManager
    }
}

impl fmt::Display for MoleculeManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = String::from("Molecule Manager is:\n");
        for (id, mol) in self.mol_arr.iter() {
            printer = printer + format!("{}: {}\n", id, mol).as_str();
        }

        write!(f, "{}", printer)
    }
}

/* impl RManager for MoleculeManager {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_name(&self) -> KernelCompNames {
        KernelCompNames::MoleculeManager
    }

    fn apply_changes_t(&mut self, mut add_arr: Vec<Box<dyn PGObj>>) {
        loop {
            match add_arr.pop() {
                Some(mut mol) => {
                    let mut mol: &mut Box<Molecule> = match mol.as_any().downcast_mut::<Box<Molecule>>() {
                        Some(m) => m,
                        None => panic!("Not a molecule"),
                    };
                    mol.set_id(self.total_counter);
                    match self.mol_arr.insert(self.total_counter, mol) {
                        Some(m) => eprintln!("A molecule with this id already exists! \nCheck for errors - Old: {}; New had same id", m),   //only makes sense with manual id-setting
                        None => ()
                    }
                    self.total_counter += 1; //move up if you want to start numbering the mols at 1
                }
                None => break,
            }
        }
    }
} */

impl MoleculeManager {
    ///Constructor
    pub fn new(approx_spawned: usize) -> MoleculeManager {
        MoleculeManager {
            mol_arr: HashMap::new(),
            total_counter: 0,
            mol_to_add_arr: MoleculeArray::with_capacity(approx_spawned), //choose reasonable size so it doesn't have to be reallocated later on
            mol_to_des_arr: Vec::new(),
        }
    }

    // not used in air-based approach but necessary for legacy functionalities
    ///add a molecule to the to-add-list without setting the id; no immediate change
    pub fn add_molecule(&mut self, mol: Molecule) {
        let new_mol: Box<Molecule> = Box::new(mol);
        self.mol_to_add_arr.push(new_mol);
    }

    /// Get one Pointer to a Molecule by ID
    pub fn get_molecule(&mut self, id: usize) -> Option<&mut Box<Molecule>> {
        self.mol_arr.get_mut(&id)
    }

    /// Get Iterator of Moleculepointers
    pub fn get_all_molecules(&mut self) -> ValuesMut<'_, usize, Box<Molecule>> {
        //returns consumable Iterator
        self.mol_arr.values_mut()
    }

    pub fn apply_changes(&mut self) {
        //! destructing via apply_changes_sk()
        //destructing first for memory efficiency
        loop {
            match self.mol_to_des_arr.pop() {
                Some(key_id) => {
                    self.mol_arr.remove(&key_id);
                }
                None => break,
            }
        }

        //add molecules from queue while giving them a new id
        // ? should manual id-setting be possible?
        loop {
            match self.mol_to_add_arr.pop() {
                Some(mut mol) => {
                    mol.set_id(self.total_counter);
                    // remove match statement if manual id-setting is impossible
                    match self.mol_arr.insert(self.total_counter, mol) {
                        Some(m) => eprintln!("A molecule with this id already exists! \nCheck for errors - Old: {}; New had same id", m),   //only makes sense with manual id-setting
                        None => ()
                    }
                    self.total_counter += 1; //move up if you want to start numbering the mols at 1
                }
                None => break,
            }
        }
    }

    pub fn add_molecules_sk(&mut self, mut add_arr: Vec<Box<Molecule>>) {
        loop {
            match add_arr.pop() {
                Some(mut mol) => {
                    mol.set_id(self.total_counter);
                    match self.mol_arr.insert(self.total_counter, mol) {
                        Some(m) => eprintln!("A molecule with this id already exists! \nCheck for errors - Old: {}; New had same id", m),   //only makes sense with manual id-setting
                        None => ()
                    }
                    self.total_counter += 1; //move up if you want to start numbering the mols at 1
                }
                None => break,
            }
        }
    }

    /// used by simulation_kernel to delete molecules
    // @code as param: mut add_arr: Vec<Box<Molecule>>
    pub fn apply_changes_sk(&mut self, mut del_arr: Vec<usize>) {
        //deleting first for memory efficiency
        loop {
            match del_arr.pop() {
                Some(key_id) => {
                    self.mol_arr.remove(&key_id);
                }
                None => break,
            }
        }
    }
}

//data type for better readability. Uses the same functions as Vec
#[derive(Debug, PartialEq)]
struct MoleculeArray {
    list: Vec<Box<Molecule>>,
}

impl MoleculeArray {
    fn new() -> MoleculeArray {
        MoleculeArray { list: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> MoleculeArray {
        MoleculeArray {
            list: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Box<Molecule>) {
        self.list.push(value);
    }

    fn pop(&mut self) -> Option<Box<Molecule>> {
        self.list.pop()
    }

    fn len(&self) -> usize {
        self.list.len()
    }
}






#[cfg(test)]
mod tests {

    use nalgebra::{Point3, Vector3};

    use super::*;

    fn build_molecule(
        id: usize,
        pos: Point3<f64>,
        vel: Vector3<f64>,
        obj: Option<u64>,
    ) -> Molecule {
        let mut mol = Molecule::new(pos, vel, obj);
        mol.set_id(id);
        mol
    }

    #[test]
    fn add_mol() {
        let mut manager = MoleculeManager::new(3);
        let mol = build_molecule(
            1,
            Point3::new(1.0f64, 2.0, 3.0),
            Vector3::new(2.0f64, 4.0, 6.0),
            Some(1),
        ); //id-Setting irrelevant here (will be 0 here)
        manager.add_molecule(mol);
        assert_eq!(manager.mol_arr.len(), 0);
        assert_eq!(manager.mol_to_add_arr.len(), 1);

        manager.apply_changes();
        assert_eq!(manager.mol_arr.len(), 1);
        assert_eq!(manager.mol_to_add_arr.len(), 0);
    }

    #[test]
    fn id_overwrite() {
        let mut manager = MoleculeManager::new(1);
        let mol = build_molecule(
            3456,
            Point3::new(1.0f64, 2.0, 3.0),
            Vector3::new(2.0f64, 4.0, 6.0),
            Some(1),
        ); //id-setting should be zero after adding
        manager.add_molecule(mol);
        manager.apply_changes();
        assert_eq!(manager.get_molecule(3456), None);
        assert_ne!(manager.get_molecule(0), None)
    }

    #[test]
    fn destroy_mol() {
        let mut manager = MoleculeManager::new(2);
        let mol = build_molecule(
            1,
            Point3::new(1.0f64, 2.0, 3.0),
            Vector3::new(2.0f64, 4.0, 6.0),
            Some(1),
        ); //id-Setting irrelevant here
        let mol2 = build_molecule(
            2,
            Point3::new(2.0f64, 3.0, 4.0),
            Vector3::new(4.0f64, 6.0, 8.0),
            Some(1),
        );
        manager.add_molecule(mol);
        manager.add_molecule(mol2);
        manager.apply_changes();
        assert_eq!(manager.mol_arr.len(), 2);
        let del_arr = vec![0];
        println!("Before applying changes \n{:?}", manager.mol_arr);
        manager.apply_changes_sk(del_arr);
        println!("After applying changes \n{:?}", manager.mol_arr);
        assert_eq!(manager.mol_arr.len(), 1);
    }
}
