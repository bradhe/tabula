use snafu::Snafu;

#[derive(PartialEq, Debug, Snafu)]
pub enum Error {
    #[snafu(display("not implemented"))]
    NotImplemented,

    #[snafu(display("end of file"))]
    EOF,

    #[snafu(display("invalid record length"))]
    InvalidRecordLength,

    #[snafu(display("failed to write data"))]
    WriteFailed,
}

#[derive(Debug, PartialEq)]
pub enum Cell {
    String(String),
    Number(f64),
}

#[derive(Clone)]
pub enum ColumnDataType {
    String,
    Number,
}

#[derive(Clone)]
pub struct Column {
    pub name: String,
    pub data_type: ColumnDataType,
}

fn encode_usize_varint(_s: usize) -> Vec<u8> {
    Vec::new()
}

fn encode_string(s: &str) -> Vec<u8> {
    let mut res = Vec::new();
    res.extend(encode_usize_varint(s.len()));
    res.extend(s.as_bytes());
    res
}

impl Column {
    pub fn read_from(_read: &mut impl std::io::Read) -> Result<Self, Error> {
        Err(Error::NotImplemented)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend(encode_string(&self.name));
        res
    }
}

pub struct TabulaReader<T: std::io::Read> {
    cols: Vec<Column>,
    read: T,
}

impl<T: std::io::Read> TabulaReader<T> {
    pub fn new(read: T) -> Result<Self, Error> {
        Ok(Self {
            cols: Vec::new(),
            read,
        })
    }

    pub fn columns(&self) -> Vec<Column> {
        self.cols.to_vec()
    }

    pub fn read_record(&self) -> Result<Vec<Cell>, Error> {
        Err(Error::NotImplemented)
    }
}

pub struct TabulaWriter<T: std::io::Write + std::io::Seek> {
    cols: Vec<Column>,
    write: T,
}

impl<T: std::io::Write + std::io::Seek> TabulaWriter<T> {
    pub fn new(columns: &[Column], mut write: T) -> Result<Self, Error> {
        if let Err(_err) = write.seek(std::io::SeekFrom::Start(0)) {
            // TODO: Do something with this error.
            return Err(Error::WriteFailed);
        } else {
            for col in columns {
                write.write_all(&col.to_vec()).unwrap();
            }
        }

        Ok(Self {
            cols: columns.to_vec(),
            write,
        })
    }

    pub fn columns(&self) -> Vec<Column> {
        self.cols.to_vec()
    }

    fn write_string_cell(&self, _cell: &Cell) -> Result<(), Error> {
        Err(Error::NotImplemented)
    }

    fn write_number_cell(&self, _cell: &Cell) -> Result<(), Error> {
        Err(Error::NotImplemented)
    }

    pub fn write_record(&self, cells: &[Cell]) -> Result<(), Error> {
        if cells.len() != self.cols.len() {
            Err(Error::InvalidRecordLength)
        } else {
            for (idx, col) in self.cols.iter().enumerate() {
                let res = match col.data_type {
                    ColumnDataType::String => self.write_string_cell(&cells[idx]),
                    ColumnDataType::Number => self.write_number_cell(&cells[idx]),
                };

                if res.is_err() {
                    return res;
                }
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_works() {
        // buffer should be big enough to fit the whole test.
        let mut buf = vec![0u8; 10000];

        let columns = vec![
            Column {
                name: "Column1".to_string(),
                data_type: ColumnDataType::String,
            },
            Column {
                name: "Column2".to_string(),
                data_type: ColumnDataType::Number,
            },
        ];

        {
            let tablua = TabulaWriter::new(&columns, Cursor::new(&mut buf)).unwrap();
            assert_eq!(tablua.columns().len(), 2);

            let res = tablua.write_record(&vec![
                Cell::String("hello, world!".to_string()),
                Cell::Number(1.0),
            ]);

            assert_eq!(res, Ok(()));
        }

        //buf.seek(SeekFrom::Start(0));

        {
            let tablua = TabulaReader::new(Cursor::new(&mut buf)).unwrap();
            assert_eq!(tablua.columns().len(), 2);

            let res = tablua.read_record();
            assert!(res.is_ok());
        }
    }
}
