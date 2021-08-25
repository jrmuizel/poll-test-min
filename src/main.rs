use std::fs::File;
fn main() {
    use isahc::{prelude::*};

    let mut res = Request::get("https://speed.hetzner.de/1GB.bin").metrics(true).body(()).unwrap().send();
    if let Ok(ref mut response) = &mut res {
        println!("{:?}", response);

        let mut f = File::create("out.bin").unwrap();


        response.copy_to(&mut f).unwrap();
    }
}
