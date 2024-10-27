
extern crate rand;

use rand::Rng;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct VetSurgery<'a> {
    pub address: String,
    pub phone_number: String,
    pub email: String,
    pub vets: &'a Vec<&'a Vet>,
    pub rooms: &'a Vec<&'a Room<'a>>
}

impl <'a>VetSurgery<'a> {
    pub fn new(address: String, phone_number: String, email: String, vets: &'a Vec<&'a Vet>, rooms: &'a Vec<&'a Room> ) -> VetSurgery<'a>
    {   
        VetSurgery { address, phone_number, email, vets, rooms }
    }
}

#[derive(Debug)]
pub struct Animal {
    pub name: String,
    pub owner_name: String,
    pub species: Species,
}

impl Animal {
    pub fn new(name: String, owner_name: String, species: Species) -> Animal {
        Animal {name, species, owner_name}
    }

}


#[derive(Debug)]
pub struct Room<'a> {
    pub id: i8,
    pub available: bool,
    pub vet: Option<&'a Vet>,
    pub animal: Option<&'a Animal>
}

impl <'a>Room<'a> {
    pub fn new(id: i8, available: bool, vet: Option<&'a Vet>, animal: Option<&'a Animal>) -> Room<'a> {
        Room { id, available, vet, animal }
    }
}


#[derive(Debug)]
pub enum Species {
    Dog,
    Cat,
    Rat,
    GuineaPig,
    Fish,
    Lizard,
    Mouse,
    Rabbit,
    Snake,
    Chicken,
    Duck,
    SeaMonkey,
    Frog
}

impl Species {
    fn get_range() -> usize {
        Species::Frog as usize
    }
    
    
    pub fn generate_random_species() -> Self {
        //Get total number of enums
        let total_species: usize = Self::get_range();
        //Randomly select number
        match rand::thread_rng().gen_range(0..=total_species) {
            0 => Species::Dog,
            1 => Species::Cat,
            2 => Species::Rat,
            3 => Species::GuineaPig,
            4 => Species::Fish,
            5 => Species::Lizard,
            6 => Species::Mouse,
            7 => Species::Rabbit,
            8 => Species::Snake,
            9 => Species::Chicken,
            10 => Species::Duck,
            11 => Species::SeaMonkey,
            12 => Species::Frog,
            _ => panic!("Unexpected species index")
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Vet {
    pub id: i64,
    pub forename: String,
    pub surname: String,
    pub age: u8,
    pub available: bool
}

impl Vet {
    pub fn new(id: i64, forename: String, surname: String, age: u8, available: bool) -> Vet {
        Vet {id, forename, surname, age, available}
    }

    pub fn default() -> Vet {
        Vet { id: 0, forename: "".to_string(), surname: "".to_string(), age: 0, available: false}
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateVet {
    pub forename: String,
    pub surname: String,
    pub age: u8,
    pub available: bool
}