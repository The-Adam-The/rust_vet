use sqlite::{State, Connection, Value};

use crate::vet_surgery::Vet;


pub fn set_up_db() {

    let connection = sqlite::open("vets.db").unwrap();
    
    let query = "
    DROP TABLE IF EXISTS vets;
    CREATE TABLE vets (id INTEGER PRIMARY KEY AUTOINCREMENT, forename TEXT, surname TEXT, age INTEGER, available INTEGER);
    INSERT INTO vets VALUES (NULL, 'Carol', 'Hoots', 42, 1);
    INSERT INTO vets VALUES (NULL, 'Jim', 'Doggington', 39,  0);
    ";
    connection.execute(query).unwrap();
    
    let query = "
    DROP TABLE IF EXISTS rooms;
    CREATE TABLE rooms (id INTEGER PRIMARY KEY AUTOINCREMENT, vet_id INTEGER, animal_id INTEGER, available INTEGER);
    INSERT INTO rooms VALUES (NULL, 1, 1, NULL);
    INSERT INTO rooms VALUES (NULL, 2, 1, NULL);
    ";
    connection.execute(query).unwrap();
}

pub fn get_all_vets() -> Vec<Vet>{
        let connection = sqlite::open("vets.db").unwrap();
    let query = "SELECT * FROM vets";
    let mut statement = connection.prepare(query).unwrap();

    let mut vets: Vec<Vet> = vec!();

    while let Ok(State::Row) = statement.next() {
        let id: i64 = statement.read::<i64, _>("id").unwrap();
        let forename = statement.read::<String, _>("forename").unwrap();
        let surname: String = statement.read::<String, _>("surname").unwrap();
        let age: u8 = statement.read::<i64, _>("age").unwrap().try_into().unwrap();
        let available: bool = statement.read::<i64, _>("available").unwrap() != 0;
        
        let vet = Vet::new(
            id,
            forename,
            surname,
            age,
            available
        );
        vets.push(vet);
}
    
    vets.to_vec()  
}

pub fn get_vet_by_id(id: i64) -> Vet {
    let connection = sqlite::open("vets.db").unwrap();
    let query = "SELECT * FROM vets WHERE id = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, id)).unwrap();

    
    if let Ok(State::Row) = statement.next() {
        let id: i64 = statement.read::<i64, _>("id").unwrap();
        let forename = statement.read::<String, _>("forename").unwrap();
        let surname: String = statement.read::<String, _>("surname").unwrap();
        let age: u8 = statement.read::<i64, _>("age").unwrap().try_into().unwrap();
        let available: bool = statement.read::<i64, _>("available").unwrap() != 0;
            
        let vet = Vet::new(
            id,
            forename,
            surname,
            age,
            available
        );

        vet
    } else {
        Vet::default() // Assuming Vet has a default implementation
    }
}  