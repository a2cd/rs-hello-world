

fn main() {
    // let x = String::from("abc");
    // let z;
    // {
    //     let y = String::from("abcd");
    //     z = longest(x.as_str(), y.as_str());
    // }
    // println!("longest={z}");
    // failed_borrow();
}

fn longest<'a>(x:&'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

