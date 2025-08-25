use rexify::*;

fn main() {
    println!("Hello, world!");

    let rex = Rex::new(vec![
        Box::new(Number::new()),
        Box::new(Repeat::new(AnyChar::new())),
    ]);
    let text = "asdija123102aaanejn";
    println!("{:?}", rex.find(text));
    println!("{:?}", rex.capture(&text[6..]))
}
