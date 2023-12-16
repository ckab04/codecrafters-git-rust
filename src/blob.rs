use std::fs;
use std::fs::DirEntry;

// The return is the full file name (including the directory name)
pub fn read_blob(content_obj: DirEntry, arg: &String) -> String{
    let v = content_obj.path().to_str().unwrap().to_string();
    let content_folder = fs::read_dir(v).unwrap();
    let val: Vec<String> = content_folder.into_iter().map(|x | {
        let v = x.unwrap().path().to_str().unwrap().to_string();
        let v = &v[13..];
        v.replace(&['/'], "").to_string()
    }).collect();

    let result = val.get(0);
    if result.is_some() && (result.unwrap() == arg)
    {
        return result.unwrap().to_string();
    }
    String::new()

   }