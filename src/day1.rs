use std::fs;

struct Dial {
    position: i32,
    hits: i32,
}

impl Dial {
    pub fn turn(&mut self, command: &str) {
        let direction = &command[0..1];
        let amount: i32 = command[1..].parse().unwrap();

        match direction {
            "L" => {
                self.subtract(amount);
            }
            "R" => {
                self.add(amount);
            }
            _ => {
                println!("Invalid command");
            }
        }
    }

    fn add(&mut self, amount: i32) {
        for _ in 0..amount {
            self.position += 1;
            if self.position == 100 {
                self.position = 0;
            }
            if self.position == 0 {
                self.hits += 1;
            }
        }
    }

    fn subtract(&mut self, amount: i32) {
        for _ in 0..amount {
            self.position -= 1;
            if self.position == -1 {
                self.position = 99;
            }
            if self.position == 0 {
                self.hits += 1;
            }
        }
    }
}

pub fn run() {
    let mut dial = Dial {
        position: 50,
        hits: 0,
    };

    let contents = read_file();
    for line in contents.lines() {
        dial.turn(line)
    }

    println!("Counter: {}", dial.hits);
}

fn read_file() -> String {
    return fs::read_to_string("./data_day1.txt").expect("Failed to read file");
}
