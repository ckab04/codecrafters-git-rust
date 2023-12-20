mod blob;

#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::process::Command;
use std::str::from_utf8;
use std::io::Write;
use crate::blob::create_blob_object;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    //println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "init" => init(),
        "cat-file" => cat_file(&args[3]),
        "hash-object" => hash_object(&args[3]),
        _ => println!("unknown command: {}", args[1])
    }
    // if args[1] == "init" {
    //    init();
    // } else if args[1] == "cat-file" {
    //     cat_file( &args[3]);
    // }
    // else {
    //     println!("unknown command: {}", args[1])
    // }
}
fn init(){
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    println!("Initialized git directory")
}

fn cat_file(cmd_line_arg: &String){
    let dir_content = fs::read_dir(".git/objects").unwrap();

    for p in dir_content {
        let string = blob::read_blob(p.unwrap(), cmd_line_arg);
        if !string.is_empty() {
            let res = Command::new("git")
                .arg("cat-file")
                .arg("-p")
                .arg(string)
                .output()
                .expect("Failed to execute the command");

            write!(std::io::stdout(), "{}", from_utf8(&res.stdout[..]).unwrap()).expect("Failed to write to stdout");
        }
    }
}

fn hash_object(file_name: &str){
    let file_content = fs::read_to_string(file_name).expect("Failed to read the file");

    //println!("File content : {file_content}");
    create_blob_object(&file_content);
}
