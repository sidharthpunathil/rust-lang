use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(c) => panic!("error creating file {:?}", c),
                },
                other_error => {
                    panic!("Error with file {:?}", error)
                }
            }
    };
}
