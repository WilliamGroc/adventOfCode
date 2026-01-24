use std::fs;

struct Detector {
    sum_bad_ids: i64,
}

impl Detector {
    pub fn detect_bad_id(&mut self, range: Vec<&str>) -> Result<(), std::num::ParseIntError> {
        let start_id: i64 = range[0].parse()?;
        let end_id: i64 = range[1].parse()?;

        for id in start_id..=end_id {
            println!("Checking ID: {}", id);
            if Detector::is_bad_id(id) {
                self.sum_bad_ids += id;
            }
        }

        Ok(())
    }

    fn is_bad_id(id: i64) -> bool {
        let str = id.to_string();
        let len = str.len();

        for i in 1..=len {
            let block = str[i - 1..i].chars().next().unwrap();
            println!("Block: {}", block);
        }

        return true;
    }
}

pub fn run() -> Result<(), std::num::ParseIntError> {
    let mut detector = Detector { sum_bad_ids: 0 };

    let contents = read_file();

    for range_ids in contents.split(",") {
        let range: Vec<&str> = range_ids.split("-").collect();
        detector.detect_bad_id(range)?;
    }

    println!("Counter: {}", detector.sum_bad_ids);
    Ok(())
}

fn read_file() -> String {
    return fs::read_to_string("./data_day2.txt").expect("Failed to read file");
}
