pub fn create_initialized_vector_with_fixed_capacity<T: Default>(capacity: usize) -> Vec<T> {
    let mut vector = Vec::with_capacity(capacity);
    for _ in 0..capacity {
        vector.push(T::default());
    }

    vector
}

pub fn initialize_string_array<const SIZE: usize>() -> [String; SIZE] {

    unsafe {
        let mut arr: [String; SIZE] = std::mem::uninitialized();
        // let mut arr = std::mem::MaybeUninit::<String>::uninit_array::<GROUP_SIZE>();
        for item in &mut arr[..] {
            std::ptr::write(item, String::new());
        }
        arr
    }
}