use std::fs;

struct Detector {
    sum_bad_ids: i64,
}

impl Detector {
    pub fn detect_bad_id(&mut self, range: Vec<&str>) -> Result<(), std::num::ParseIntError> {
        let start_id: i64 = range[0].parse()?;
        let end_id: i64 = range[1].parse()?;

        for id in start_id..=end_id {
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
            let chunked = Detector::split_string(&str, i as u32);
            let first_chunk = chunked[0];

            if chunked.len() > 1 && chunked.iter().all(|&c| c == first_chunk) {
                println!("Bad ID found: {}", id);
                return true;
            }
        }

        false
    }

    fn split_string(s: &str, chunk: u32) -> Vec<&str> {
        s.as_bytes()
            .chunks(chunk as usize)
            .map(|c| std::str::from_utf8(c).unwrap())
            .collect()
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
    fs::read_to_string("./data_day2.txt").expect("Failed to read file")
}
