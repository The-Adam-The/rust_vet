
#[derive(Debug, Clone)]
struct Vet {
    employee_id: i64,
    first_name: String,
    second_name: String,
    date_of_birth: String
}

impl Vet {
    fn new(employee_id: i64, first_name: String, second_name: String, date_of_birth: String) -> Vet {
        Vet {employee_id, first_name, second_name, date_of_birth}
    }
}


#[derive(Debug)]
struct VetSurgery<'a> {
    address: String,
    phone_number: String,
    email: String,
    vets: &'a Vec<&'a Vet>,
    rooms: &'a Vec<&'a Room<'a>>
}

impl <'a>VetSurgery<'a> {
    fn new(address: String, phone_number: String, email: String, vets: &'a Vec<&'a Vet>, rooms: &'a Vec<&'a Room> ) -> VetSurgery<'a>
    {   
        VetSurgery { address, phone_number, email, vets, rooms }
    }
}

#[derive(Debug)]
struct Room<'a> {
    id: i8,
    available: bool,
    vet: &'a Vet,
    animal: &'a Animal
}

impl <'a>Room<'a> {
    fn new(id: i8, available: bool, vet: &'a Vet, animal: &'a Animal) -> Room<'a> {
        Room { id, available, vet, animal }
    }
}

#[derive(Debug)]
enum Species {
    Dog,
    Cat,
    Rat,
    GuineaPig,
}

#[derive(Debug)]
struct Animal {
    name: String,
    owner_name: String,
    species: Species,
}

impl Animal {
    fn new(name: String, owner_name: String, species: Species) -> Animal {
        Animal {name, species, owner_name}
    }
}

fn main() {
    let vet_carol: Vet = Vet::new(1, "Carol".to_string(), "Hoots".to_string(), "29/02/1980".to_string());
    let vet_jim: Vet = Vet::new(2, "Jim".to_string(), "Doggington".to_string(), "09/09/1985".to_string());
    let vets: Vec<&Vet> = [&vet_carol, &vet_jim].to_vec();

    let animal_0: Animal = Animal::new("fido".to_string(), "David Dogman".to_string(), Species::Dog);
    
    let room_one: Room = Room::new(1, true, &vet_carol, &animal_0);

    let rooms: Vec<&Room> = [&room_one].to_vec();



    let vet_surgery_0: VetSurgery = VetSurgery::new("12 Animal Rd".to_string(),
                                                    "11234555".to_string(), 
                                                    "animalrdVets@gmail.com".to_string(), 
                                                    &vets, 
                                                    &rooms);
    
    println!("Address: {:?}", vet_surgery_0.address);
    println!("Phone number: {:?}", vet_surgery_0.phone_number);
    println!("email: {:?}", vet_surgery_0.email);

    println!("animal_0: {} {} {:?}", animal_0.name, animal_0.owner_name, animal_0.species);
    for vet in vet_surgery_0.vets {
        println!("Vet id: {} Vet: {} {} DOB: {}", vet.employee_id, vet.first_name, vet.second_name, vet.date_of_birth)
    }

    for room in vet_surgery_0.rooms {
        println!("room id: {}, available: {} vet: {} {} animal: {}", room.id, room.available, room.vet.first_name, room.vet.second_name, room.animal.name)
    }

    


}

