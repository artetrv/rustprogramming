use std::fs::File;
use std::io::{Write, BufRead, BufReader};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    let mut file = File::create(filename).expect("Could not create output file");
    for b in books {
        writeln!(file, "{},{},{}", b.title, b.author, b.year)
            .expect("Failed writing a line");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).expect("Could not open input file");
    let reader = BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut parts = line.splitn(3, ',');
        let title  = parts.next().unwrap_or("").trim().to_string();
        let author = parts.next().unwrap_or("").trim().to_string();
        let year_s = parts.next().unwrap_or("").trim();

        if title.is_empty() || author.is_empty() || year_s.is_empty() {
            continue;
        }

        if let Ok(year) = year_s.parse::<u16>() {
            result.push(Book { title, author, year });
        }
    }

    result
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        Book { title: "The Great Gatsby".to_string(), author: "F. Scott Fitzgerald".to_string(), year: 1925 },
        Book { title: "Pride and Prejudice".to_string(), author: "Jane Austen".to_string(), year: 1813 },
        Book { title: "The Catcher in the Rye".to_string(), author: "J.D. Salinger".to_string(), year: 1951 },
        Book { title: "The Hobbit".to_string(), author: "J.R.R. Tolkien".to_string(), year: 1937 },
        Book { title: "The Lord of the Rings".to_string(), author: "J.R.R. Tolkien".to_string(), year: 1954 },
        Book { title: "Fahrenheit 451".to_string(), author: "Ray Bradbury".to_string(), year: 1953 },
        Book { title: "The Alchemist".to_string(), author: "Paulo Coelho".to_string(), year: 1988 },
        Book { title: "The Road".to_string(), author: "Cormac McCarthy".to_string(), year: 2006 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("\nLoaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}

