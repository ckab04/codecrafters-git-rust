use std::fs;
use std::fs::{DirEntry, File};
use sha1::{Digest, Sha1};
use crate::cat_file;

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

pub fn create_blob_object(file_content: &str){
    //print!("File content : {file_content}");
    //println!();
    let mut hasher  = Sha1::new();
    hasher.update(file_content.as_bytes());
    let result = hasher.finalize();
    let encoded_result = hex::encode(result);
    let folder_name = &encoded_result[0..2];
    let file_name = &encoded_result[2..];
    let folder_to_create = format!("{}/{}", ".git/objects/", folder_name);
    if fs::create_dir(&folder_to_create).is_ok(){
        let file_to_create = format!("{}/{}", folder_to_create, file_name);
        let _ = File::create(file_to_create).expect("Unable to create a file");
    }

    eprintln!("Test : {:?}", cat_file(&encoded_result));
    print!("{encoded_result}");
    //println!("Second : {:?}", result[..]);
}