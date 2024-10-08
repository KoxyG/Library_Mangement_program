#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub genre: Genre,
    pub year: u32,
    pub availability: bool,
}

// manage a collection of book
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct Library {
   pub books: Vec<Book>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Genre {
    Fiction,
    Science,
    Fantasy,
    Horror,
    Romance,
}

#[allow(dead_code)]
impl Library {
    // A new empty library 
    pub fn new() -> Library {
        Library {
            books: Vec::new(),
        }
    }

    // Add a book to the library
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    // Remove book from the library
    pub fn remove_book(&mut self, title: &str) -> Result<(), String> {   
        match self.books.iter().position(|b| b.title == title) {
            Some(index) => {
                self.books.remove(index);
                Ok(())
            },
            None => Err(format!("Book with title {} not found", title)),
        }
    }

    // Find books in the library
    pub fn find_books(&self, title: &str) -> Result<&Book, String> {
        match self.books.iter().find(|b| b.title == title) {
            Some(book) => Ok(book),
            None => Err(format!("Book with title '{}' not found", title)),
        }
    }

    // List books in the library
    pub fn list_books(&self) -> &Vec<Book> {   
        &self.books
    }

    // Count books in the library
    pub fn count_books(&self) -> usize {
        self.books.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let library = Library::new();
        assert_eq!(library.count_books(), 0);
    }

    #[test]
    fn test_add_book() {
        let mut library = Library::new();

        let book = Book {
            id: 1,
            title: String::from("Rust book"),
            author: String::from("Koxy"),
            genre: Genre::Fiction,
            year: 1925,
            availability: true,
        };
        library.add_book(book);

        assert_eq!(library.count_books(), 1);
        assert_eq!(library.find_books("Rust book").unwrap().title, "Rust book");
    }

    #[test]
    fn test_remove_book() {
        let mut library = Library::new();
        let book = Book {
            id: 2,
            title: String::from("Remove soon"),
            author: String::from("Koxy"),
            genre: Genre::Fiction,
            year: 2000,
            availability: true,
        };
        library.add_book(book);

        assert_eq!(library.count_books(), 1);

        // ensure book is removed
        assert!(library.remove_book("Remove soon").is_ok());
        assert_eq!(library.count_books(), 0);

        // ensure book is not found
        assert!(library.remove_book("Remove soon").is_err());
    }

    #[test]
    fn test_find_book() {
        let mut library = Library::new();
        let book = Book {
            id: 3,
            title: String::from("Blockchain development"),
            author: String::from("Koxy"),
            genre: Genre::Science,
            year: 2020,
            availability: true,
        };
        library.add_book(book);

        assert_eq!(library.count_books(), 1);
        assert_eq!(library.find_books("Blockchain development").unwrap().title, "Blockchain development");
        assert!(library.find_books("Blockchain").is_err());
    }

    #[test]
    fn test_list_books() {
       let mut library = Library::new();
        let book = Book {
            id: 4,
            title: String::from("Rust book"),
            author: String::from("Koxy"),
            genre: Genre::Fiction,
            year: 1925,
            availability: true,
        };
        library.add_book(book);

        library.list_books();
    }

    #[test]
    fn test_count_books() {
        let library = Library::new();
        assert_eq!(library.count_books(), 0);
    }
}
