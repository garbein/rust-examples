#[derive(Debug)]
struct Unit;

struct Tuple(&'static str, u32);

struct User {
    name: String,
    age: u32,
}

impl User {
    fn set_age(&mut self, value: u32) {
        self.age = value;
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

impl User {
    pub fn new(name: String, age: u32) -> User {
        User { name, age }
    }
}

struct Config<'a> {
    ip: &'a str,
    port: u32,
    deamon: bool,
}

fn main() {
    let unit = Unit;
    println!("{:?}", unit);
    let tuple = Tuple("王一", 1);
    println!("{}", tuple.0);
    let _user1 = User {
        name: String::from("张三"),
        age: 3,
    };

    let name = String::from("李四");
    let user2 = User { age: 4, name };
    println!("{}", user2.name);

    let _config = Config {
        ip: "127.0.0.1",
        port: 7878,
        deamon: false,
    };

    let username = String::from("关联函数");
    let mut user3 = User::new(username, 32);
    println!("{}", user3.get_age());
    user3.set_age(18);
    println!("{}", user3.get_age());
    let update = String::from("struct update syntax");
    let user4 = User {
        name: update,
        ..user3
    };
    println!("{}", user4.age);
}