use nalgebra::{Point3, Vector3};
use std::fmt;
use std::time;

use crate::rogona::Section3D;
use crate::rogona::{
    molecule_mod::molecule::Molecule,
    object_mod::scene_manager::SceneManager,
    rg_attributes::pgtraits::{Interpolation, KernelCompNames, PGComponent, PGObj},
};


/* TODO [Future Work fluid-based]
    Runge-Kutta-Interpolation
    implementing branches for Interpolation styles
*/

#[derive(Debug)]
pub struct Pathplotter {
    tracer: Vec<Vec<(time::Duration, Point3<f64>)>>,

    time_step_print: Vec<(usize, Point3<f64>)>,
    out_path: Option<String>,

    full: bool,
    area: Option<Section3D>,
}

#[derive(Debug)]
pub struct MovementPredictor {
    interpolation_method: Interpolation,
    pathplotter: Pathplotter,
}

// ! Print for Blender Add-on

impl fmt::Display for Pathplotter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut printer = String::from("id,x,y,z,cell_id,obj_id\n");
        for (id, pos) in self.time_step_print.iter() {
            printer = printer + format!("{},{},{},{},-1,0\n", id, pos.x, pos.y, pos.z).as_str();
        }

        write!(f, "{}", printer)
    }
}


impl PGComponent for MovementPredictor {
    type Comp = (); //TODO[Future Work]

    fn get_name(&self) -> KernelCompNames {
        KernelCompNames::MovementPredictor
    }
}

impl MovementPredictor {
    pub fn new(printpath: Option<String>, full_area: bool, area_3d: Option<Section3D>) -> MovementPredictor {
        let pathplotter = Pathplotter{
            tracer: vec![],
            time_step_print: vec![],
            out_path: printpath,
            full: full_area,
            area: area_3d
        };
        
        MovementPredictor {
            interpolation_method: Interpolation::NotImplemented,
            pathplotter
        }
    }

    pub fn set_interpolation_method(&mut self, int_method: Interpolation) {
        self.interpolation_method = int_method;
    }

    pub fn set_pathplotter_print(&mut self, vec: Vec<(usize, Point3<f64>)>) {
        self.pathplotter.time_step_print = vec;
    }

    pub fn reset_pathplotter_print(&mut self) {
        self.pathplotter.time_step_print = vec![];
    }

    pub fn get_plotting_path(&self) -> Option<String> {
        self.pathplotter.out_path.clone()
    }

    pub fn get_time_step_print(&self) -> &Pathplotter {
        &self.pathplotter
    }

    /* arguments:
    &mut Box<Molecule>				directly change the Molecule Values on the heap
    SceneManager 					to get the flow from the object and calculate new position*/
    pub fn predict(
        &mut self,
        mol: &mut Box<Molecule>,
        scene_man: &SceneManager,
        sim_time: &time::Duration,
        delta_time: &time::Duration,
    ) {
        let flow;

        if self.interpolation_method == Interpolation::Linear {
            flow = mol.get_velocity();
        } else {
            //Interpolation (k-) Nearest Neighbor
            flow = scene_man.get_flow_by_position(
                mol.get_position(),
                mol.get_object_id().unwrap(),
                self.interpolation_method,
            );
        }

        //Integration Forward Euler and Linear
        mol.set_position(Point3::from(
            (mol.get_position()) + flow * delta_time.as_secs_f64(),
        ));

        // ! not used by air-based case
        /* //update cell ID
        let cell_id =
            scene_man.get_cell_id_by_position(mol.get_position(), mol.get_object_id().unwrap());
        mol.set_cell_id(cell_id); */

        //record trajectory

        //self.trace(sim_time, mol);
        self.step_plot(mol);
    }

    // store molecule position this time-step
    fn step_plot(&mut self, mol: &Box<Molecule>) {
        if !self.pathplotter.full {
            let pos = mol.get_position();
            if pos.x < self.pathplotter.area.unwrap().min.x || pos.x > self.pathplotter.area.unwrap().max.x {
                return
            }
            if pos.y < self.pathplotter.area.unwrap().min.y || pos.y > self.pathplotter.area.unwrap().max.y {
                return
            }
            if pos.z < self.pathplotter.area.unwrap().min.z || pos.z > self.pathplotter.area.unwrap().max.z {
                return
            }
        }
        self.pathplotter.time_step_print
            .push((mol.get_id().unwrap(), mol.get_position()))
    }

    // store molecule position over full simulation
    fn trace(&mut self, sim_time: &time::Duration, mol: &Box<Molecule>) {
        let id = mol.get_id().unwrap();

        loop {
            if id < self.pathplotter.tracer.len() {
                break;
            }
            self.pathplotter.tracer.push(vec![]);
        }

        self.pathplotter.tracer[id].push((*sim_time, mol.get_position()));
    }
}
