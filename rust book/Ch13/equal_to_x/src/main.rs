fn main() {
    let x = vec![1, 2, 3];

    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y))
}
