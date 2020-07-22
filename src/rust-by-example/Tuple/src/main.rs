fn reverse (pair : (i32,bool)) -> (bool,i32) {
    let (no,boole) = pair;
    (boole,no)
}

fn transpose (matirx : (i32,i32,i32,i32)) -> (i32,i32,i32,i32) {
    let (ele1,ele2,ele3,ele4) = matrix;
    (ele1,ele3,ele2,ele4)
}

fn tupleDriver() {
    // Small Tuples are printable
    // While long tuples are not printable
    let no : (i32,bool) = (5,true);

    // Tuple with single element
    let single_element_tuple = (4,);
    println!("{:?}",single_element_tuple);

    let x = reverse(no);
}

fn main() {
    tupleDriver();
    println!("Hello, world!");

}


