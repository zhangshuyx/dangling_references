fn main() {
    //let reference_to_nothing = dangle();
    let string = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
