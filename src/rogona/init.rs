use crate::rogona::{
    bitstream_generator_mod::{
        bitstreamgen_manager::BitstreamgenManager, bitstreamgenerator::Bitstreamgenerator,
    },
    injector_mod::{injector::SprayNozzle, injector_manager::InjectorManager},
    mesh_mod::mesh_manager,
    modulation_mod::{modulation_ook::ModulationOOK, modulation_manager::ModulationManager},
    molecule_mod::{
        molecule::Molecule, molecule_manager::MoleculeManager,
        movement_predictor::MovementPredictor,
    },
    object_mod::{scene_manager::SceneManager, tube::Tube},
    rg_attributes::pgtraits::{Interpolation, RManager},
    sensor_mod::{
        sensor_camera::SensorCamera, sensor_counting::SensorCounting,
        sensor_destructing::SensorDestructing, sensor_manager::SensorManager,
    },
    simulation_kernel::SimulationKernel,
    vector_field_mod::{
        vector_field_manager::VectorFieldManager, vector_field_parser::VectorFieldParser,
    }, Section3D,
};

use nalgebra::{Matrix3, Point3, Vector3};
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, env};
use std::{fs, time};

extern crate log;
use log::{debug, error, info, warn};

/* This is with a hardcoded configuration */

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ConfigVars {
    // also Reconstruction vars
    pub mps: usize, // mol per spray
    pub d: f64,     // distance between sprayers in m
    pub dy: f64,    // offset of the receiver in m
    pub t_sim: f64, // simulation time limit in s
    pub t_sym: f64, // duration of one symbol (spray_time + pause_time) in s
    pub t_sp: f64,  // spray_time
    pub t_dts: f64, // delta time step (attribute to camera shutter time)
    pub cam_fps: usize,
    pub v_mean: f64,  // initial spray velocity
    pub v_sigma: f64, // velocity sigma
    // Simulation vars
    pub dist_sigma: f64,  // distribution sigma
    pub lmc: Option<f64>, // liter_molecule_conversion (mol per liter)
    // Inj far
    pub off_f: f64,    // symbol offset in starting time in symbols
    pub rep_f: u32,    // repetitions of the message
    pub msg_f: String, // file path for msg
    // Inj near
    pub off_n: f64,
    pub rep_n: u32,
    pub msg_n: String,
    // Sensor camera
    pub cam_z: f64,
    pub cam_height: f64,
    pub cam_w_proj: f64,
    pub cam_ratio_l: f64,
    pub cam_ratio_w: f64,

    // Output files
    pub liv_path: String,
}

fn get_parameters(config_path: &str) -> ConfigVars {
    let c = fs::read_to_string(config_path).expect("unable to find config_path in init");
    let mut c: ConfigVars = serde_yaml::from_str(&c).unwrap();
    if env::var("SIM2VID").is_ok() {
        c.t_dts = 1.0 / (c.cam_fps as f64);
    }
    c
}

pub fn init_air_tube(config_path: &str) -> SimulationKernel {
    info!("Initializing Simulation Kernel");

    let config = get_parameters(config_path);

    if config.t_dts > config.t_sym {
        error!(
            "Time Step Size (t_dts = {}) MUST be smaller than Symbol Duration (t_sym = {})",
            config.t_dts, config.t_sym
        );
        panic!()
    }
    // *"config variables" for our scenario

    let sim_time_limit = time::Duration::from_secs_f64(config.t_sim); // maximum simulation time
    let base_delta_time = time::Duration::from_secs_f64(config.t_dts); // time step size // ! should be <= 1/480 s = approx 2 ms

    // * Injectors

    /*
    distance (translates to translation of one Inj)
    inj_amount_per_time = 0.54 liter/minute = 0.009 liter/second
    shutter_time = minimum 30ms = 0.03 s
    liter_molecule_conversion = _ (molecules/liter)

    velocity (mean) = 12.82 meter/second
    velocity (sigma) = 3 m/s
    distribution (sigma) = 1.55      (in y speed to simulate different droplet sizes and therefore air drag)

    */
    let distance: f64 = config.d;

    let inj_amount_per_time = 0.009;
    let shutter_time = time::Duration::from_secs_f64(config.t_sp);
    // for constant conversion
    let liter_molecule_conversion = config.lmc;
    let mps = config.mps; // if you want to go by mol per spray = inj_amount_base

    let inj_amount_base: i64 = match liter_molecule_conversion {
        Some(lmc) => (inj_amount_per_time * shutter_time.as_secs_f64() * lmc).floor() as i64,
        None => mps as i64,
    };

    let velocity = config.v_mean;
    let velocity_sigma = config.v_sigma;
    let distribution_sigma = config.dist_sigma;

    let zero_vec = Vector3::new(0.0, 0.0, 0.0);
    // inj in (0,0,0) (injector far)
    let inj_0 = SprayNozzle::new(
        0,
        zero_vec,
        zero_vec,
        zero_vec,
        false,
        inj_amount_base,
        velocity,
        velocity_sigma,
        distribution_sigma,
        None,
        base_delta_time,
        shutter_time,
    );
    // inj in (0, d, 0) (injector near)
    let inj_1 = SprayNozzle::new(
        1,
        Vector3::new(0.0, distance, 0.0),
        zero_vec,
        zero_vec,
        true,
        inj_amount_base,
        velocity,
        velocity_sigma,
        distribution_sigma,
        None,
        base_delta_time,
        shutter_time,
    );

    // * Modulation

    /*
     start_time (same as Bitstreamgen)
     symbol duration = shutter_time (inj) + pause_time
    */

    let symbol_duration = time::Duration::from_secs_f64(config.t_sym);

    let (off_f, off_n) = correct_negative_offsets(config.off_f, config.off_n);

    // Modulation for Inj 0 far
    let offset_0 = off_f;
    let start_time_0 =
        time::Duration::from_nanos((symbol_duration.as_nanos() as f64 * offset_0).floor() as u64);
    let mod_0 = ModulationOOK::new(0, start_time_0, symbol_duration, base_delta_time);

    // Modulation for Inj 1 near
    let offset_1 = off_n;
    let start_time_1 =
        time::Duration::from_nanos((symbol_duration.as_nanos() as f64 * offset_1).floor() as u64);
    let mod_1 = ModulationOOK::new(1, start_time_1, symbol_duration, base_delta_time);

    // * Bitstreamgenerators

    /*
     start_time
     repetitions
     file_path
     binary_msg -> via environment variable
    */

    let repetitions_0 = config.rep_f;
    let file_path_0 = &config.msg_f[..];
    let bg_0 = Bitstreamgenerator::new(0, start_time_0, repetitions_0, file_path_0);

    let repetitions_1 = config.rep_n;
    let file_path_1 = &config.msg_n[..];
    let bg_1 = Bitstreamgenerator::new(1, start_time_1, repetitions_1, file_path_1);

    
    // * Sensor Counting Camera

    let delta_y = config.dy;
    let cam_z = config.cam_z;
    let height = config.cam_height;
    let width_projected = config.cam_w_proj; // ! ?
    let cam_ratio = (config.cam_ratio_l, config.cam_ratio_w);
    let s_cam = SensorCamera::new(
        Vector3::new(0.0, (distance / 2.0) + delta_y, cam_z),
        height,
        width_projected,
        cam_ratio,
    );

    // * Sensor Destructing

    /*
     one left and one right to save capacity
    */

    let margin = 1E-6;
    let s_des_0 = SensorDestructing::new(None, Some((f64::MIN, -margin)), None);
    let s_des_1 = SensorDestructing::new(None, Some((distance + margin, f64::MAX)), None);


    // * Build kernel components
    let molecule_manager = MoleculeManager::new(config.mps); //trade memory for speed - read from config how many molecules will be approx added every time step to size the vector accordingly

    let mut movement_predictor = MovementPredictor::new(
            if env::var("SIM2VID").is_ok() {
            let mut s = config.liv_path.split(".txt");
            Some(String::from(s.next().expect("No path given in liv_path to constitute pathplotter")))
        } else {
            None
        },
        if env::var("POSITION_DEBUG").is_ok() {
            true
        } else {
            false
        },
        if env::var("POSITION_DEBUG").is_ok() {
            None
        } else {
            let min = Vector3::new(-0.5 * config.cam_w_proj * config.cam_ratio_w/config.cam_ratio_l, config.d/2.0 + config.dy - 0.5 * config.cam_w_proj, config.cam_z - config.cam_height);
            let max = Vector3::new(0.5 * config.cam_w_proj * config.cam_ratio_w/config.cam_ratio_l, config.d/2.0 + config.dy + 0.5 * config.cam_w_proj, config.cam_z);
            Some(Section3D{min, max})
        }
    );

    let mut scene_manager = SceneManager::new(); // not needed for this airbased scenario

    let mut bitstreamgen_manager = BitstreamgenManager::new();

    let mut modulation_manager = ModulationManager::new();

    let mut injector_manager = InjectorManager::new();

    let mut sensor_manager = SensorManager::new();

    // *MOLECULE MANAGER

    // *MOVEMENT PREDICTOR

    movement_predictor.set_interpolation_method(Interpolation::Linear);

    // *INJECTOR MANAGER

    let inj_arr = vec![Box::new(inj_0), Box::new(inj_1)];
    injector_manager.apply_changes(inj_arr);

    // *MODULATION MANAGER

    let mod_arr = vec![Box::new(mod_0), Box::new(mod_1)];
    modulation_manager.apply_changes(mod_arr);

    // *BITSTREAMGEN MANAGER

    let bg_arr = vec![Box::new(bg_0), Box::new(bg_1)];
    bitstreamgen_manager.apply_changes(bg_arr);

    // *SENSOR MANAGER

    sensor_manager.add_sensor_destructing(s_des_0);
    sensor_manager.add_sensor_destructing(s_des_1);
    sensor_manager.attach_camera(s_cam);

    // *attach kernel components to simulation_kernel

    let simulation_kernel = SimulationKernel::new(
        molecule_manager,
        movement_predictor,
        scene_manager,
        bitstreamgen_manager,
        modulation_manager,
        injector_manager,
        sensor_manager,
        sim_time_limit,
        base_delta_time,
    );

    simulation_kernel
}

// Relative Offset to receiver and other injector is consistent. The exact frame offset cannot be assured if t_sym % t_dts or camera fps != 0.
fn correct_negative_offsets(mut off1: f64, mut off2: f64) -> (f64, f64) {
    while off1 < 0.0 || off2 < 0.0 {
        off1 += 1.0;
        off2 += 1.0;
    }

    if (off1 - off2).abs() > 1.0 {
        warn!("Offset difference >= 1.0 alters the testsequence!");
    }

    (off1, off2)
}

// _____________________________________________________________________________________
// _____________________________________________________________________________________
// _____________________________________________________________________________________

// Previous fluid-based implementation not maintained!!!

pub fn init_air_tube_hard() -> SimulationKernel {
    info!("Initializing Simulation Kernel");

    // *"config variables" for our scenario

    let sim_time_limit = time::Duration::new(4, 0); // maximum simulation time
    let base_delta_time = time::Duration::from_micros(1500); // time step size // ! should be <= 1/480 s = approx 2 ms
    let base_delta_time = time::Duration::from_millis(2); // time step size // ! should be <= 1/480 s = approx 2 ms

    // * Injectors

    /*
    distance (translates to translation of one Inj)
    inj_amount_per_time = 0.54 liter/minute = 0.009 liter/second
    shutter_time = minimum 30ms = 0.03 s
    liter_molecule_conversion = _ (molecules/liter)

    velocity (mean) = 12.82 meter/second
    velocity (sigma) = 3 m/s
    distribution (sigma) = 1.55      (in y speed to simulate different droplet sizes and therefore air drag)

    */
    let distance: f64 = 118.0 * 1e-2;

    let inj_amount_per_time = 0.009;
    let shutter_time = time::Duration::from_millis(20);
    // for constant conversion
    let liter_molecule_conversion = 185185.1852; // approx. 50 mols per spray
                                                 //let liter_molecule_conversion = 18518.51852; // approx. 5 mols per spray

    let k = 1000; // if you want to go by mol per spray = inj_amount_base
                  //let liter_molecule_conversion = k / (inj_amount_per_time * shutter_time.as_secs_f64());

    let inj_amount_base: i64 = match k {
        0 => (inj_amount_per_time * shutter_time.as_secs_f64() * liter_molecule_conversion).floor()
            as i64,
        _ => k,
    };

    let velocity = 12.82;
    let velocity_sigma = 3.0;
    let distribution_sigma = 1.55;

    // inj in (0,0,0)
    let zero_vec = Vector3::new(0.0, 0.0, 0.0);
    let inj_0 = SprayNozzle::new(
        0,
        zero_vec,
        zero_vec,
        zero_vec,
        false,
        inj_amount_base,
        velocity,
        velocity_sigma,
        distribution_sigma,
        None,
        base_delta_time,
        shutter_time,
    );
    // inj in (0, d, 0)
    let inj_1 = SprayNozzle::new(
        1,
        Vector3::new(0.0, distance, 0.0),
        zero_vec,
        zero_vec,
        true,
        inj_amount_base,
        velocity,
        velocity_sigma,
        distribution_sigma,
        None,
        base_delta_time,
        shutter_time,
    );

    // * Modulation

    /*
     start_time (same as Bitstreamgen)
     symbol duration = shutter_time (inj) + pause_time
    */

    let pause_time = time::Duration::from_millis(30);

    let symbol_duration = shutter_time + pause_time;

    // Modulation for Inj 0
    let offset_0 = 0.0;
    let start_time_0 =
        time::Duration::from_nanos((symbol_duration.as_nanos() as f64 * offset_0).floor() as u64);
    let mod_0 = ModulationOOK::new(0, start_time_0, symbol_duration, base_delta_time);

    let offset_1 = 0.0;
    let start_time_1 =
        time::Duration::from_nanos((symbol_duration.as_nanos() as f64 * offset_1).floor() as u64);
    let mod_1 = ModulationOOK::new(1, start_time_1, symbol_duration, base_delta_time);

    // Var Bitstreamgenerators

    /*
     start_time
     repetitions
     file_path
     binary_msg -> via environment variable
    */

    let repetitions_0 = 1;
    let file_path_0 = "Msg_l";
    let bg_0 = Bitstreamgenerator::new(0, start_time_0, repetitions_0, file_path_0);

    let repetitions_1 = 1;
    let file_path_1 = "Msg_r";
    let bg_1 = Bitstreamgenerator::new(1, start_time_1, repetitions_1, file_path_1);

    // TODO: Make it useful for debug.
    // * Sensor Camera // ! not needed for sim yet, because the geometry is just handed to the next part in the toolchain

    let delta_y = 0.08 * (distance / 2.0);
    let cam_z = 0.025; // 50cm Tube
    let height = 0.05;
    let width_projected = 0.05; // ! ?
    let cam_ratio = (16.0, 9.0);
    let s_cam = SensorCamera::new(
        Vector3::new(0.0, (distance / 2.0) + delta_y, cam_z),
        height,
        width_projected,
        cam_ratio,
    );

    // * Sensor Destructing

    /*
     one left and one right to save capacity
    */

    let s_des_0 = SensorDestructing::new(None, Some((f64::MIN, -0.000001)), None);
    let s_des_1 = SensorDestructing::new(None, Some((f64::MIN, -0.000001)), None);

    // * Build kernel components
    let mut molecule_manager = MoleculeManager::new(100); //trade memory for speed - read from config how many molecules will be approx added every time step to size the vector accordingly

    let mut movement_predictor = MovementPredictor::new(None, true, None); // ! rewrite the new() because no of molecules can't possibly be known

    let mut scene_manager = SceneManager::new(); // not needed for this airbased scenario

    let mut bitstreamgen_manager = BitstreamgenManager::new();

    let mut modulation_manager = ModulationManager::new();

    let mut injector_manager = InjectorManager::new();

    let mut sensor_manager = SensorManager::new();

    // *MOLECULE MANAGER

    // *MOVEMENT PREDICTOR

    movement_predictor.set_interpolation_method(Interpolation::Linear); // * here

    // *INJECTOR MANAGER

    let inj_arr = vec![Box::new(inj_0), Box::new(inj_1)];
    injector_manager.apply_changes(inj_arr);

    // *MODULATION MANAGER

    let mod_arr = vec![Box::new(mod_0), Box::new(mod_1)];
    modulation_manager.apply_changes(mod_arr);

    // *BITSTREAMGEN MANAGER

    let bg_arr = vec![Box::new(bg_0), Box::new(bg_1)];
    bitstreamgen_manager.apply_changes(bg_arr);

    // *SENSOR MANAGER

    // TODO: change how sensors are added to the sensor_manager
    sensor_manager.add_sensor_destructing(s_des_0);
    sensor_manager.add_sensor_destructing(s_des_1);
    sensor_manager.attach_camera(s_cam);

    // *attach kernel components to simulation_kernel
    /* let mut simulation_kernel = SimulationKernel2::new(sim_time_limit, base_delta_time);

    let mut kernel_comps: Vec<Box<dyn RManager>> = vec![];
    kernel_comps.push(Box::new(molecule_manager));

    simulation_kernel.apply_changes(kernel_comps);

    let mut molman = simulation_kernel.get_kernel_comp(pgtraits::KernelCompNames::MoleculeManager); */

    let simulation_kernel = SimulationKernel::new(
        molecule_manager,
        movement_predictor,
        scene_manager,
        bitstreamgen_manager,
        modulation_manager,
        injector_manager,
        sensor_manager,
        sim_time_limit,
        base_delta_time,
    );

    simulation_kernel
}

pub fn init_tube() -> SimulationKernel {
    // *"config variables" for our scenario

    let tube_radius: f64 = 0.75E-3; //0.75mm
    let tube_length: f64 = 5.0E-2; //5cm
    let k: u32 = 20; //number of molecules
    let sim_file_path = RelativePath::new("../tube_r0.75mm_l5cm_5mlpmin_10cells/0.79");

    let sim_time_limit = time::Duration::new(2, 0); // maximum simulation time
    let base_delta_time = time::Duration::from_millis(1); // time step size
                                                          //let base_delta_time = time::Duration::from_micros(50);

    // * Build kernel components

    let mut molecule_manager = MoleculeManager::new(k as usize); //trade memory for speed - read from config how many molecules will be approx added every time step to size the vector accordingly

    let mut movement_predictor = MovementPredictor::new(None, true, None);

    let mut scene_manager = SceneManager::new();

    let mut injector_manager = InjectorManager::new();

    let mut sensor_manager = SensorManager::new();
    /*
     ~~~ To be added in the future ~~~
    let mut mesh_manager = MeshManager::new();
    */

    // *MOLECULE MANAGER

    // Build Molecules (which replaces Injectors for now)

    for mol in build_k_rand_molecules_full_circle(k, tube_radius, tube_length) {
        molecule_manager.add_molecule(mol);
    }

    molecule_manager.apply_changes();

    // *MOVEMENT PREDICTOR

    movement_predictor.set_interpolation_method(Interpolation::Euler);

    // *SCENE MANAGER

    // Build vectorfield for tube

    let vfp = VectorFieldParser {};
    let walls_patch_names = HashSet::new();
    let vf = vfp.parse_folder(sim_file_path, walls_patch_names);
    let mut vfm = VectorFieldManager::new(vf);
    vfm.generate_rtree();

    // Build tube

    let t0 = Tube::new(tube_length, tube_radius, Some(vfm)); //creates a hardcoded tube.

    scene_manager.add_object(t0);

    // *SENSOR MANAGER

    // Build Sensors - may configure this part to add or delete sensors

    let s_des = SensorDestructing::new(
        None,
        Some((tube_length - 0.01 * tube_length, tube_length)),
        None,
    );
    let s_count = SensorCounting::new(tube_length - 0.01 * tube_length, tube_length);
    let s_count2 = SensorCounting::new(
        tube_length - 0.02 * tube_length,
        tube_length - 0.01 * tube_length,
    );

    sensor_manager.add_sensor_destructing(s_des);
    sensor_manager.add_sensor_counting(s_count);
    sensor_manager.add_sensor_counting(s_count2);

    let mut bitstreamgen_manager = BitstreamgenManager::new();
    let mut modulation_manager = ModulationManager::new();

    // *attach kernel components to simulation_kernel
    let simulation_kernel = SimulationKernel::new(
        molecule_manager,
        movement_predictor,
        scene_manager,
        bitstreamgen_manager,
        modulation_manager,
        injector_manager,
        sensor_manager,
        sim_time_limit,
        base_delta_time,
    );

    simulation_kernel
}

pub fn init_ypiece() -> SimulationKernel {
    // * "config variables" for our scenario

    let origin_1 = Vector3::new(4.0E-2, 0., 2.3E-2);
    let angle_1 = Point3::new(0., 130., 0.) * glm::two_pi() / 360.;
    let origin_2 = Vector3::new(-0.4E-2, 0., -0.7E-2);
    let angle_2 = Point3::new(0., -30., 0.) * glm::two_pi() / 360.;

    let outlet_radius: f64 = 0.76E-3; //0.76mm
    let outlet_length: f64 = 5E-2; //5cm
    let k: u32 = 10; //number of molecules
    let sim_file_path =
        RelativePath::new("../y-piece_r0.76mm_bg10mlpmin_in16.6mlpmin_o5cm_bg1cm_p5cm/0.75");

    let sim_time_limit = time::Duration::new(1, 0);
    //let base_delta_time = time::Duration::from_millis(1);
    //let base_delta_time = time::Duration::from_micros(1);
    let base_delta_time = time::Duration::from_nanos(100);

    // *Build kernel components

    let mut molecule_manager = MoleculeManager::new(k as usize); //trade memory for speed - read from config how many molecules will be approx added every time step to size the vector accordingly

    let mut movement_predictor = MovementPredictor::new(None, true, None);

    let mut scene_manager = SceneManager::new();

    let mut injector_manager = InjectorManager::new();

    let mut sensor_manager = SensorManager::new();
    /*
     ~~~ To be added in the future ~~~
    let mut mesh_manager = MeshManager::new();
    */

    // *MOLECULE MANAGER

    // Build Molecules (which replaces Injectors for now)

    for mol in
        build_k_rand_molecules_y_piece(k, outlet_radius, (origin_1, angle_1), (origin_2, angle_2))
    {
        molecule_manager.add_molecule(mol);
    }

    molecule_manager.apply_changes();

    // *MOVEMENT PREDICTOR

    movement_predictor.set_interpolation_method(Interpolation::Euler);

    // *SCENE MANAGER

    // Build vectorfield for tube

    let vfp = VectorFieldParser {};
    let walls_patch_names = HashSet::new();
    let vf = vfp.parse_folder(sim_file_path, walls_patch_names);
    let mut vfm = VectorFieldManager::new(vf);
    vfm.generate_rtree();

    // ! Build ypiece is right now still constructed under Tube Object because there is no special difference yet

    let t0 = Tube::new(outlet_length, outlet_radius, Some(vfm)); //creates a hardcoded tube.

    scene_manager.add_object(t0);

    // *SENSOR MANAGER

    // Build Sensors - may configure to add or delete sensors along the outlet tube

    let s_des = SensorDestructing::new(
        None,
        Some((outlet_length - 0.01 * outlet_length, outlet_length)),
        None,
    );
    let s_count = SensorCounting::new(outlet_length - 0.01 * outlet_length, outlet_length);
    let s_count2 = SensorCounting::new(
        outlet_length - 0.05 * outlet_length,
        outlet_length - 0.04 * outlet_length,
    );

    sensor_manager.add_sensor_destructing(s_des);
    sensor_manager.add_sensor_counting(s_count);
    sensor_manager.add_sensor_counting(s_count2);

    let mut bitstreamgen_manager = BitstreamgenManager::new();
    let mut modulation_manager = ModulationManager::new();

    // *attach kernel components to simulation_kernel
    let simulation_kernel = SimulationKernel::new(
        molecule_manager,
        movement_predictor,
        scene_manager,
        bitstreamgen_manager,
        modulation_manager,
        injector_manager,
        sensor_manager,
        sim_time_limit,
        base_delta_time,
    );

    simulation_kernel
}

use rand::prelude::*;

fn build_k_rand_molecules_y_piece(
    k: u32,
    radius: f64,
    base_1: (Vector3<f64>, Point3<f64>),
    base_2: (Vector3<f64>, Point3<f64>),
) -> Vec<Molecule> {
    let mut rng = thread_rng();
    let mut mols: Vec<Molecule> = Vec::new();
    let mut r: f64;
    let mut phi: f64;

    let (origin_1, angle_1) = base_1;
    let (origin_2, angle_2) = base_2;

    let turn_matrix_x_1 = Matrix3::new(
        1.,
        0.,
        0.,
        0.,
        angle_1.x.cos(),
        -angle_1.x.sin(),
        0.,
        angle_1.x.sin(),
        angle_1.x.cos(),
    );
    let turn_matrix_y_1 = Matrix3::new(
        angle_1.y.cos(),
        0.,
        angle_1.y.sin(),
        0.,
        1.,
        0.,
        -angle_1.y.sin(),
        0.,
        angle_1.y.cos(),
    );
    let turn_matrix_z_1 = Matrix3::new(
        angle_1.z.cos(),
        -angle_1.z.sin(),
        0.,
        angle_1.z.sin(),
        angle_1.z.cos(),
        0.,
        0.,
        0.,
        1.,
    );

    let turn_matrix_x_2 = Matrix3::new(
        1.,
        0.,
        0.,
        0.,
        angle_2.x.cos(),
        -angle_2.x.sin(),
        0.,
        angle_2.x.sin(),
        angle_2.x.cos(),
    );
    let turn_matrix_y_2 = Matrix3::new(
        angle_2.y.cos(),
        0.,
        angle_2.y.sin(),
        0.,
        1.,
        0.,
        -angle_2.y.sin(),
        0.,
        angle_2.y.cos(),
    );
    let turn_matrix_z_2 = Matrix3::new(
        angle_2.z.cos(),
        -angle_2.z.sin(),
        0.,
        angle_2.z.sin(),
        angle_2.z.cos(),
        0.,
        0.,
        0.,
        1.,
    );

    let mut point: Point3<f64>;

    for i in 0..k {
        r = rng.gen_range(0.0..0.998 * radius);
        phi = rng.gen_range(0.0..glm::two_pi());

        point = r * Point3::new(phi.cos(), phi.sin(), 0.0);

        if i % 2 == 0 {
            // spawn at base 1
            point = turn_matrix_x_1 * point;
            point = turn_matrix_y_1 * point;
            point = turn_matrix_z_1 * point;
            point = point + origin_1;
        } else {
            // spawn at base 2
            point = turn_matrix_x_2 * point;
            point = turn_matrix_y_2 * point;
            point = turn_matrix_z_2 * point;
            point = point + origin_2;
        }

        mols.push(Molecule::new(point, Vector3::new(0., 0., 0.), Some(0)));
    }

    mols
}

extern crate nalgebra_glm as glm;

fn build_k_rand_molecules_full_circle(k: u32, radius: f64, length: f64) -> Vec<Molecule> {
    let mut rng = thread_rng();
    let mut r: f64;
    let mut phi: f64;
    let mut z: f64;
    let mut mols: Vec<Molecule> = Vec::new();

    for i in 0..k {
        r = rng.gen_range(0.0..0.998 * radius);
        phi = rng.gen_range(0.0..glm::two_pi());
        z = rng.gen_range(0.01 * length..0.02 * length); //define inlet zone with length of object

        mols.push(Molecule::new(
            Point3::new(r * phi.cos(), r * phi.sin(), z),
            Vector3::new(0.0f64, 0.0, 0.0),
            Some(0),
        ));
    }

    mols
}
