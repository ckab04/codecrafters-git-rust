mod blob;

#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::process::Command;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
        println!("Initialized git directory")
    } else if args[1] == "cat-file"{
       let dir_content = fs::read_dir(".git/objects").unwrap();

        for p in dir_content{
            let string = blob::read_blob(p.unwrap(), &args[3]);
            if !string.is_empty(){
                let mut command = Command::new("git")
                    .arg("cat-file")
                    .arg("-p")
                    .arg(string)
                    .spawn()
                    .expect("Failed to read the command")
                    .stdout.expect("Failed to read the command on the stdout");

                //println!("{}", string);
            }
        }
    }

    else {
        println!("unknown command: {}", args[1])
    }
}
