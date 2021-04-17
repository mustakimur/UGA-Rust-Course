fn main() {
    encapsulation();
}
mod list_maintainer {
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn new() -> AveragedCollection {
            AveragedCollection {
                list: Vec::new(),
                average: 0.0,
            }
        }
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
}

fn encapsulation() {
    let mut avg_obj = list_maintainer::AveragedCollection::new();

    //avg_obj.list.push(10);
    //avg_obj.average = 20.0;
    avg_obj.add(10);
    avg_obj.add(8);
    avg_obj.add(5);
    avg_obj.add(12);

    println!("Current average: {}", avg_obj.average());

    avg_obj.remove();

    println!("Current average: {}", avg_obj.average());
}
