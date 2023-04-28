use enum_iterator::IntoEnumIterator;

/* Initialization Stages
use InitStages::into_enum_iter() to generate an iterator through the stages
*/
// TODO: Determine whether this is needed or not
#[derive(Debug, IntoEnumIterator, PartialEq)]
pub enum InitStages {
    BuildScene,
    CheckArguments,
    CreateDataStructures,
    CreateFiles,
    CreateFolders,
    CreateSensorSubscriptions,
    CreateTeleporters,
    RegisterSensors,
    SetUpFlowSystem,
    StartSimulation,
}

/* Notification Stages */
// ! not needed in air-based scenario
// TODO: which notification stage does what?

#[derive(Debug, IntoEnumIterator, PartialEq)]
pub enum NotificationStages {
    Bitstreaming,   //
    Destructing,    //
    Logging,        //
    Modulation,     //
    Pumping,        //
    Spawning,       //
}

/* Simulation Stages
add stages when necessary/appropriate*/

/**SimStages explanation
 * Read:
 *      message to transmit from file until Buffer is full. If buffer isn't empty, do nothing   (IN bitstreamgenerator.rs)
 * Bitsequence:
 *      convert ASCII string to bits as symbols (IN bitstreamgenerator.rs); set up Modulation
 * Modulation:
 *      set parameters for transition times (IN modulationOOK.rs); activates injection
 * Position:
 *      new position and cell id for all molecules (IN movement-predictor.rs) //TODO: INCL. position recording
 * Object:
 *      (IN scene-manager.rs, sensor-teleport.rs) //in later implementations
 * Destruct:
 *      delete molecules from simulation (IN sensor-destructing.rs, molecule-manager.rs)
  */

#[derive(Debug, IntoEnumIterator, PartialEq)]
pub enum SimStages {
//    Read,           // TODO: [Future Work] Implement possibility of a refreshing file for continuous transmission
    Bitsequence,    // (similar to previously: Bitstreaming) sends a bit to the modulation
    Position,       // ! different to before. Injection already incorporates the movement during the time-step
    Modulation,     // turns injectors on or off including all other param for successful spraying
    Object,         // sensor teleport and scene manager
    Sense           //
}

/* Kernel Component Enums (maybe move to diffrent file) */

use crate::rogona::{
    molecule_mod::{molecule_manager::MoleculeManager, movement_predictor::MovementPredictor},
    object_mod::scene_manager::SceneManager, 
    sensor_mod::sensor_manager::SensorManager,
};

#[derive(Debug)]
pub enum KernelComp {
    MoleculeManager(MoleculeManager),
    MovementPredictor(MovementPredictor),
    SceneManager(SceneManager),
    SensorManager(SensorManager),
    MeshManager,
}
