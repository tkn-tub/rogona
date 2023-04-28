// not used in this scenario

























/* use linfa::DatasetBase;
use linfa::traits::{Fit, FitWith, Predict};
use linfa_clustering::{KMeansParams, KMeans, IncrKMeansError};
//use linfa_datasets::generate;
use ndarray::{Axis, array, s};
//use approx::assert_abs_diff_eq;

fn kmeans_test () {
	/* // Our random number generator, seeded for reproducibility
	let seed = 42;
	let mut rng = Xoshiro256Plus::seed_from_u64(seed); */
	
	// `expected_centroids` has shape `(n_centroids, n_features)`
	// i.e. three points in the 2-dimensional plane
	//let expected_centroids = array![[0., 1.], [-10., 20.], [-1., 10.]];

	// TODO calculate an estimate for the 4 centroids depending on parameters (e.g. geometry and spray time) of the simulation

	let expected_centroids = array![[0., 0.], [4., 0.], [6., 0.], [10., 0.]];	// the 4 centroids for the expected LIVs

	// Let's generate a synthetic dataset: three blobs of observations
	// (100 points each) centered around our `expected_centroids`

	
	/* let data = generate::blobs(100, &expected_centroids, &mut rng);
	let n_clusters = expected_centroids.len_of(Axis(0)); */
	
	// ! Take the first 400 entries in the received data
	// ! get data -> array of f64 LIVs (all lives from the tool filtered after synchronisation)
	let n_clusters = 4;
	let data = array![0., 3., 4., 4.1].zip(|x| [x, 0.]); // TODO: get correct array

	// Standard K-means
	{
		let observations = DatasetBase::from(data.clone());
		// Let's configure and run our K-means algorithm
		// We use the builder pattern to specify the hyperparameters
		// `n_clusters` is the only mandatory parameter.
		// If you don't specify the others (e.g. `n_runs`, `tolerance`, `max_n_iterations`)
		// default values will be used.
		let model = KMeans::params_with_rng(n_clusters, rng.clone())
        .tolerance(1e-2)
        .fit(&observations)
        .expect("KMeans fitted");
		//let model = KMeans::params(n_clusters);
			//.tolerance(1e-2);
			//.fit(&observations)
			//.expect("KMeans fitted");
	
		// Once we found our set of centroids, we can also assign new points to the nearest cluster
		let new_observation = DatasetBase::from(array![[-9., 20.5]]);
		// Predict returns the **index** of the nearest cluster
		let dataset = model.predict(new_observation);
		// We can retrieve the actual centroid of the closest cluster using `.centroids()`
		let closest_centroid = &model.centroids().index_axis(Axis(0), dataset.targets()[0]);
		assert_abs_diff_eq!(closest_centroid.to_owned(), &array![-10., 20.], epsilon = 1e-1);
	}
	
	// Incremental K-means
	{
		let batch_size = 100;
		// Shuffling the dataset is one way of ensuring that the batches contain random points from
		// the dataset, which is required for the algorithm to work properly
		let observations = DatasetBase::from(data.clone()).shuffle(&mut rng);
	
		let n_clusters = expected_centroids.nrows();
		let clf = KMeans::params_with_rng(n_clusters, rng.clone()).tolerance(1e-3);
	
		// Repeatedly run fit_with on every batch in the dataset until we have converged
		let model = observations
			.sample_chunks(batch_size)
			.cycle()
			.try_fold(None, |current, batch| {
				match clf.fit_with(current, &batch) {
					// Early stop condition for the kmeans loop
					Ok(model) => Err(model),
					// Continue running if not converged
					Err(IncrKMeansError::NotConverged(model)) => Ok(Some(model)),
					Err(err) => panic!("unexpected kmeans error: {}", err),
				}
			})
			.unwrap_err();
	
		let new_observation = DatasetBase::from(array![[-9., 20.5]]);
		let dataset = model.predict(new_observation);
		let closest_centroid = &model.centroids().index_axis(Axis(0), dataset.targets()[0]);
		assert_abs_diff_eq!(closest_centroid.to_owned(), &array![-10., 20.], epsilon = 1e-1);
	}

} */

