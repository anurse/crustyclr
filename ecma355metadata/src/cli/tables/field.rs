use std::io::Cursor;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::tables::{TableIndex, TableReader};
use cli::{StringHandle, BlobHandle, StringHandleReader, BlobHandleReader, MetadataSizes};

use error::Error;

pub struct Field {
    pub flags: u16,
    pub name: StringHandle,
    pub signature: BlobHandle,
}

pub struct FieldReader {
    string_reader: StringHandleReader,
    blob_reader: BlobHandleReader,
}

impl TableReader for FieldReader {
    type Item = Field;
    const INDEX: TableIndex = TableIndex::Field;

    fn new(sizes: &MetadataSizes) -> FieldReader {
        FieldReader {
            string_reader: StringHandleReader::new(sizes),
            blob_reader: BlobHandleReader::new(sizes),
        }
    }

    fn row_size(&self) -> usize {
        size_of::<u16>() +
            self.string_reader.size() +
            self.blob_reader.size()
    }

    fn read(&self, buf: &[u8]) -> Result<Field, Error> {
        let mut cursor = Cursor::new(buf);

        Ok(Field {
            flags: cursor.read_u16::<LittleEndian>()?,
            name: self.string_reader.read(&mut cursor)?,
            signature: self.blob_reader.read(&mut cursor)?,
        })
    }
}