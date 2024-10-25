struct User {
    name: String,
    email: String,
    user_count: i32,
    active: bool
}



fn main() {

   let user : User = User  {
        name: String::from("Moises"),
        email: String::from("moises@gmail.com"),
        user_count: 1,
        active: true
    };
    
    fn create_ser(name: String, email: String) -> User{
        let new_user = User {
            name,
            email,
            user_count: 2,
            active: true
        };
        print!("{}, {}, {}, {}", new_user.name, new_user.email, new_user.active, new_user.user_count);
    return  new_user;

    }
    
  let user_name = String::from("Daniela villareal");
  let user_email = String::from("Dany@thewarning.com");  

    create_ser(user_name, user_email);

}


/*

interface User {
name: string;
email: string;
user_count: number;
active: boolean;

const user : User = {
    name: Moises
    email: moises@gmail.com
    user_count: 01
    active: true
}

    const createUser = (name: string, email: string) : User => {
        return {
            name: name,
            email: email,
            user_count: 1,
            active: false
        }
    }

*/