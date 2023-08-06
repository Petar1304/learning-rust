
pub mod algorithms {

    pub fn sort_vector() {
        let mut vec = vec![1, 5, 10, 2, 15];

        println!("Sorting vector: {:?}", vec);
        vec.sort();
        println!("Sorted vector: {:?}", vec);

        assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    }

    pub fn sort_floats() {
        let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];

        vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("Sorted vector of floats: {:?}", vec);
        assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
    }
}