#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate directories;
extern crate whoami;
use directories::{BaseDirs};
use whoami::realname;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      create_parent_dir, 
      return_list_of_notes, 
      delete_folder,
      get_json,
      write_name_to_json,
      create_folder,
      return_list_of_notes_from_array_path
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// //return the operating system
// pub fn get_os() -> String {
//   let os = std::env::consts::OS;
//   return String::from(os);
// }

#[tauri::command]
fn create_parent_dir() {
  println!("RS: Creating Parent dir");
  if let Some(dir) = BaseDirs::new() {
    let the_path = Path::new(&dir.data_dir()).join("NoteAIO");
    _ = fs::create_dir_all(the_path);

    let child_path = Path::new(&dir.data_dir()).join("NoteAIO").join("notes");
    _ = fs::create_dir_all(child_path);

    let child_path_2 = Path::new(&dir.data_dir()).join("NoteAIO").join("config");
    _ = fs::create_dir_all(child_path_2);

    let i_hate_rust = Path::new(&dir.data_dir()).join("NoteAIO").join("config").join("config.json");
    let exists = i_hate_rust.exists();
    if exists == false {
      //create a new json object, set name property equal to the realname function
      let mut config = serde_json::Map::new();
      config.insert("name".to_string(), serde_json::Value::String(realname()));
      //write the json object to the file
      let mut file = fs::File::create(i_hate_rust).unwrap();
      let json = serde_json::to_string(&config).unwrap();
      file.write_all(json.as_bytes()).unwrap();
    }
  }
}

#[tauri::command]
fn delete_folder(folder_name: String) {
  println!("RS: Deleting {:?}", folder_name);
  if let Some(dir) = BaseDirs::new() {
    let the_path = Path::new(&dir.data_dir()).join("NoteAIO").join("notes").join(&folder_name);
    _ = fs::remove_dir_all(the_path);
  }
}

#[tauri::command]
fn return_list_of_notes(folder_name: String) -> Vec<String> {
  println!("RS: Returning Contents Of {:?}", folder_name);
  let mut list_of_notes = vec![];
  if let Some(dir) = BaseDirs::new() {
    if folder_name.is_empty() {
      let the_path = Path::new(&dir.data_dir()).join("NoteAIO").join("notes");      
      for entry in fs::read_dir(the_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        list_of_notes.push(file_name.to_string());
      }
    } else {
      let the_path = Path::new(&dir.data_dir()).join("NoteAIO").join("notes").join(&folder_name);
      for entry in fs::read_dir(the_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        list_of_notes.push(file_name.to_string());
      }
    }
  }
  list_of_notes.into()
}

#[tauri::command]
fn return_list_of_notes_from_array_path (folder_path: Vec<String>) -> Vec<String> {
  println!("RS: Returning Contents Of {:?}", folder_path);
  let mut list_of_notes = vec![];
  if let Some(dir) = BaseDirs::new() {
    let mut the_path = Path::new(&dir.data_dir()).join("NoteAIO").join("notes");
    for path in folder_path {
      the_path = the_path.join(&path);
    }
    for entry in fs::read_dir(the_path).unwrap() {
      let entry = entry.unwrap();
      let path = entry.path();
      let file_name = path.file_name().unwrap().to_str().unwrap();
      list_of_notes.push(file_name.to_string());
    }
  }
  list_of_notes.into()
}

#[tauri::command]
fn get_json() -> String {
  println!("RS: Getting Name");
  if let Some(dir) = BaseDirs::new() {
    let the_path = Path::new(&dir.data_dir()).join("NoteAIO").join("config").join("config.json");
    let file_contents = fs::File::open(the_path).unwrap();
    let mut reader = std::io::BufReader::new(file_contents);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    contents
  } else {
    String::from("")
  }
}

#[tauri::command]
fn write_name_to_json(name: String) {
  println!("RS: Writing Name");
  if let Some(dir) = BaseDirs::new() {
    let the_path = Path::new(&dir.data_dir()).join("NoteAIO").join("config").join("config.json");
    let file_contents = fs::File::open(the_path).unwrap();
    let mut reader = std::io::BufReader::new(file_contents);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let mut json = serde_json::from_str::<serde_json::Value>(&contents).unwrap();
    json["name"] = serde_json::Value::String(name);
    let json_string = serde_json::to_string(&json).unwrap();
    let the_path2 = Path::new(&dir.data_dir()).join("NoteAIO").join("config").join("config.json");
    _ = fs::write(the_path2, json_string);
  }
}

#[tauri::command]
fn create_folder(folder_name: String) {
  println!("RS: Creating Folder");
  if let Some(dir) = BaseDirs::new() {
    let the_path = Path::new(&dir.data_dir()).join("NoteAIO").join("notes").join(&folder_name);
    _ = fs::create_dir_all(the_path);
  }
}