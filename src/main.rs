use ndarray::Array;

fn main() {
    println!("Hello, world!");

    let arr1 = Array::from_elem((32, 47), 0.5);

    let arr2 = Array::from_elem((47, 59), 4.);

    let arr3 = arr1.dot(&arr2);

    println!("{}", arr3);
}
