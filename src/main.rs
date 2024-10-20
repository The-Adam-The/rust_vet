mod vet_surgery;

use vet_surgery::{ Animal, Room, Species, Vet, VetSurgery };

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

