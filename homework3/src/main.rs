/*
Rust Assignment: Book Catalog
Objective

Create a simple Book Catalog system in Rust that demonstrates struct usage and file I/O operations.
Requirements

    Create a Book struct with the following fields:
        title: String
        author: String
        year: u16

    Implement the following functions:
        save_books(books: &Vec<Book>, filename: &str): Saves all books to a file.
        load_books(filename: &str) -> Vec<Book>: Loads books from a file.

    In the main function:
        Create a few Book instances
        Save the books to a file
        Load the books from the file and print them
*/

use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro

    let mut file = File::create(filename).expect("Unable to create file");

    for book in books{
        let line = format!("{},{},{}\n", book.title, book.author, book.year);
        file.write_all(line.as_bytes()).expect("Unable to write to file");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut books = Vec::new();

    for line in reader.lines(){
        let line = line.expect("Could not read line");
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() == 3{
            let book = Book{
                title: parts[0].to_string(),
                author: parts[1].to_string(),
                year: parts[2].parse().unwrap_or(0),
            };
            books.push(book);
        }
    }
    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}