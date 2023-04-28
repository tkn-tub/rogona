/// Not needed in airbased scenario



use std::{collections::HashSet, path::PathBuf};

use nalgebra::{Point3, Vector3};

use crate::rogona::{
    object_mod::object,
    rg_attributes::{pgtraits::Interpolation, stages},
    vector_field_mod::{vector_field_manager::VectorFieldManager}
};



#[derive(Debug)]
pub struct Tube {
    name: Option<String>,
    radius: Option<f64>,
    // Radius of the tube in m.
    length: Option<f64>,
    // Length of the tube in m.
    inlet_zone: Option<f64>,
    /* Length of the inlet zone in m.

    This segment at the start of the tube mesh will be cut off,
    since the OpenFOAM simulation showed that the flow profile
    before this threshold does not yet match the analytically
    determined profile of a tube to a sufficient degree.

    'Cut off' here means that this ObjectTube will be shifted
    along its axis such that it will appear as though you are
    dealing with a tube that is simply `inlet_zone` metres shorter.*/
    outlet_zone: Option<f64>,
    /*    For teleporting: If a molecule enters this zone at the end of the
    tube, it will be teleported to a connected object. */
    flow_rate: Option<f64>,
    //Flow rate in ml/min.
    mesh_resolution: Option<u64>,
    /*    Mesh resolution, defined as number of
    'radius cells' in the OpenFOAM blockMeshDict. */
    variant: Option<String>,
    /*    Additional variant information.

    If given, look for an OpenFOAM case with '_<variant>' appended
    to its path name. */
    fallback_flow_rate: Option<u64>,
    /*    Fallback flow rate for making sensor subscriptions possible in the
    case that this Object starts with an overall flow rate of 0
    (i.e., `is_active == False`). */
    mesh_length_cm: i32,
    /*
    The actual length of the underlying mesh.
    Should account for both self.length and self.inlet_zone.
    */
    pub object_id: Option<u64>,
    //Object index, set by the scene manager.
    translation: Point3<f64>,
    rotation: Point3<f64>,
    scale: Point3<f64>,
    openfoam_cases_path: Option<PathBuf>,
    walls_patch_names: HashSet<String>,
    inlets: HashSet<String>,
    outlets: HashSet<String>,
    pub is_active: bool,
    default_time_str: String,
    vector_field_manager: Option<VectorFieldManager>,
}

impl Tube {
    pub fn new(length: f64, radius: f64, vfm: Option<VectorFieldManager>) -> Tube {
        let name = Some(String::from("Tube"));

        let inlet_zone = Some(0.05);
        let outlet_zone = Some(0.005);
        let flow_rate = Some(5 as f64);
        let mesh_resolution = Some(11);
        let variant = Some(String::from(""));
        let fallback_flow_rate = Some(5);
        let mesh_length_cm = 15;

        let object_id = None;
        let openfoam_cases_path = None;
        let translation = Point3::new(0.0f64, 0.0, 0.0);
        let rotation = Point3::new(0.0f64, 0.0, 0.0);
        let scale = Point3::new(1.0f64, 1.0, 1.0);
        let walls_patch_names = HashSet::from([
            String::from("walls"),
            String::from("yConnectorPatch"),
            String::from("tubePatch"),
        ]);
        /* A superset of names of wall patches.
        Required by the VectorFieldParser:
        "Which patches to consider as boundaries
        (typically does not include inlets and outlets). */

        let inlets = HashSet::new();
        /*Names of the inlets in the OpenFOAM mesh*/
        let outlets = HashSet::new();
        /*Names of the outlets in the OpenFOAM mesh*/

        let is_active = true;
        /*If True, indicates that fluid inside this Object is moving.  */
        let default_time_str = String::from("latest");
        /*      Default sub-folder of the OpenFOAM simulation results to use.
        If "latest", will search for the sub-folder which name is a
        floating point number and has the greatest value. */

        let vector_field_manager = vfm;
        /*This object's VectorFieldManager, set by child classes. */

        Tube {
            name,
            radius: Some(radius),
            length: Some(length),
            inlet_zone,
            outlet_zone,
            flow_rate,
            mesh_resolution,
            variant,
            fallback_flow_rate,
            mesh_length_cm,
            object_id,
            translation,
            rotation,
            scale,
            openfoam_cases_path,
            walls_patch_names,
            inlets,
            outlets,
            is_active,
            default_time_str,
            vector_field_manager,
        }
    }
}

impl object::TObject for Tube {
    
    fn get_flow_by_position(&self, position_global: Point3<f64>, int_method: Interpolation) -> Vector3<f64> {
        self.vector_field_manager
            .as_ref()
            .unwrap()
            .get_flow_by_position(position_global, Some(int_method))  // !CHANGE INTERPOLATION HANDOVER SO IT IS DEFINED IN SIMULATION KERNEL
            .unwrap()
    }

    fn get_cell_id_by_position(&self, position_local: Point3<f64>) -> Option<usize> {
        self.vector_field_manager
            .as_ref()
            .unwrap()
            .get_cell_id_by_position(position_local)
    }

    // ~~ for future implementation
    fn get_path(&self) {}

    fn get_mesh_index(&self) {}

    fn find_latest_time_step(path: String) {}
}

