mod blob;

#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

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
    } else if args[1] == "cat-file -p"{
       let dir_content = fs::read_dir(".git/objects").unwrap();

        for p in dir_content{
            let v = p.unwrap().path().to_str().unwrap();
            let file = v.replace(&['/', '.'], "");
            if args[2].is_empty() && args[2] == file{
                println!("{file}");
                break;
            }
        }
    }

    else {
        println!("unknown command: {}", args[1])
    }
}
