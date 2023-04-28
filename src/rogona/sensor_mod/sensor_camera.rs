use nalgebra::{Vector3, Point3};

use crate::rogona::{
    rg_attributes::pgtraits::PGObj
};



#[derive(Debug)]
pub struct SensorCamera{
    position: Vector3<f64>,
    height: f64,
    width_projected: f64,
    ratio: f64,
    mol_count: usize,
    liv: f64,
    liv_arr: Vec<f64>,
}

impl PGObj for SensorCamera {
	fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl SensorCamera {
    pub fn new (position: Vector3<f64>, height: f64, width_projected: f64, cam_ratio: (f64, f64)) -> SensorCamera {
        let (y, x) = cam_ratio;
        let ratio = y/x;

        SensorCamera {
            position,
            height,
            width_projected,
            ratio,
            mol_count: 0,
            liv: 0.0,
            liv_arr: vec![],
        }
    }

    /* z - height
       y - width
       x - depth */ 
    // Is mol in area?
    pub fn check(&mut self, mol_pos: Point3<f64>) -> bool {
        let htx = mol_pos.z - (self.position.z - self.height);
        let hrx = self.height;

        if mol_pos.z >= (self.position.z - self.height) && mol_pos.z < self.position.z {
            let width = (1.0f64-(htx/hrx)) * self.width_projected; // Intercept theorem
            let w_min = self.position.y - width/2.0;
            let w_max = self.position.y + width/2.0;
            if mol_pos.y >= w_min && mol_pos.y <= w_max {
                let depth = width / self.ratio;
                let d_min = self.position.x - depth/2.0;
                let d_max = self.position.x + depth/2.0;
                if mol_pos.x >= d_min && mol_pos.x <= d_max {
                    self.mol_count +=1;
                    self.liv += damp_function(self.mol_count);
                    return true;
                }
            }
        }
        false
    }

    pub fn push_liv(&mut self) {
        self.liv_arr.push(self.liv);
        self.mol_count = 0;
        self.liv = 0.0;
    }

    pub fn get_liv_arr(&self) -> Vec<f64> {
        self.liv_arr.clone()
    }

    pub fn get_liv(&self) -> f64 {
        self.liv
    }


}


fn damp_function(i: usize) -> f64 {
    return 1.0;

    // TODO [Future Work] damping factor due to overlap
}