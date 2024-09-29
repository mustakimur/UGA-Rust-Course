fn is_alive() -> bool {
    true
}

mod animal {

    mod human {
        pub struct Person {
            pub name: String,
        }

        impl Person {
            pub fn new(name: &str) -> Person {
                Person {
                    name: name.to_string(),
                }
            }

            pub fn talk(&self) {
                println!("{} talks", self.name);
                self.walk();
            }
        }
    }
    mod pets {

        pub struct Dog {
            pub name: String,
        }

        impl Dog {
            pub fn new(name: &str) -> Dog {
                Dog {
                    name: name.to_string(),
                }
            }

            pub fn bark(&self) {
                println!("{} barks", self.name);
            }
        }

        impl super::human::Person {
            pub fn walk(&self) {
                println!("{} walks", self.name);
            }
        }
    }

    pub fn can_move() -> bool {
        true
    }

    pub fn create_animal(human: bool) {
        if human {
            let person = human::Person::new("John");
            person.talk();
        } else {
            let dog = pets::Dog::new("Rusty");
            dog.bark();
        }
        super::is_alive();
    }
}

mod plant {
    #[derive(Debug)]
    pub enum PlantType {
        Tree,
        Flower,
    }
    pub struct Tree {
        pub name: String,
        kind: PlantType,
    }

    impl Tree {
        pub fn new(name: &str, is_flower: bool) -> Tree {
            Tree {
                name: name.to_string(),
                kind: if is_flower {
                    PlantType::Flower
                } else {
                    PlantType::Tree
                },
            }
        }

        pub fn grow(&self) {
            println!("{} grows", self.name);
        }
    }

    pub fn create_plant(name: String) {
        let tree = Tree::new(&name, true);
        tree.grow();
        println!("can plant move: {}", can_move());
    }

    pub fn can_move() -> bool {
        false
    }
}

use self::animal::can_move as can_animal_move;
use crate::plant::can_move as can_plant_move;
use plant::PlantType;

fn main() {
    println!("Can animal move: {}", crate::animal::can_move());
    println!("Can plant move: {}", plant::can_move());

    animal::create_animal(true);

    plant::create_plant("Oak".to_string());

    println!("Can animal move: {}", can_animal_move());
    println!("Can plant move: {}", can_plant_move());

    println!("{:?}", PlantType::Tree);
}
