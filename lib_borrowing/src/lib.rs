// A Library struct to manage a collection of books. Implement the following methods:

#[derive(Debug, PartialEq, Eq)]
struct Book {
    id: number,
    title: String,
    author: String,
    genre: Genre,
    year: u32,
    availability: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct  Library {
    books: Vec<Book>,
}

#[derive(Debug, PartialEq, Eq)]
enum Genre {
    Fiction,
    Science,
    Fantasy,
    Horror,
    Romance,
}

impl Library {

    // A new empty library 
    fn new() -> Library {
        Library {
            books: Vec::new()
        }
    }

    // Add a book to the library
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    // remove book from the library
    fn remove_book(&mut self, title: &str) -> Result<(), String> {   
       match self.books.iter().position(|b| b.title == title) {
              Some(index) => {
                self.books.remove(index);
                Ok(())
              },
              None => Err(format!("Book with title {} not found", title))
       }
    }

    // find books in the library
    fn find_books(&self, title: &str) -> Result<&Book, String> {
       match self.books.iter().find(|b| b.title == title) {
        Some(book) => Ok(book),
        None => Err(format!("Book with title '{}' not found", title)),
       }
    }

    // list books in the library
    fn list_books(&self) {   
        for book in &self.books {
            println!("Title: {}, Author: {}, Year: {}", book.id, book.title, book.author, book.genre. book.year, book.availability);
        }
    }

    // count books in the library
    fn count_books(&self) -> usize {
        self.books.len()
    }


}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        todo!()
    }

    #[test]
    fn test_addBook() {
        todo!()
    }

    #[test]
    fn test_removeBook() {
        todo!()
    }

    #[test]
    fn test_findbook() {
        todo!()
    }

    #[test]
    fn test_listBook() {
        todo!()
    }

    #[test]
    fn test_countBook() {
        todo!()
    }
}