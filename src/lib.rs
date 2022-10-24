use snafu::Snafu;
use std::io::{Read, Seek, SeekFrom, Write};

mod encode;

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

    #[snafu(display("invalid cell type"))]
    InvalidCellType,

    #[snafu(display("invalid column type"))]
    InvalidColumnType,
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

impl ColumnDataType {
  pub fn to_vec(&self) -> Vec<u8> {
    match self {
      ColumnDataType::String => vec![0x01; 1],
      ColumnDataType::Number => vec![0x02; 1],
    }
  }

  pub fn read_from(read: &mut impl Read) -> Result<Self, Error> {
    let b = read.bytes().next().and_then(|result| result.ok()).unwrap();
    match b {
      0x00 => Err(Error::InvalidColumnType),
      0x01 => Ok(ColumnDataType::String),
      0x02 => Ok(ColumnDataType::Number),
      3_u8..=u8::MAX => Err(Error::InvalidColumnType),
    }
  }
}

#[derive(Clone)]
pub struct Column {
  pub name: String,
  pub data_type: ColumnDataType,
}

impl Column {
  pub fn read_from(read: &mut impl Read) -> Result<Self, Error> {
    // this is the column type

    Err(Error::NotImplemented)
  }

  pub fn to_vec(&self) -> Vec<u8> {
    let mut res = Vec::new();
    res.extend(self.data_type.to_vec());
    res.extend(encode::string(&self.name));
    res
  }
}

pub struct TabulaReader<T: Read> {
  cols: Vec<Column>,
  read: T,
}

impl<T: Read> TabulaReader<T> {
  pub fn new(read: T) -> Result<Self, Error> {
    // let's see about getting the columns out.
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

fn write_string_cell<T: Write>(write: &mut T, cell: &Cell) -> Result<(), Error> {
  if let Cell::String(val) = cell {
    if let Err(_err) = write.write(&encode::string(&val)) {
      Err(Error::WriteFailed)
    } else {
      Ok(())
    }
  } else {
    Err(Error::InvalidCellType)
  }
}

fn write_number_cell<T: Write>(write: &mut T, cell: &Cell) -> Result<(), Error> {
  if let Cell::Number(val) = cell {
    if let Err(_err) = write.write(&encode::number(val)) {
      Err(Error::WriteFailed)
    } else {
      Ok(())
    }
  } else {
    Err(Error::InvalidCellType)
  }
}

pub struct TabulaWriter<T: Write + Seek> {
  cols: Vec<Column>,
  write: T,
}

impl<T: Write + Seek> TabulaWriter<T> {
  pub fn new(columns: &[Column], mut write: T) -> Result<Self, Error> {
    if let Err(_err) = write.seek(SeekFrom::Start(0)) {
      // TODO: Do something with this error.
      return Err(Error::WriteFailed);
    } else {
      write.write_all(&encode::usize_varint(columns.len())).unwrap();

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

  pub fn write_record(&mut self, cells: &[Cell]) -> Result<(), Error> {
    if cells.len() != self.cols.len() {
      Err(Error::InvalidRecordLength)
    } else {
      for (idx, col) in self.cols.iter().enumerate() {
        let res = match col.data_type {
          ColumnDataType::String => write_string_cell(&mut self.write, &cells[idx]),
          ColumnDataType::Number => write_number_cell(&mut self.write, &cells[idx]),
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

    {
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

      let mut tablua = TabulaWriter::new(&columns, Cursor::new(&mut buf)).unwrap();
      assert_eq!(tablua.columns().len(), 2);

      let res = tablua.write_record(&vec![
                                    Cell::String("hello, world!".to_string()),
                                    Cell::Number(1.0),
      ]);

      assert_eq!(res, Ok(()));
    }

    {
      let tablua = TabulaReader::new(Cursor::new(&mut buf)).unwrap();
      assert_eq!(tablua.columns().len(), 2);

      let res = tablua.read_record();
      assert!(res.is_ok());
    }
  }
}
