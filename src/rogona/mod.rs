pub mod rogona_main;

pub mod init;

pub mod rg_attributes;


pub mod simulation_kernel;

pub mod object_mod;
pub mod vector_field_mod;
pub mod mesh_mod;

pub mod bitstream_generator_mod;
pub mod modulation_mod;
pub mod injector_mod;

pub mod molecule_mod;

pub mod sensor_mod;






// * Structs for everyone


use nalgebra::Vector3;
// TODO [Future Work] more than a cube with fn as args
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Section3D {
    min: Vector3<f64>,
    max: Vector3<f64>,
}

