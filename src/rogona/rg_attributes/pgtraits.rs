use std::any::Any;

use crate::rogona::{
    rg_attributes::stages::{InitStages, NotificationStages, SimStages}
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum KernelCompNames {
    MoleculeManager,
    MovementPredictor,
    SceneManager,
    SensorManager,
    MeshManager,
    InjectorManager,
    BitstreamgenManager,
    ModulationManager,
    NotImplemented,
}


pub trait PGObj {
    fn as_any(&self) -> &dyn Any;
}

pub trait PGComponent {
    type Comp;

    fn initialize(&self) {}

    fn notify_own(&mut self, note_stage: &NotificationStages) {}

    fn simulate_own(&mut self, sim_stage: &SimStages) {}

    fn attach_component(&mut self, comp: Self::Comp) {}

    fn get_name(&self) -> KernelCompNames {
        KernelCompNames::NotImplemented
    }
}

pub trait PGSensor {
}

pub trait RManager {
    fn as_any(&self) -> &dyn Any;

    fn get_name(&self) -> KernelCompNames {
        KernelCompNames::NotImplemented
    }

    fn apply_changes_t(&mut self, mut add_arr: Vec<Box<dyn PGObj>>) {}
}

//impl PGObj for dyn PGSensor {}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Interpolation {
    Linear,     // for air-based-scenario
    Euler,
    RungeKutta,
    NotImplemented
}

#[derive(Debug, PartialEq)]
pub enum InjType {
    SprayNozzle,
    Spawn,
    Other
}