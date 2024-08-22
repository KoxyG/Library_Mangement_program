#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
struct User {
    id: u32,
    name: String,
    borrowed_books: Vec<u32>,
}   

// manage a collection of users
#[derive(Debug, PartialEq, Eq)]
struct LibraryUsers {
    users: Vec<User>,
}


impl LibraryUsers {
    // An empty new users
    fn new() -> LibraryUsers {
        LibraryUsers {
            users: Vec::new(),
        }
    }

    // Register a new user
    fn register_user(&mut self, user: User) -> Result<(), String> {
       
       if self.users.iter().any(|u| u.id == user.id) {
           return Err(format!("User with id {} already exists", user.id));
         }
        self.users.push(user);
        Ok(())

    }

    // Remove user from the library
    fn remove_user(&mut self, id: u32) -> Result<(), String> {
       
        match self.users.iter().position(|u| u.id == id) {
            Some(index) => {
                self.users.remove(index);
                Ok(())
            },
            None => Err(format!("User with id {} not found", id)),
        }
    }

    // Find user in the library
    fn find_user(&self, id: u32) -> Result<&User, String> {
        match self.users.iter().find(|u| u.id == id) {
            Some(user) => Ok(user),
            None => Err(format!("User with id {} not found", id)),
        }
    }

}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_register_user() {
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
    fn test_remove_user() {
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
    fn test_find_user() {
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

}
