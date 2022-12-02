pub fn create_initialized_vector_with_fixed_capacity<T: Default>(capacity: usize) -> Vec<T> {
    let mut vector = Vec::with_capacity(capacity);
    for _ in 0..capacity {
        vector.push(T::default());
    }

    vector
}