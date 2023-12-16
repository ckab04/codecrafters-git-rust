use std::fs;
use std::fs::DirEntry;
use std::io::Write;
use flate2::Compression;
use flate2::read::ZlibEncoder;

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

pub fn create_blob_object(file_name: &str){
    println!("Is this working ?");
    let content = fs::read_to_string(file_name).expect("Failed to read the file");
    let mut comp = ZlibEncoder::new(Vec::new(), Compression::default());
    comp.write_all(content.as_bytes()).expect("Failed to compress");
    let compressed = comp.finish().unwrap();
    println!("{:?}", compressed);
}