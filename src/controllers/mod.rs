pub mod tickets;

// pub fn create(&self) {
//     let connection = establish_connection();
//
//     print!("Enter a title: ");
//     stdout().flush().unwrap();
//     let mut title = String::new();
//     stdin().read_line(&mut title).unwrap();
//     let title = title.trim_right();
//
//     print!("Enter an description: ");
//     stdout().flush().unwrap();
//     let mut description = String::new();
//     stdin().read_line(&mut description).unwrap();
//     let description = description.trim_right();
//
//     println!("Creating ticket!");
//
//     let _ = create_ticket(&connection, title, description);
//     println!("\nSaved {}", title);
// }
