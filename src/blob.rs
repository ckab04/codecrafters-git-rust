use std::fs;
use std::fs::DirEntry;

// The return is the full file name (including the directory name)
pub fn read_blob(content_obj: DirEntry, arg: &String) -> String{
    let v = content_obj.unwrap().path().to_str().unwrap();

    let content_folder = fs::read_dir(v).unwrap();

    let val: Vec<String> = content_folder.into_iter().map(|x | {
        let v = x.unwrap().path().to_str().unwrap();
        v.replace(&['/', '.'], "").to_string();
    }).collect();

    if val.is_empty(){
        String::new()
    }

    let result = val.get(0).expect("Unable to get the path").into_string();
    result

   }