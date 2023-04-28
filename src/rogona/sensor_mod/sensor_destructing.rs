use nalgebra::{Point, U3}; // for test

use crate::rogona::{molecule_mod::molecule::Molecule, rg_attributes::pgtraits::PGSensor};

#[derive(Debug)]
pub struct SensorDestructing {
    x: bool,
    y: bool,
    z: bool,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    zmin: f64,
    zmax: f64,
}

impl PGSensor for SensorDestructing {}

impl SensorDestructing {

    // TODO [Future Work fluid-based] Mapping to cell IDs
    pub fn new(
        xminmax: Option<(f64, f64)>,
        yminmax: Option<(f64, f64)>,
        zminmax: Option<(f64, f64)>,
    ) -> SensorDestructing {
        SensorDestructing {
            x: match xminmax{
                Some(_) => true,
                None => false,
            },
            y: match yminmax{
                Some(_) => true,
                None => false,
            },
            z: match zminmax{
                Some(_) => true,
                None => false,
            },
            xmin: match xminmax{
                Some((xmin, _)) => xmin,
                None => 0.0,
            },
            xmax: match xminmax{
                Some((_, xmax)) => xmax,
                None => 0.0,
            },
            ymin: match yminmax{
                Some((ymin, _)) => ymin,
                None => 0.0,
            },
            ymax: match yminmax{
                Some((_, ymax)) => ymax,
                None => 0.0,
            },
            zmin: match zminmax{
                Some((zmin, _)) => zmin,
                None => 0.0,
            },
            zmax: match zminmax{
                Some((_, zmax)) => zmax,
                None => 0.0,
            },
        }
    }

    // Is mol in section?
    pub fn destruct(&self, mol: &Box<Molecule>) -> bool {
        let mut marker = false;
        let pos = mol.get_position();

        if self.x == true {
            if pos.x > self.xmin && pos.x < self.xmax {
                marker = true;
            } else {
                return false;
            }
        }
        if self.y == true {
            if pos.y > self.ymin && pos.y < self.ymax {
                marker = true;
            } else {
                return false;
            }
        }
        if self.z == true {
            if pos.z > self.zmin && pos.z < self.zmax {
                marker = true;
            } else {
                return false;
            }
        }
        return marker;
    }




    
    pub fn destruct_test(&self, pos: Point<f64, U3>) -> bool {
        let mut marker = false;

        if self.x == true {
            if pos.x > self.xmin && pos.x < self.xmax {
                marker = true;
            } else {
                return false;
            }
        }
        if self.y == true {
            if pos.y > self.ymin && pos.y < self.ymax {
                marker = true;
            } else {
                return false;
            }
        }
        if self.z == true {
            if pos.z > self.zmin && pos.z < self.zmax {
                marker = true;
            } else {
                return false;
            }
        }
        return marker;
    }
}


#[cfg(test)]
mod tests{
    use nalgebra::Point3;

    use super::*;

    fn make_sensor_des() -> SensorDestructing {
        SensorDestructing { x: true, y: false, z: true, xmin: 0.0, xmax: 1.0, ymin: 0.0, ymax: 1.0, zmin: 0.0, zmax: 1.0 }
    }

    #[test]
    fn des(){
        let s = make_sensor_des();
        assert!(s.destruct_test(Point3::new(0.5f64,5.3, 0.7)));
        assert!(!s.destruct_test(Point3::new(0.6f64, 8.0, 2.1)));
    }
}