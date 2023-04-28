use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::{self, Duration};
use std::{fmt, env};
use std::io::Write;

use enum_iterator::IntoEnumIterator;
use log::debug;
use nalgebra::{Point3, Vector3};

use crate::rogona::{
    bitstream_generator_mod::bitstreamgen_manager::BitstreamgenManager,
    modulation_mod::modulation_manager::ModulationManager,
    injector_mod::injector_manager::InjectorManager,
    molecule_mod::{
        molecule_manager::MoleculeManager,
        movement_predictor::MovementPredictor,
        molecule::Molecule,
    },
    object_mod::scene_manager::SceneManager,
    sensor_mod::sensor_manager::SensorManager,
    rg_attributes::{
        pgtraits::{self, KernelCompNames, RManager},
        stages::{InitStages, KernelComp, NotificationStages, SimStages},
    },


};




// ! In an attempt to use trait objects and make it more elegant...
pub struct SimulationKernel2 {
    /* kernel_comps: HashMap<
        pgtraits::KernelCompNames,
        Box<dyn pgtraits::PGComponent<Comp = dyn pgtraits::PGObj>>,
    >, */
    kernel_comps: HashMap<pgtraits::KernelCompNames, Box<dyn RManager>>,
    //time_configs
    sim_time_limit: time::Duration,
    base_delta_time: time::Duration,
}

impl SimulationKernel2 {
    pub fn new(
        sim_time_limit: time::Duration,
        base_delta_time: time::Duration,
    ) -> SimulationKernel2 {
        SimulationKernel2 {
            kernel_comps: HashMap::new(),
            sim_time_limit,
            base_delta_time,
        }
    }

    /* pub fn get_kernel_comp(&mut self, kernel_comp: KernelCompNames) -> Option<&mut Box<dyn pgtraits::PGComponent<Comp = dyn pgtraits::PGObj>>> {
        self.kernel_comps.get_mut(&kernel_comp)
    }

    pub fn apply_changes(&mut self, mut add_arr: Vec<Box<dyn pgtraits::PGComponent<Comp = dyn pgtraits::PGObj>>>) {
        loop{
            match add_arr.pop(){
                Some(mut kernel_comp) => {
                    match self.kernel_comps.insert(kernel_comp.get_name(), kernel_comp) {
                        None => (),
                        Some(comp) => eprintln!("This Kernel Component {:?} should only exist once!", comp.get_name()),
                    }
                }
                None => break
            }
        }
    } */

    pub fn get_kernel_comp(&mut self, kernel_comp: KernelCompNames) -> Option<&mut Box<dyn pgtraits::RManager>> {
        self.kernel_comps.get_mut(&kernel_comp)
    }

    pub fn apply_changes(&mut self, mut add_arr: Vec<Box<dyn RManager>>) {
        loop{
            match add_arr.pop(){
                Some(mut kernel_comp) => {
                    match self.kernel_comps.insert(kernel_comp.get_name(), kernel_comp) {
                        None => (),
                        Some(comp) => eprintln!("This Kernel Component {:?} should only exist once!", comp.get_name()),
                    }
                }
                None => break
            }
        }
    }

    //pub fn attach(&mut self, comp: Box<dyn RManager>)
}


//___________________________________________
//___________________________________________
//___________________________________________


pub struct SimulationKernel {
    molecule_manager: MoleculeManager,
    movement_predictor: MovementPredictor,
    scene_manager: SceneManager,
    //mesh_manager: MeshManager,
    bitstreamgen_manager: BitstreamgenManager,
    modulation_manager: ModulationManager,
    injector_manager: InjectorManager,
    sensor_manager: SensorManager,

    //time_configs
    sim_time_limit: time::Duration,
    base_delta_time: time::Duration,
}

impl SimulationKernel {
    pub fn new_empty() -> SimulationKernel {
        SimulationKernel {
            molecule_manager: MoleculeManager::new(0),
            movement_predictor: MovementPredictor::new(None, false, None),
            scene_manager: SceneManager::new(),
            bitstreamgen_manager: BitstreamgenManager::new(),
            modulation_manager: ModulationManager::new(),
            injector_manager: InjectorManager::new(),
            sensor_manager: SensorManager::new(),
            sim_time_limit: Duration::new(0, 0),
            base_delta_time: Duration::new(0, 0),
        }
    }

    /// Constructor
    // Signature will be less verbose when "SimulationKernelInit" works
    pub fn new(
        molecule_manager: MoleculeManager,
        movement_predictor: MovementPredictor,
        scene_manager: SceneManager,
        bitstreamgen_manager: BitstreamgenManager,
        modulation_manager: ModulationManager,
        injector_manager: InjectorManager,
        sensor_manager: SensorManager,
        sim_time_limit: time::Duration,
        base_delta_time: time::Duration,
    ) -> SimulationKernel {
        SimulationKernel {
            molecule_manager,
            movement_predictor,
            scene_manager,
            bitstreamgen_manager,
            modulation_manager,
            injector_manager,
            sensor_manager,

            sim_time_limit,
            base_delta_time,
        }
    }

    pub fn get_movement_predictor(&self) -> &MovementPredictor {
        &self.movement_predictor
    }

    pub fn get_sensor_manager(&self) -> &SensorManager {
        &self.sensor_manager
    }

    pub fn get_molecule_count(&self) -> usize {
        self.sensor_manager.get_molecule_count()
    }

    pub fn get_injector_manager(&self) -> &InjectorManager {
        &self.injector_manager
    }


    /// Main simulation function
    pub fn simulation_loop(&mut self) {
        let mut sim_time = time::Duration::new(0, 0);
        let mut time_step_no = 0;
        while sim_time < self.sim_time_limit {
            //debug!("SimTime: {}", sim_time.as_secs_f64());
            for sim_stage in SimStages::into_enum_iter() {
                
                self.simulate_connected(&sim_stage, &sim_time);
                
            }
            sim_time += self.base_delta_time;
            
            if env::var("SIM2VID").is_ok() || env::var("POSITION_DEBUG").is_ok() {
                self.print_for_toolchain(time_step_no);
            }
            time_step_no += 1;
        }
    }

    
    fn print_for_toolchain(&mut self, time_step_no: usize) {
        let path = format!("{}/positions.csv.{}", self.movement_predictor.get_plotting_path().unwrap(), time_step_no);
        let mut this_path = File::create(Path::new(&path)).expect("creating positions.csv failed");
        write!(&mut this_path, "{}", self.movement_predictor.get_time_step_print()).expect("writing in positions.csv failed");
        self.movement_predictor.set_pathplotter_print(vec![]);
    }

    /// unused
    fn notify(&mut self) {
        for note_stage in NotificationStages::into_enum_iter() {
            self.notify_connected(&note_stage)
        }
    }

    /// unused
    fn notify_connected(&mut self, stage: &NotificationStages) {}

    fn simulate_connected(&mut self, stage: &SimStages, sim_time: &time::Duration) {
        match stage {
            SimStages::Bitsequence => self.bitsequence(sim_time),
            SimStages::Position => self.position(sim_time, self.base_delta_time),
            SimStages::Modulation => self.modulate(sim_time),
            //SimStages::Injecting => self.inject(),        //happens in Modulation
            //SimStages::Notify => self.notify(),
            //SimStages::Object => self.object(),
            SimStages::Sense => self.sense(sim_time),
            _ => (),
        }
    }

    fn bitsequence(&mut self, sim_time: &time::Duration) {
        
        let mods = self.modulation_manager.get_all_modulators();
        for modu in mods {
            //debug!("Before: \n{:#?}", modu);
            self.bitstreamgen_manager.bitsequence(sim_time, modu);
            //debug!("After: \n{:#?}", modu);
        }
        
    }
    fn modulate(&mut self, sim_time: &time::Duration) {
        
        let injs = self.injector_manager.get_all_injectors();
        let mut new_spawn: Vec<Box<Molecule>> = vec![];
        for inj in injs {
            self.modulation_manager.modulate(sim_time, inj);
            
            new_spawn.append(&mut inj.spawn());
            
        }
        
        //debug!("This time step {} mols are spawned", new_spawn.len());
        self.molecule_manager.add_molecules_sk(new_spawn);
        
    }

    fn position(&mut self, sim_time: &time::Duration, step_size: time::Duration) {
        let mols = self.molecule_manager.get_all_molecules();
        for mol in mols {
            self.movement_predictor
                .predict(mol, &self.scene_manager, sim_time, &step_size);
        }
    }

    fn object(&mut self) {
        let mols = self.molecule_manager.get_all_molecules();
        for mol in mols {
            //self.sensor_manager.teleport_dummy(mol);
        }
    }

    fn sense(&mut self, sim_time: &time::Duration) {
        let mols = self.molecule_manager.get_all_molecules();
        let mut del_arr: Vec<usize> = Vec::new();
        for mol in mols {
            self.sensor_manager.count_mol(mol, sim_time);
            self.sensor_manager.cam_mol(mol);
            if self.sensor_manager.destruct_mol(mol) {
                del_arr.push(mol.get_id().unwrap());
            }
        }
        self.sensor_manager.push_liv();
        self.molecule_manager.apply_changes_sk(del_arr);
    }
}


//______________________________________________
//______________________________________________
//______________________________________________


//TODO: idea is to have a SimulationKernelInit with an attach_components(KernelComponent) to make function signatures more readable.
pub struct SimulationKernelInit {
    /* molecule_manager: Option<U>,
    movement_predictor: Option<V>,
    scene_manager: Option<W>,
    sensor_manager: Option<X>, */
    molecule_manager: Option<MoleculeManager>,
    movement_predictor: Option<MovementPredictor>,
    scene_manager: Option<SceneManager>,
    sensor_manager: Option<SensorManager>,
    //mesh_manager: Option<MeshManager>,
    //injector_manager: Option<InjectorManager>,
}

/* impl SimulationKernelInit
{
    //TODO: prevent multiple overwriting
    fn attach_component<T>(&mut self, comp: T)
    where T : pgtraits::PGComponent {
        let name = comp.get_name();
        match name {
            KernelCompNames::MoleculeManager => {self.molecule_manager = comp.downcast::<MoleculeManager>()},
            KernelCompNames::MovementPredictor => {self.movement_predictor = Some(comp)},
            KernelCompNames::SceneManager => {self.scene_manager = Some(comp)},
            KernelCompNames::SensorManager => {self.sensor_manager = Some(comp)},
            _ => panic!("This is not a valid kernelcomponent :<")
        }
    }
} */

//When casting works reimplement
impl SimulationKernel {
    /*
    pub fn new (init: SimulationKernelInit) -> SimulationKernel {
        SimulationKernel {
            molecule_manager: init.molecule_manager.unwrap(),
            movement_predictor: init.movement_predictor.unwrap(),
            scene_manager: init.scene_manager.unwrap(),
            //mesh_manager: init.mesh_manager.unwrap(),
            //injector_manager: init.injector_manager.unwrap(),
            sensor_manager: init.sensor_manager.unwrap(),
            //time_configs - init.time_configstruct.unwrap()
        }
    } */
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::rogona::sensor_mod::sensor_destructing::SensorDestructing;

    fn build_example_mol(x: f64) -> Molecule {
        let pos = Point3::new(x, 0.0, 1.0);
        let vel = Vector3::new(0.0f64, 1.0, 1.0);
        Molecule::new(pos, vel, Some(1))
    }

    #[test]
    //attempt for the simulation_kernel_init in later versions
    fn tester_02() {
        let mut comps: HashMap<KernelCompNames, KernelComp> = HashMap::new();
        let mut mol_man = KernelComp::MoleculeManager(MoleculeManager::new(5));
        let mut scene_man = KernelComp::SceneManager(SceneManager::new());

        comps.insert(KernelCompNames::MoleculeManager, mol_man);
        comps.insert(KernelCompNames::SceneManager, scene_man);

        println!("after first insert:\ncurrent comps: {:?}\n\n\n", comps);

        let mut v = match comps.get_mut(&KernelCompNames::MoleculeManager).unwrap() {
            KernelComp::MoleculeManager(molman) => molman,
            _ => {
                panic!("There is no Molecule Manager attached oh no!")
            }
        };

        let m1 = build_example_mol(1.0f64);
        v.add_molecule(m1);

        println!("after changing molman:\ncurrent comps: {:?}", comps);
    }

    #[test]
    //Proof of concept for one timestep without sensor manager
    fn tester_01() {
        let mut mol_man = MoleculeManager::new(5);
        let m1 = build_example_mol(1.0f64);
        let m2 = build_example_mol(2.0f64);
        let m3 = build_example_mol(3.0f64);
        let m4 = build_example_mol(4.0f64);
        mol_man.add_molecule(m1);
        mol_man.add_molecule(m2);
        mol_man.add_molecule(m3);
        mol_man.add_molecule(m4);
        mol_man.apply_changes();

        let movement_predictor = MovementPredictor::new(None, false, None);

        let sensor_des = SensorDestructing::new(None, Some((8.0, 13.0)), None);

        println!("Mol_Man state: {:?}", mol_man);

        //skip sensor manager

        //predict new position

        let mols = mol_man.get_all_molecules();
        for mol in mols {
            //movement_predictor.predict_dummy(mol);
        }

        println!("\n\n\nMol_Man state: {:?}", mol_man);

        //predict if it should be destructed

        let mols = mol_man.get_all_molecules();

        //Sensor_Manager would manage the data
        let mut del_arr: Vec<usize> = Vec::new();
        for mol in mols {
            match sensor_des.destruct(mol) {
                true => del_arr.push(mol.get_id().unwrap()),
                false => (),
            }
        }
        mol_man.apply_changes_sk(del_arr);

        println!("\n\n\nMol_Man state: {:?}", mol_man);
    }
}
