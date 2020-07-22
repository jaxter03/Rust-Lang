enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),

}
use crate::WebEvent::*;
fn matchFn (event : WebEvent) {
    match event {
        PageLoad => println!("Page Loaded"),
        PageUnload => println!("Page Unload"),
        KeyPress(char) => println!("{} character pressed",char),
        Paste(str)=> println!("{} is coied string", str),

    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pastedString = WebEvent::Paste("My name is Krishna".to_owned()); //WHY TO OWNED

 }
