use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, BufReader, BufWriter},
};

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    title: String,
    author: String,
    available: bool,
    isbn: String,
}

impl Book {
    fn new(title: &str, author: &str, isbn: &str) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            available: true,
            isbn: isbn.to_string(),
        }
    }

    fn borrow_book(&mut self) {
        if self.available {
            self.available = false;
            println!("You have borrowed the book: {}", self.title)
        } else {
            println!("The book {} is currently unavailable", self.title)
        }
    }

    fn return_book(&mut self) {
        if !self.available {
            self.available = true;
            println!("You've retruned the book: {}", self.title);
        } else {
            println!("The book {} was not borrowed.", self.title)
        }
    }
}

fn add_book(books: &mut Vec<Book>, title: &str, author: &str, isbn: &str) {
    let book = Book::new(title, author, isbn);
    books.push(book);
    println!("Book added: {}", title)
}

fn generate_report(books: &[Book]) {
    for book in books {
        println!(
            "Title: {}, Author: {}, ISBN: {}, Available: {}",
            book.title, book.author, book.isbn, book.available
        )
    }
}

fn save_book(books: &[Book], filename: &str) -> io::Result<()> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &books)?;

    Ok(())
}

fn load_books(filename: &str) -> io::Result<Vec<Book>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let books = serde_json::from_reader(reader)?;
    Ok(books)
}

fn main() {
    let mut books: Vec<Book> = match load_books("books.json") {
        Ok(books) => books,
        Err(_) => vec![],
    };

    loop {
        println!("\nLibrary Management System");
        println!("1. Add Book");
        println!("2. Borrow Book");
        println!("3. Return Book");
        println!("4. Generate Report");
        println!("5. Save Data");
        println!("6. Load Data");
        println!("7. Exit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => {
                let (title, author, isbn) = get_book_details();
                add_book(&mut books, &title, &author, &isbn);
            }
            2 => {
                let isbn = get_isbn();
                if let Some(book) = books.iter_mut().find(|b| b.isbn == isbn) {
                    book.borrow_book();
                } else {
                    println!("Book not found.");
                }
            }
            3 => {
                let isbn = get_isbn();
                if let Some(book) = books.iter_mut().find(|b| b.isbn == isbn) {
                    book.return_book();
                } else {
                    println!("Book not found.");
                }
            }
            4 => generate_report(&books),
            5 => {
                if save_book(&books, "books.json").is_ok() {
                    println!("Data saved successfully.");
                } else {
                    println!("Failed to save data.");
                }
            }
            6 => {
                if let Ok(loaded_books) = load_books("books.json") {
                    books = loaded_books;
                    println!("Data loaded successfully.");
                } else {
                    println!("Failed to load data.");
                }
            }
            7 => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn get_book_details() -> (String, String, String) {
    println!("Enter book title:");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    println!("Enter book author:");
    let mut author = String::new();
    io::stdin()
        .read_line(&mut author)
        .expect("Failed to read line");

    println!("Enter book ISBN:");
    let mut isbn = String::new();
    io::stdin()
        .read_line(&mut isbn)
        .expect("Failed to read line");

    (
        title.trim().to_string(),
        author.trim().to_string(),
        isbn.trim().to_string(),
    )
}

fn get_isbn() -> String {
    println!("Enter book ISBN:");
    let mut isbn = String::new();
    io::stdin()
        .read_line(&mut isbn)
        .expect("Failed to read line");
    isbn.trim().to_string()
}
