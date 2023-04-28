/// Not needed in airbased scenario


use na::Vector3;
use nalgebra as na;

use openfoamparser as ofp;

use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Not;
use std::panic;
use std::path::Path;
use relative_path::RelativePath;

use crate::rogona::{
    vector_field_mod::{vector_field::VectorField, face::Face}
};

pub struct VectorFieldParser {}

impl VectorFieldParser {
    pub fn new() {
        // pass
    }

    pub fn parse_folder(
        self,
        folder: &RelativePath,
        walls_patch_names: HashSet<&str>,
        //dummy_boundary_points: DummyBoundaryPointsVariant
    ) -> VectorField {
        // Check whether to use default walls_patch_names
        let mut walls_patch_names = walls_patch_names;
        if walls_patch_names.is_empty() {
            walls_patch_names = HashSet::from(["walls", "yConnectorPatch", "tubePatch"]);
        }

        let folder = folder.to_path(".");

        // Check if folder exists
        if folder.is_dir().not() {
            panic!(
                "The OpenFOAM simulation result folder {:?} does not exist.\n
                    Are you sure your path configuration is correct?",
                folder.as_os_str()
            );
        }

        // Check if cell centre file exists
        let centre_file = folder.join(Path::new("C"));
        if centre_file.is_file().not() {
            panic!(
                "The OpenFOAM simulation result folder {:?} does not contain cell centres.\n
                    Run 'postProcess -func writeCellCentres' to generate them.",
                folder.as_os_str()
            );
        }

        // Parse flow
        let flow: Vec<Vector3<f64>> =
            ofp::parse_internal_field(folder.join(Path::new("U")), |s| ofp::parse_vector3(s))
                .expect("Could not parse flow.");

        // Create foam mesh and read cell centres
        let mut mesh =
            ofp::FoamMesh::new(folder.join(Path::new(".."))).expect("Could not create foam mesh.");
        mesh.read_cell_centers(folder.join(Path::new("C")))
            .expect("Could not read cell centres.");
        let centres = mesh.cell_centers.to_owned().unwrap();
        let centres_len = centres.len();

        // Find boundary cells
        let mut is_boundary: Vec<bool> = vec![false; centres_len];
        let mut boundary_cells: Vec<usize> = Vec::new();
        for patch_name in &walls_patch_names {
            let mut cells = mesh.boundary_cells(patch_name);
            boundary_cells.append(&mut cells);
        }

        // mark boundary cells
        if boundary_cells.len() > 0 {
            for centre_id in boundary_cells {
                is_boundary[centre_id] = true;
            }
        } else {
            panic!("Are you sure this mesh has no boundary cells?");
        }

        // Find boundary faces
        let mut boundary_faces: HashMap<usize, Face> = HashMap::new();

        for (face, face_id) in mesh.faces.iter().zip(0..mesh.faces.len()) {
            for patch_name in &walls_patch_names {
                let patch_name = Option::from(String::from(*patch_name));
                if mesh.is_face_on_boundary(face_id, patch_name) == true {
                    let cell_id = mesh.owners[face_id];
                    let mut face_points = Vec::new();

                    for point_id in face {
                        face_points.push(mesh.points[*point_id]);
                    }
                    // Calculate normal
                    let position = face_points[0];
                    let plane_vector_1 = Vector3::from(face_points[1] - position);
                    let plane_vector_2 = Vector3::from(face_points[2] - position);
                    let mut normal = plane_vector_1.cross(&plane_vector_2);
                    normal = normal / normal.norm();

                    // Temporary face
                    let temporary_face = Face::new(face_id, position, normal, 0.0);

                    // Calculate distance from cell center to face
                    let distance_to_centre = VectorFieldParser::point_distance_to_plane(
                        centres[cell_id],
                        temporary_face,
                    );

                    let face_with_distance =
                        Face::new(face_id, position, normal, distance_to_centre);

                    // Save into hashmap
                    boundary_faces.insert(cell_id, face_with_distance);
                };
            }
        }
        VectorField {
            cell_centres: centres,
            flow,
            is_boundary,
            boundary_faces,
        }
    }

    fn point_distance_to_plane(position: na::Point3<f64>, face: Face) -> f64 {
        let pos_diff = position - face.position;
        face.normalized_normal.dot(&pos_diff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_parser_and_parse_test_case() {
        let vfp = VectorFieldParser {};
        let walls_patch_names = HashSet::new();
        let field = vfp.parse_folder(
            //"/home/hamza/openfoamcase/tube_r0.75mm_l5cm_5mlpmin_10cells/0.79",
            RelativePath::new("../tube_r0.75mm_l5cm_5mlpmin_10cells/0.79"),
            walls_patch_names,
        );
        println!("Printing samples of parsed field.. \n\n");
        println!("Cell centres (42): ");
        println!("{}\n", field.cell_centres[42]);
        println!("Flow (100): ");
        println!("{}\n", field.flow[100]);
        println!("Is boundary (555): ");
        println!("{}\n", field.is_boundary[555]);
        println!("Boundary faces (two from 1): ");
        let mut keys = field.boundary_faces.keys();
        let mut vals = field.boundary_faces.values();
        for _i in 1..3 {
            println!("{:#?}", keys.next().unwrap());
            println!("{:#?}", vals.next().unwrap());
        }
        println!("\n\n");
    }
}
