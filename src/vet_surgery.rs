#[derive(Debug, Clone)]
pub struct Vet {
    pub employee_id: i64,
    pub first_name: String,
    pub second_name: String,
    pub date_of_birth: String
}

impl Vet {
    pub fn new(employee_id: i64, first_name: String, second_name: String, date_of_birth: String) -> Vet {
        Vet {employee_id, first_name, second_name, date_of_birth}
    }
}

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
pub struct Room<'a> {
    pub id: i8,
    pub available: bool,
    pub vet: &'a Vet,
    pub animal: &'a Animal
}

impl <'a>Room<'a> {
    pub fn new(id: i8, available: bool, vet: &'a Vet, animal: &'a Animal) -> Room<'a> {
        Room { id, available, vet, animal }
    }
}

#[derive(Debug)]
pub enum Species {
    Dog,
    Cat,
    Rat,
    GuineaPig,
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