// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  // println!("Trying the DB function:\n\n");
  // test();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![double, greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn double(count: i32) -> i32 {
    count * 2
}

fn test() {

    let connection = sqlite::open("/home/ben/projects/JSTG/JSOT.db").unwrap();

    let query = "
        SELECT * FROM [Assessors];
    ";

    connection
        .iterate(query, |pairs| {
            for &(name, value) in pairs.iter() {
                println!("{} = {}", name, value.unwrap());
            }
            true
        })
        .unwrap();

}
