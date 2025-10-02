fn main() {
    // In rust we have arrays, slices, and vectors
    // an array is a fixed size collection of elements of the same type,
    // so it is stored on the stack. Should be faster to access than a vector
    let arr = [1, 2, 3, 4, 5];
    // it includes the type and size of the array
    let arr_len = arr.len();
    println!("Array: {:?}, Length: {}", arr, arr_len);

    // a slice is a view into a contiguous sequence of elements in an array or vector
    let slice = &arr[1..4];
    println!("Slice: {:?}", slice);

    // a vector is a dynamic array that can grow or shrink in size
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    vec.push(5);
    println!("Vector: {:?}", vec);

    // you can also create a vector with new
    let mut vec2: Vec<i32> = Vec::new();
    vec2.push(10);
    vec2.push(20);
    println!("Vector 2: {:?}", vec2);

}
