// UNIT STRUCT
struct Unit;

// TUPLE STRUCT
struct Pair(i32,i32);

// A STRUCT WITH TWO FIELD
struct Point {
    x : f32,
    y : f32,
}

struct Rectangle {
    topLeft : Point,
    bottomRight :Point,
}
// DOUBT
fn areaRectangle (rectangle :Rectangle) -> f32 {
    let Rectangle {topLeft:Length {},bottomRight:width}= rectangle;

}

fn main() {
    let pair = Pair(1,2);
    println!("{:?}  and {:?} ", pair.0,pair.2);

    let point1 = Point {
        x : 2.2,
        y:4.4,
    };
    // DESTRUCTURE
    let Pair(no1,no2) = pair;




}
