use std::fs;
use std::io::Error;
use std::path::Path;

pub fn read_file(file: &str) {
    let result: Result<Vec<u8>, Error> = fs::read(file);
    // let convert_bytes_to_string: impl Fn(String, &u8) -> String = |mut a: String, v: &u8| {
    //     let new_char: char = char::from(v);
    //     a.push(new_char);
    //     return a;
    // };

    let convert_bytes_to_string: Box<dyn Fn(String, &u8) -> String> =
        Box::new(|mut a: String, v: &u8| {
            let new_char: char = char::from(*v);
            a.push(new_char);
            a
        });
    if result.is_ok() {
        println!(
            "Data Found is {}.",
            result
                .ok()
                .unwrap()
                .iter()
                .fold(String::from(""), convert_bytes_to_string)
        );
    }
}

pub fn remove_file(path: &str) {
    fs::remove_file(path).unwrap();
}
pub fn remove_dir(path: &str) {
    fs::remove_dir_all(path).unwrap();
}
pub fn create_random_files() {
    let path: &str = "./data/file1.txt";
    let text: &str = "Heavy Salary";
    fs::write(path, text).unwrap();
    let path2: &str = "./data/file2.txt";
    let text2: &str = "Heavy Salary 2";
    fs::write(path2, text2).unwrap();
}
pub fn create_test_dir() {
    let path: &str = "./data";
    let my_path: &Path = std::path::Path::new(path);
    if my_path.exists() {
        println!("Haa haa Nice try, it already exists");
        return;
    }
    let create_dir_result: Result<(), Error> = fs::create_dir(path);
    if create_dir_result.is_ok() {
        println!("success");
    } else {
        println!("bad : {:?}", create_dir_result.err().unwrap());
    }
}
fn main() {
    read_file("./data/file1.txt");
}
