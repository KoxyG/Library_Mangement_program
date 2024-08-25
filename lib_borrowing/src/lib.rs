use lib_inventory::{Library, Book, Genre};
use lib_users::{LibraryUsers, User};


// FUNCTION TO BORROW A BOOK
pub fn borrow_book(
    lib_book_users: &mut LibraryUsers,
    library: &mut Library,
    user_id: u32,
    book_title: &str,
) -> Result<(), String> {
    // Check if the user is registered
    match lib_book_users.find_user(user_id) {
        Ok(_) => {
            // Check if the book is available
            match library.find_books(book_title) {
                Ok(book) => {
                    let book_id = book.id;

                    if let Some(book) = library.books.iter_mut().find(|b| b.id == book_id) {
                        if book.availability {
                            // Mark book availability as false
                            book.availability = false;

                            // Update the user's borrowed books list
                            lib_book_users.borrowed_books(user_id, book_id);

                            Ok(())
                        } else {
                            Err(format!("Book '{}' is not available", book_title))
                        }
                    } else {
                        Err(format!("Book '{}' not found", book_title))
                    }
                }
                Err(_) => Err("Book not found".to_string()),
            }
        }
        Err(_) => Err("User is not registered".to_string()),
    }
}


// FUNCTION TO RETURN A BOOK
pub fn return_book(
    lib_book_users: &mut LibraryUsers,
    library: &mut Library,
    user_id: u32,
    book_title: &str,
) -> Result<(), String> {
    // Check if the user is registered
    match lib_book_users.find_user(user_id) {
        Ok(user) => {
            // Check if the book is available
            match library.find_books(book_title) {
                Ok(book) => {
                    let book_id = book.id;

                    if let Some(_pos) = user.borrowed_books.iter().position(|&id| id == book_id) {
                        // Remove the book from the user's borrowed list
                        lib_book_users.return_book(user_id, book_id);

                        // Mark the book as available
                        if let Some(book) = library.books.iter_mut().find(|b| b.id == book_id) {
                            book.availability = true;
                        }

                        Ok(())
                    } else {
                        Err(format!("Book '{}' is not borrowed by user {}", book_title, user_id))
                    }
                }
                Err(_) => Err("Book not found".to_string()),
            }
        }
        Err(_) => Err("User is not registered".to_string()),
    }
}


// TESTING
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_borrow_book() {
        let mut users = LibraryUsers::new();
        let mut library = Library::new();

        // Add a user and a book to the library
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            borrowed_books: Vec::new(),
        };
        users.register_user(user.clone()).unwrap();

        let book = Book {
            id: 1,
            title: "The Great Rust".to_string(),
            author: "Rustacean".to_string(),
            genre: Genre::Fiction,
            year: 2024,
            availability: true,
        };
        library.add_book(book.clone());

        // Test borrowing a book
        assert_eq!(borrow_book(&mut users, &mut library, 1, "The Great Rust"), Ok(()));
        assert_eq!(library.find_books("The Great Rust").unwrap().availability, false);
        assert!(users.find_user(1).unwrap().borrowed_books.contains(&1));
    }

    #[test]
    fn test_return_book() {
        let mut users = LibraryUsers::new();
        let mut library = Library::new();

        // Add a user and a book to the library
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            borrowed_books: vec![1],
        };
        users.register_user(user.clone()).unwrap();

        let book = Book {
            id: 1,
            title: "The Great Rust".to_string(),
            author: "Rustacean".to_string(),
            genre: Genre::Fiction,
            year: 2024,
            availability: false, // Assume it's borrowed
        };
        library.add_book(book.clone());

        // Test returning the book
        assert_eq!(return_book(&mut users, &mut library, 1, "The Great Rust"), Ok(()));
        assert_eq!(library.find_books("The Great Rust").unwrap().availability, true);
        assert!(!users.find_user(1).unwrap().borrowed_books.contains(&1));
    }
}
