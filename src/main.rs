use ndarray::Array;

fn matmul(ndarray: &ndarray::Array2<f32>, ndarray2: &ndarray::Array2<f32>) -> ndarray::Array2<f32> {
    let a = ndarray.dot(ndarray2);
    return a;
}

fn main() {
    println!("Hello, world!");

    let arr1 = Array::from_elem((32, 47), 0.5f32);
    let arr2 = Array::from_elem((47, 59), 4.0f32);

    let arr3 = matmul(&arr1, &arr2);

    println!("{}", arr3);
}
