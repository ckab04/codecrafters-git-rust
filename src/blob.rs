use std::fs;
use std::fs::{DirEntry, File};
use std::io::Write;
use sha1::{Digest, Sha1};

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

pub fn create_blob_object(file_content: Vec<u8>){
    //print!("File content : {file_content}");
    //println!();
    //let file_c = file_content.as_bytes();
     let mut header = format!("blob0x20{}", file_content.len()).into_bytes();
    //header.push(b'\0');
    let content = [&header[..], &file_content[..]].concat();
    let mut compressed = Vec::new();
    let mut compressor = flate2::write::ZlibEncoder::new(&mut compressed, flate2::Compression::default());
    compressor.write_all(&content).expect("Failed to write compressed content");
    compressor.finish().expect("Compression has not finished properly");
    let mut hasher  = Sha1::new();
    //hasher.update(file_content.as_bytes());
    hasher.update(&content);
    let result = hasher.finalize();
    let encoded_result = hex::encode(result);
    let folder_name = &encoded_result[0..2];
    let file_name = &encoded_result[2..];
    let folder_to_create = format!("{}/{}", ".git/objects/", folder_name);
    if fs::create_dir_all(&folder_to_create).is_ok(){
        let file_to_create = format!("{}/{}", folder_to_create, file_name);
        let _ = File::create(file_to_create).expect("Unable to create a file");
    }


    print!("{encoded_result}");
    //println!("Second : {:?}", result[..]);
}