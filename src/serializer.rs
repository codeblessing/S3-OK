use std::{error::Error, io::Write};
pub struct Serializer<T: Write> {
    is_buffered: bool,
    records: Vec<Record>,
    writer: T,
}

impl<T: Write> Serializer<T> {
    pub fn new(writer: T) -> Self {
        Self {
            is_buffered: true,
            records: Vec::new(),
            writer: writer,
        }
    }

    pub fn buffered(&mut self, is_buffered: bool) -> &Self {
        self.is_buffered = is_buffered;
        self
    }

    pub fn add_record(&mut self, record: Record) -> &Self {
        self.records.push(record);
        if !self.is_buffered {
            self.save("").unwrap();
        }
        self
    }

    pub fn save(&mut self, delimiter: &str) -> Result<(), Box<dyn Error>> {
        for record in &self.records {
            self.writer
                .write_all(format!("{}\n", record.serialize()).as_bytes())?;
        }
        self.writer.write_all(delimiter.as_bytes())?;
        self.writer.flush()?;
        self.records.clear();
        Ok(())
    }

    pub fn get_writer(&self) -> &T {
        &self.writer
    }
}

pub struct Record {
    iteration: u64,
    makespan: u128,
}

impl Record {
    pub fn new(iteration: u64, makespan: u128) -> Self {
        Self {
            iteration,
            makespan,
        }
    }

    pub fn serialize(&self) -> String {
        format!("{},{}", self.iteration, self.makespan)
    }
}

#[cfg(test)]
mod test_serializer {
    use super::*;

    #[test]
    fn test_create_empty() {
        let serializer = Serializer::new(Vec::new());

        assert_eq!(serializer.is_buffered, true);
        assert!(serializer.records.is_empty());
    }

    #[test]
    fn test_serialize_record() {
        let mut serializer = Serializer::new(Vec::new());

        for i in 0..5 {
            serializer.add_record(Record {
                iteration: i,
                makespan: i as u128,
            });
        }

        serializer.save("").unwrap();
        let out = String::from_utf8(serializer.get_writer().to_owned()).unwrap();
        assert_eq!(out, String::from("0,0\n1,1\n2,2\n3,3\n4,4\n"));
    }
}
