fn main() {
    let mut contacts = HashMap::new();
    contact_book(&mut contacts);
}

const MSG: &str  = "
Contact Book
╔═══════════════════════════╗
║ 1. Create     4. Edit     ║
║ 2. Search     5. Delete   ║
║ 3. List       q. Exit     ║
╚═══════════════════════════╝
";

use std::io::{self, Write};
use std::collections::HashMap;

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn contact_book(contacts: &mut HashMap<String, u64>) {
    println!("{}", MSG);
    let option: String = read_input("Select an option: ");

    match option.trim() {
        "1" => create(contacts),
        "2" => search(contacts),
        "3" => list(contacts),
        "4" => edit(contacts),
        "5" => delete(contacts),
        "q" => println!("Goodbye!"),
        _ => {
            println!("Invalid option.");
            contact_book(contacts);
        }
    }
}

fn create(contacts: &mut HashMap<String, u64>) {
    let name: String = read_input("Name: ");
    
    if name.is_empty() {
        println!("Name is required.");
        create(contacts);
    } else if name == "q" {
        contact_book(contacts);
    } else if !contacts.contains_key(&name) {
        add_contact(contacts, name);
    } else {
        println!("Contact already exists.");
        create(contacts);
    }
}

fn add_contact(contacts: &mut HashMap<String, u64>, name: String) {
    let phone: String = read_input("Phone: ");
    let phone = phone.trim();

    if phone == "q" {
        contact_book(contacts);
    } else if phone.len() > 11 {
        println!("Phone number is too long.");
    } else if phone.parse::<u64>().is_ok() {
        contacts.insert(name, phone.parse().unwrap());
        println!("Contact added successfully.");
        contact_book(contacts);
    } else {
        println!("Phone number is invalid.");
        add_contact(contacts, name);
    }
}

fn search(contacts: &mut HashMap<String, u64>) {
    println!("Search contact");
    let name: String = read_input("Name: ");
    
    if name.is_empty() {
        println!("Name is required.");
        search(contacts);
    } else if name == "q" {
        contact_book(contacts);
    } else if let Some(phone) = contacts.get(&name) {
        println!("Name: {} - Phone: {}", name, phone);
        contact_book(contacts);
    } else {
        println!("Contact not found.");
        search(contacts);
    }
}

fn list(contacts: &mut HashMap<String, u64>) {
    if contacts.is_empty() {
        println!("No contacts found.");
    } else {
        for (name, phone) in contacts.iter() {
            println!("Name: {} - Phone: {}", name, phone);
        }
    }
    contact_book(contacts);
}

fn edit(contacts: &mut HashMap<String, u64>) {
    println!("Edit contact");
    let name: String = read_input("Name: ");
    
    if name.is_empty() {
        println!("Name is required.");
        edit(contacts);
    } else if name == "q" {
        contact_book(contacts);
    } else if let Some(phone) = contacts.get(&name) {
        println!("Name: {} - Phone: {}", name, phone);
        add_contact(contacts, name);
    } else {
        println!("Contact not found.");
        edit(contacts);
    }
}

fn delete(contacts: &mut HashMap<String, u64>) {
    println!("Delete contact");
    let name: String = read_input("Name: ");
    
    if name.is_empty() {
        println!("Name is required.");
        delete(contacts);
    } else if name == "q" {
        contact_book(contacts);
    } else if let Some(_) = contacts.remove(&name) {
        println!("Contact deleted successfully.");
        contact_book(contacts);
    } else {
        println!("Contact not found.");
        delete(contacts);
    }
}