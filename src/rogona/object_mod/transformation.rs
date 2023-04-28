/// TODO: [Future Work] implement Transformation for a more versatile approach on Injectors and Sensors.
/// For now a +/- y is sufficient



use nalgebra::{Matrix3, Vector3, Matrix4};
use ndarray::{arr2, ArrayBase, OwnedRepr, Dim};

#[derive(Debug)]
pub struct Transformation {

    translation: Vector3<f64>,
    rotation: Vector3<f64>,
    scaling: Vector3<f64>,

	translation_matrix: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
    rotation_matrix: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
    scaling_matrix: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,

    matrix: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
	inverse_matrix: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,

    direction_matrix: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
	inverse_direction_matrix: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,

    rotation_order: RotationOrder,
	set_from_matrix: bool
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum RotationOrder {
    XYZ,
    XZY,
    YXZ,
    YZX,
    ZXY,
    ZYX,
}

impl Transformation {
    /* pub fn new(
        translation: Vector3<f64>,
        rotation: Vector3<f64>,
        scaling: Vector3<f64>,
        matrix: Option<Vector3<f64>>,
        direction_matrix: Option<Vector3<f64>>,
        rotation_order: RotationOrder,
    ) -> Transformation {

		match (matrix, direction_matrix) {
			(Some(matrix_), Some(direction_matrix_)) => {
				Transformation {
					translation: (),
            		rotation: (),
            		scaling: (),
            		matrix: Some(matrix_),
            		direction_matrix: Some(direction_matrix_),
            		rotation_order,
				}
			}
		}

        Transformation {
            translation: (),
            rotation: (),
            scaling: (),
            matrix: (),
            direction_matrix: (),
            rotation_order: (),
        }
    } */

	// Directly translated from Pogona
	fn update_matrix(&mut self){
        // Scaling is applied first, then rotation, then translation!
        self.matrix = self.translation_matrix.dot(&self.rotation_matrix.dot(&self.scaling_matrix));
        //self.inverse_matrix = self.matrix.try_inverse();
        self.direction_matrix = self.rotation_matrix.dot(&self.scaling_matrix);
        //self.inverse_direction_matrix = self.direction_matrix.inv();

	}




}




// Directly translated from Pogona
fn generate_translation_matrix(translation_vector: Vector3<f64>) -> ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> {
	arr2(&[
		[1.0, 0.0, 0.0, translation_vector[0]],
		[0.0, 1.0, 0.0, translation_vector[1]],
		[0.0, 0.0, 1.0, translation_vector[2]],
		[0.0, 0.0, 0.0, 1.0]
		])
}

// Directly translated from Pogona
fn generate_scaling_matrix(scaling_vector: Vector3<f64>) -> ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>{
	arr2(&[
		[scaling_vector[0], 0.0, 0.0, 0.0],
		[0.0, scaling_vector[1], 0.0, 0.0],
		[0.0, 0.0, scaling_vector[2], 0.0],
		[0.0, 0.0, 0.0, 1.0]
	])

}

// Directly translated from Pogona
fn generate_rotation_matrix(
	rotation_vector: Vector3<f64>,
	order: RotationOrder
) -> ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> {

	let rx = arr2(&[
		[1.0, 0.0, 0.0, 0.0],
		[0.0, rotation_vector[0].cos(), -rotation_vector[0].sin(), 0.0],
		[0.0, rotation_vector[0].sin(), rotation_vector[0].cos(), 0.0],
		[0.0, 0.0, 0.0, 1.0]
	]);

	let ry = arr2(&[
		[rotation_vector[1].cos(), 0.0, rotation_vector[1].sin(), 0.0],
		[0.0, 1.0, 0.0, 0.0],
		[-rotation_vector[1].sin(), 0.0, rotation_vector[1].cos(), 0.0],
		[0.0, 0.0, 0.0, 1.0]
	]);
	
	let rz = arr2(&[
		[rotation_vector[2].cos(), -rotation_vector[2].sin(), 0.0, 0.0],
		[rotation_vector[2].sin(), rotation_vector[2].cos(), 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0]
	]);

	// Seems like we have to multiply the matrices in reverse order
	// to match Blender's definition:

	if order == RotationOrder::XYZ {
		return rz.dot(&ry.dot(&rx))
	}
	if order == RotationOrder::YXZ {
		return rz.dot(&rx.dot(&ry))
	}
	if order == RotationOrder::XZY {
		return ry.dot(&rz.dot(&rx))
	}
	if order == RotationOrder::ZXY {
		return ry.dot(&rx.dot(&rz))
	}
	if order == RotationOrder::ZYX {
		return rx.dot(&ry.dot(&rz))
	}
	if order == RotationOrder::YZX {
		return rx.dot(&rz.dot(&ry))
	}
	panic!("Invalid order {:?}", order);
}


/**
 * return: rotation mat does not do anything at the moment so it is just an Option and might be added later
	Decompose a given transformation matrix into translation vector,
	rotation matrix, and scaling vector.
	This is not guaranteed to work for transformation matrices
	that are not coming from our Transformation class.
	# With help from https://math.stackexchange.com/a/1463487

	Directly translated from Pogona
 */
fn decompose_matrix(
	mat: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>
) -> (Vector3<f64>, Option<ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>>, Vector3<f64>) {

	
	// Translation: last column
	let translation_vector = mat.column(3).to_vec();
	// ! why 4 and how do i transform this?? for now: just discard the last value
	let translation_vector = Vector3::new(translation_vector[0],translation_vector[1],translation_vector[2]);

	// Scale: length of the first three column vectors
	let scale_vector = Vector3::new(norm_column(&mat, 0), norm_column(&mat, 1), norm_column(&mat, 2));	
	

	//Rotation matrix:
	let rotation_mat = None;
	/* rotation_mat = np.concatenate(
		(
			[
				np.append(np.true_divide(mat[:3, i], scale[i]), 0)
				for i in range(3)
			],
			[[0, 0, 0, 1]],
		),
		axis=0
	).T  # TODO: to vector */

	(translation_vector, rotation_mat, scale_vector)

}

fn norm_column(mat: &ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, index: usize) -> f64 {
	(mat.column(index).dot(&mat.column(index))).sqrt()	
}