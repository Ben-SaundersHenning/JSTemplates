use sqlite::State;
use std::string::String;
use serde::Serialize;

const DB_PATH: &str = if cfg!(windows) {
    "C:\\Users\\Ben Saunders-Henning\\projects\\JSTG\\JSTG.sqlite3"
} 
else {
    "/home/ben/projects/JSTG/JSTG.sqlite3"
};

#[derive(Serialize)]
pub struct Assessor {
    salutation: String,
    first_name: String,
    last_name: String
}

pub fn get_all_assessor_info() -> Vec<Assessor> {

    let connection = sqlite::open(DB_PATH).unwrap();
    let query = "SELECT Salutation, FirstName, LastName FROM [Assessors];";
    let mut statement = connection.prepare(query).unwrap();

    let mut assessors: Vec<Assessor> = Vec::new();

    while let Ok(State::Row) = statement.next() {
        let assessor = Assessor {
            salutation: statement.read::<String, _>("Salutation").unwrap(),
            first_name: statement.read::<String, _>("FirstName").unwrap(),
            last_name: statement.read::<String, _>("LastName").unwrap(),
        };
        assessors.push(assessor);
    }

    assessors

}

//func to help retrieve absolute paths
//on different machines during development.
#[tauri::command]
pub fn get_path(system: &str, dir: &str) -> String {
    let connection = sqlite::open(DB_PATH).unwrap();
    let query = "
        SELECT Path FROM [Paths]
        WHERE OperatingSystem = :os
        AND Directory = :dir";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((":os", system)).unwrap();
    statement.bind((":dir", dir)).unwrap();

    while let Ok(State::Row) = statement.next() {
        return statement.read::<String, _>("Path").unwrap();
    }

    "NA".to_string()
}
