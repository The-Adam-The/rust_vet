use std::sync::Arc;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use petname::petname;
use vet_surgery::{ Animal, Room, Species, Vet, VetSurgery };
use sqlite::State;


mod vet_surgery;



#[tokio::main(flavor = "current_thread")]
async fn main() {


let connection = sqlite::open("vets.db").unwrap();

let query = "
    CREATE TABLE vets (id INTEGER PRIMARY KEY AUTOINCREMENT, forename TEXT, surname TEXT, age INTEGER, available INTEGER);
    INSERT INTO vets VALUES (NULL, 'Carol', 'Hoots', 42, 1);
    INSERT INTO vets VALUES (NULL, 'Jim', 'Doggington', 39,  0);
";
connection.execute(query).unwrap();



let query = "SELECT * FROM vets";
let mut statement = connection.prepare(query).unwrap();
// statement.bind().unwrap();

while let Ok(State::Row) = statement.next() {
    println!("name = {}", statement.read::<String, _>("forename").unwrap());
    println!("age = {}", statement.read::<i64, _>("age").unwrap());
}


    // const NUM_OF_PETS: u8 = 10;

    // let mut animals: Vec<Animal> = Vec::new();

    // //Generate Animals
    // for i in 0..=NUM_OF_PETS {
    //     let animal: Animal = Animal::new(petname(1, "").unwrap(), petname(2, " ").unwrap(), Species::generate_random_species());

    //     // println!("{} {} {:?}", animal.name, animal.owner_name, animal.species);
    //     animals.push(animal);
    // }

    
    // let room_one: Room = Room::new(1, true, Some(vet_carol), None);
    // let room_two: Room = Room::new(2, true, None, None); 

    // let rooms: Vec<&Room> = [&room_one, &room_two].to_vec();



    // let vet_surgery_0: VetSurgery = VetSurgery::new("12 Animal Rd".to_string(),
    //                                                 "11234555".to_string(), 
    //                                                 "animalrdVets@gmail.com".to_string(), 
    //                                                 &vets, 
    //                                                 &rooms);

    // initialize tracing
    tracing_subscriber::fmt::init();
    
    let app = Router::new()
    // `GET /` goes to `root`
    .route("/", get(root))
    .route("/vets", get(move || get_vets()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
    
    // build our application with a route
    
    // let pet_name  = petname(2, " ");
    // println!("Address: {:?}", vet_surgery_0.address);
    // println!("Phone number: {:?}", vet_surgery_0.phone_number);
    // println!("email: {:?}", vet_surgery_0.email);
}


// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
async fn get_vets() -> Json<Vec<Vet>> {
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
    
    Json(vets.to_vec())

}





