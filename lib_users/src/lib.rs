#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub borrowed_books: Vec<u32>,
}   

// manage a collection of users
#[derive(Debug, PartialEq, Eq)]
pub struct LibraryUsers {
    users: Vec<User>,
}


impl LibraryUsers {
    // An empty new users
    pub fn new() -> LibraryUsers {
        LibraryUsers {
            users: Vec::new(),
        }
    }

    // Register a new user
    pub fn register_user(&mut self, user: User) -> Result<(), String> {
       
       if self.users.iter().any(|u| u.id == user.id) {
           return Err(format!("User with id {} already exists", user.id));
        }

        self.users.push(user);
        Ok(())

    }

    // Remove user from the library
    pub fn remove_user(&mut self, id: u32) -> Result<(), String> {
       
        match self.users.iter().position(|u| u.id == id) {
            Some(index) => {
                self.users.remove(index);
                Ok(())
            },
            None => Err(format!("User with id {} not found", id)),
        }
    }

    // Find user in the library
    pub fn find_user(&self, id: u32) -> Result<&User, String> {
        match self.users.iter().find(|u| u.id == id) {
            Some(user) => Ok(user),
            None => Err(format!("User with id {} not found", id)),
        }
    }

    pub fn borrowed_books(&mut self, id: u32, book_id: u32) {
       if let Some(user) = self.users.iter_mut().find(|u| u.id == id) {
           user.borrowed_books.push(book_id);
       }
    }

    pub fn return_book(&mut self, id: u32, book_id: u32) {
        if let Some(user) = self.users.iter_mut().find(|u| u.id == id) {
            if let Some(pos) = user.borrowed_books.iter().position(|&id| id == book_id) {
                user.borrowed_books.remove(pos);
            }
        }
    }

}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_register_user() {
        let mut users = LibraryUsers::new();
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            borrowed_books: Vec::new(),
        };

        assert_eq!(users.register_user(user.clone()), Ok(()));
        assert_eq!(users.register_user(user.clone()), Err("User with id 1 already exists".to_string()));
    }

    #[test]
    pub fn test_remove_user() {
        let mut users = LibraryUsers::new();
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            borrowed_books: Vec::new(),
        };

        users.register_user(user.clone()).unwrap();
        assert_eq!(users.remove_user(1), Ok(()));
        assert_eq!(users.remove_user(1), Err("User with id 1 not found".to_string()));
    }

    #[test]
    pub fn test_find_user() {
        let mut users = LibraryUsers::new();
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            borrowed_books: Vec::new(),
        };

        users.register_user(user.clone()).unwrap();
        assert_eq!(users.find_user(1), Ok(&user));
        assert_eq!(users.find_user(2), Err("User with id 2 not found".to_string()));
    }

    #[test]
    fn test_burrowed_books() {
        let mut users = LibraryUsers::new();
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            borrowed_books: Vec::new(),
        };

        users.register_user(user.clone()).unwrap();
        users.borrowed_books(1, 1);
        assert_eq!(users.find_user(1).unwrap().borrowed_books, vec![1]);
    }

}
