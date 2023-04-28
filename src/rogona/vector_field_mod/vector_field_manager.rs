/// Not needed in airbased scenario



use crate::rogona::{
    rg_attributes::pgtraits::Interpolation,
    vector_field_mod::vector_field::VectorField,
};
use nalgebra::{Point3, Vector3};
use rstar::primitives::GeomWithData;
use rstar::RTree;

type CellCenterWithID = GeomWithData<[f64; 3], usize>;
type CellCenterWithIDAndDistance = (CellCenterWithID, f64);


#[derive(Debug)]
pub struct VectorFieldManager {
    vector_field_local: VectorField,

    /*
    Vector field in local coordinates relative to the origin of the mesh.
    */
    rt: Option<RTree<CellCenterWithID>>,
    /*     Spatial data structure in the form of a rstar-tree with
    cell centre positions in scene-global coordinates. */
}

impl VectorFieldManager {
    pub fn new(vf: VectorField) -> VectorFieldManager {
        VectorFieldManager {
            vector_field_local: vf,
            rt: None,
        }
    }

    pub fn get_flow_by_position(
        &self,
        position_global: Point3<f64>,
        interpolation_type: Option<Interpolation>,

                                                 /*
                                                 :param simulation_kernel: If None, interpolation_type *must* be given!
                                                 :param position_global: A position in global coordinates.
                                                 :param interpolation_type: If None, use the interpolation method
                                                     specified in the SimulationKernel.
                                                 :return: The flow vector in the local coordinate system (i.e.,
                                                     you might want to apply some rotation).
                                                 */
    ) -> Option<Vector3<f64>> {
        // transformation ignored because of simple one tube scenario (no scenes)
        // implement linear_interporalion using distances as weights

        if interpolation_type.unwrap() == Interpolation::Euler {
            let position_global = [position_global.x, position_global.y, position_global.z];
            let nns: Vec<CellCenterWithID> = self.get_nearest_neighbors(position_global);           //returns in the geometry used in RTree
            //println!("length of nns : {}",nns.len());
            //println!("nns[0] : {:?}",nns[0]);

            let mut nns_vec: Vec<(Vector3<f64>, usize)> = Vec::new();
            for n in nns.iter() {
                let y = n.geom();
                nns_vec.push((Vector3::new(y[0], y[1], y[2]), n.data));
            }

            let mut nns = nns_vec;

            //reimplement if different interpolation is used where distance is important

            let mut cell_id: usize;
            let n_count: u64 = nns.len().try_into().unwrap();
            let mut integrated_flow: Vector3<f64> = Vector3::new(0.0f64, 0.0, 0.0);
            loop {
                let nn = nns.pop();
                match nn {
                    Some((_, id)) => {
                        cell_id = id;
                    }
                    None => break,
                };

                integrated_flow += self.vector_field_local.get_flow(cell_id);   //addition for arith mean
            }
            integrated_flow = integrated_flow.unscale(n_count as f64);      //division for arith mean

            Some(integrated_flow)
        } else {
            None
        }
    }

    pub fn get_cell_id_by_position(&self, position_local: Point3<f64>) -> Option<usize> {
        let position_local = [position_local.x, position_local.y, position_local.z];

        Some(
            self.rt
                .as_ref()
                .unwrap()
                .nearest_neighbor(&position_local)
                .unwrap()
                .data,
        )
    }
    pub fn get_nearest_neighbors(
        &self,
        position_global: [f64; 3],
    ) -> Vec<CellCenterWithID> {
        let k = 1; //change for more precise interpolation
                   // returns a hash map of distances and flow for the nearest neighbors of a given position
        let nns = self
            .rt
            .as_ref()
            .unwrap()
            .nearest_neighbor_iter(&position_global)
            .take(k)
            .cloned()
            .collect::<Vec<_>>();
        nns
    }

    pub fn generate_rtree(&mut self) {
        let centers = self.vector_field_local.get_cell_centers();

        let x: Vec<CellCenterWithID> = centers
            .enumerate()
            .map(|(cell_id, pt)| CellCenterWithID::new(pt.coords.into(), cell_id))
            .collect();
        let rt = RTree::bulk_load(x);

        self.rt = Some(rt);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use nalgebra::Point3;
    use relative_path::RelativePath;

    use crate::rogona::vector_field_mod::{vector_field_manager, vector_field_parser};

    #[test]
    fn test_interpolation() {
        let vfp = vector_field_parser::VectorFieldParser {};
        let walls_patch_names = HashSet::new();
        let vf = vfp.parse_folder(
            RelativePath::new("tube_r0.75mm_l5cm_5mlpmin_10cells/0.79"),
            walls_patch_names,
        );
        let mut vfm = vector_field_manager::VectorFieldManager::new(vf);
        vfm.generate_rtree();
        let flow_vector = vfm.get_flow_by_position(
            Point3::new(-0.00003, 0.00015, 0.0000750751),
            Some(crate::rogona::rg_attributes::pgtraits::Interpolation::Euler),
        );

        println!("Printing flow_vector for 0 position : {:?}", flow_vector);
    }
}
