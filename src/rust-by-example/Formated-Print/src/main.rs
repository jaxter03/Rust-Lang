fn book_example() {
    println!("{} days", 31); // i32 consider

    // for any other
    println!("{} days",31i64);  // var + type

    // Positioned argument
    //1
    println!("{1}, this is {0}, and this is {1}","Alice","Bob");
    //2
    println!("{first} and {second} added then {third}",first = "hell",second="second",third="third");
    println!("{} of {:b} people know binary, the other half doesn't", 5, 13); //  ':' for binary
    // Right Align
    println!("{number:>width$}", number=1, width=2);
    //right Align with padding
    println!("{number:>0width$}",number = 3,width = 4);

}

fn precision_example() {
  // Precision bt argument
    println!("{0:.0}",1.35555);

    // Precision by variable

    println!("Hello first arg {0}, and second {1:.2$}",1,4.121,5);


}
#[derive(Debug)]
struct Structure(i32);
#[derive(Debug)]
struct Deep(Structure);

fn debug_eg() {

    println!("{0:#?}{1:#?}",Structure(3),Deep(Structure(4)));

}

fn main() {
//    book_example()
//    precision_example();
    debug_eg();
}