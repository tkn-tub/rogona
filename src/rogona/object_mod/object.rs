/// Not needed in airbased scenario

/* ! Currently not used. Will be effective in future versions for more abstraction and modelling details */

use std::collections::HashSet;
use std::path::PathBuf;

use crate::rogona::{
    rg_attributes::{pgtraits::Interpolation, stages},
    vector_field_mod::vector_field_manager,
    simulation_kernel
};

use na::{Point3, Vector3};
use nalgebra as na;


pub struct Object {
    pub object_id: Option<u64>,     //Object index, set by the scene manager.
    translation: Point3<f64>,
    rotation: Point3<f64>,
    scale: Point3<f64>,
    openfoam_cases_path: Option<PathBuf>,
    walls_patch_names: HashSet<String>,
    inlets: HashSet<String>,
    outlets: HashSet<String>,
    name: String,
    pub is_active: bool,
    default_time_str: String,
    vector_field_manager: Option<vector_field_manager::VectorFieldManager>,
}

impl Object {
    fn new() -> Object {
        //Set default variables
        let mut object_id = None;
        let openfoam_cases_path = None;
        let mut translation = Point3::new(0.0f64, 0.0, 0.0);
        let mut rotation = Point3::new(0.0f64, 0.0, 0.0);
        let mut scale = Point3::new(1.0f64, 1.0, 1.0);
        let walls_patch_names = HashSet::from([
            String::from("walls"),
            String::from("yConnectorPatch"),
            String::from("tubePatch"),
        ]);
        /* A superset of names of wall patches.
        Required by the VectorFieldParser:
        "Which patches to consider as boundaries
        (typically does not include inlets and outlets). */

        //let mut transformation = None;
        //Transformation of this object in the scene.
        let mut inlets = HashSet::new();
        /*Names of the inlets in the OpenFOAM mesh*/
        let mut outlets = HashSet::new();
        /*Names of the outlets in the OpenFOAM mesh*/
        let mut name = String::from("Generic Object");
        /* The name of this type of object.
        Not a unique identifier like Component.component_name! */
        let mut is_active = true;
        /*If True, indicates that fluid inside this Object is moving.  */
        let mut default_time_str = String::from("latest");
        /*      Default sub-folder of the OpenFOAM simulation results to use.
        If "latest", will search for the sub-folder which name is a
        floating point number and has the greatest value. */

        let mut vector_field_manager = None;
        /*This object's VectorFieldManager, set by child classes. */

        Object {
            object_id,
            translation,
            rotation,
            scale,
            openfoam_cases_path,
            //transformation,
            walls_patch_names,
            inlets,
            outlets,
            name,
            is_active,
            default_time_str,
            vector_field_manager,
        }
    }
}



pub trait TObject {

    fn get_flow_by_position(&self, position_local: Point3<f64>, int_method: Interpolation) -> Vector3<f64>;

    fn get_cell_id_by_position(&self, position_local: Point3<f64>) -> Option<usize>;

    // ~~ for future implementation
    fn get_path(&self);

    fn get_mesh_index(&self);

    fn find_latest_time_step(path: String);
}

// TODO: reimplement when more object type emerge
/* impl TObject for Object{




fn initialize(
    &self,
    simulation_kernel: &simulation_kernel::SimulationKernel,
    init_stage: &stages::InitStages ) {

        match init_stage {
           //TODO: init_stages::InitStages::CheckArguments => {
           //     self.transformation = Some(transformation::Transformation::new());
           // }
           //TODO: init_stages::InitStages::BuildScene => {}
            init_stages::InitStages::CreateDataStructures => {
                self.load_current_vector_field(simulation_kernel);
            },
            _ => {}
        }
 }

fn load_current_vector_field(
    &self,
    simulation_kernel: &simulation_kernel::SimulationKernel) -> Option<i64> {
        let mut old_mesh_size : Option<i64> = None;
        let mut new_mesh_size : Option<i64> = None;

        if !self.is_active {
        // This saves us the extra step of having to load a mesh
        // with all 0s.
            old_mesh_size= match self.vector_field_manager {
                Some(x) => {Some(x.get_mesh().len())},
                None => {None}
            }
        }
        let mut vector_field = simulation_kernel.get_mesh_manager().load_vector_field(

            openfoam_sim_path=self.get_path(),
            mesh_index=self.get_mesh_index(),
            walls_patch_names=self.walls_patch_names,

        );
        new_mesh_size = match self.vector_field_manager {
            Some(x) => {Some(x.get_mesh().len())},
            None => {None}
        };

        match (old_mesh_size, new_mesh_size) {
            // Comparison will only apply to states of this Object in which
            // self.is_active was True.
            (Some(x), Some(y)) => {
                if x!=y {
                    panic!("Object {} loaded a new vector field, \n
                    but the new mesh ({} cells) doesn't match \n
                    the old one ({} cells)."
                    , self.name, new_mesh_size.unwrap(), old_mesh_size.unwrap() )

                }
            },
            _ => {}
        };
        old_mesh_size
    }
fn get_closest_cell_centre_id(
    &self,
    position_local: Point3<f64>) -> Option<Vector3<f64>> {//cKDTree.query() but in rstar
        /*
        :returns: Array of closest cell centers to `position`.
            If there's only one position, output is squeezed.
            None if this Object is inactive.
            See documentation of cKDTree.query().
        */
        match self.is_active{
            False => {return None},
            True => {
                return self.vector_field_manager.get_closest_cell_centre_id(
                    position_local=position_local)}
        }
    }


fn get_path(&self){}

fn get_mesh_index(&self) {}

fn find_latest_time_step(path: String) {}






} */
