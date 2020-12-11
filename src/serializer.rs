use csv::Writer;
use serde::Serialize;
use std::io::Write;
pub struct Serializer<T: Write> {
    is_buffered: bool,
    records: Vec<Record>,
    writer: Writer<T>,
}

impl<T: Write> Serializer<T> {
    pub fn new(writer: T) -> Self {
        Self {
            is_buffered: true,
            records: Vec::new(),
            writer: Writer::from_writer(writer),
        }
    }

    pub fn buffered(&mut self, is_buffered: bool) -> &Self {
        self.is_buffered = is_buffered;
        self
    }

    pub fn add_record(&mut self, record: Record) -> &Self {
        self.records.push(record);
        self
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        for record in &self.records {
            self.writer.serialize(record)?;
        }
        self.writer.flush()?;
        Ok(())
    }

    pub fn get_writer(self) -> Result<T, ()> {
        match self.writer.into_inner() {
            Ok(writer) => Ok(writer),
            Err(err) => {
                eprintln!("Error: {}", err);
                Err(())
            }
        }
    }
}

#[derive(Serialize)]
pub struct Record {
    iteration: u64,
    makespan: u128,
}

#[cfg(test)]
mod test_serializer {
    use super::*;

    #[test]
    fn test_create_empty() {
        let file = OpenOptions::new().create(true).open("data.log").unwrap();
        let serializer = Serializer::new(file);

        assert_eq!(serializer.is_buffered, true);
        assert!(serializer.records.is_empty());
    }

    #[test]
    fn test_serialize_record() {
        let output = Vec::new();
        let mut serializer = Serializer::new(output);

        for i in 0..5 {
            serializer.add_record(Record {
                iteration: i,
                makespan: i as u128,
            });
        }

        serializer.save().unwrap();
        let out = String::from_utf8(serializer.get_writer().unwrap()).unwrap();
        assert_eq!(
            out,
            String::from("iteration,makespan\n0,0\n1,1\n2,2\n3,3\n4,4\n")
        );
    }
}
