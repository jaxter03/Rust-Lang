fn main() {
    // Array is of fixed size
    let arr1 : [i32;4] = [1,2,3,4];
    let slice : &[i32]= &arr1[1..3];
    println!("{:?}",slice);

}
