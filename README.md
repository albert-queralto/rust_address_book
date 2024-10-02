# Contact Book
This is a simple command-line contact book application written in Rust. It allows you to create, search, list, edit, and delete contacts.

## Features
- **Create:** Add a new contact with a name and phone number.
- **Search:** Find a contact by name.
- **List:** Display all contacts.
- **Edit:** Modify an existing contact's phone number.
- **Delete:** Remove a contact by name.
- **Exit:** Quit the application.

## Usage
1. Clone the repository.
2. Run the application with `cargo run`.
3. Follow the on-screen instructions to interact with the contact book.
```
Contact Book
╔═══════════════════════════╗
║ 1. Create     4. Edit     ║
║ 2. Search     5. Delete   ║
║ 3. List       q. Exit     ║
╚═══════════════════════════╝
```
4. Select and option by entering the corresponding number or `q` to exit the application.

## Example
```
$ cargo run
Contact Book
╔═══════════════════════════╗
║ 1. Create     4. Edit     ║
║ 2. Search     5. Delete   ║
║ 3. List       q. Exit     ║
╚═══════════════════════════╝
Select an option: 1
Name: John Doe
Phone: 1234567890
Contact added successfully.
```

## License
This project is licensed under the MIT License.