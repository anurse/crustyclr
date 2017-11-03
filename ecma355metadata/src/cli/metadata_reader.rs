use std::io::{Seek, SeekFrom};

use error::Error;

use pe::{DirectoryType, PeImage};
use cli::{CliHeader, MetadataHeader, MetadataStreamHeader};

pub struct MetadataReader {
    pe_image: PeImage,
    cli_header: CliHeader,
    metadata_header: MetadataHeader,
    stream_headers: Vec<MetadataStreamHeader>,
}

impl MetadataReader {
    pub fn new(mut pe: PeImage) -> Result<MetadataReader, Error> {
        // Locate and load the CLI header
        let cli_header = load_cli_header(&mut pe)?;

        let (metadata_header, stream_headers) = {
            // Locate and load metadata_header
            let mut reader = pe.create_reader(cli_header.metadata.rva)
                .ok_or(Error::CliHeaderNotFound)?;
            let metadata_header = MetadataHeader::read(&mut reader)?;

            // Load section headers
            let mut stream_headers = Vec::with_capacity(metadata_header.streams as usize);
            for _ in 0..metadata_header.streams {
                stream_headers.push(MetadataStreamHeader::read(&mut reader)?);
            }

            (metadata_header, stream_headers)
        };

        Ok(MetadataReader {
            pe_image: pe,
            cli_header: cli_header,
            metadata_header: metadata_header,
            stream_headers: stream_headers,
        })
    }

    pub fn cli_header(&self) -> &CliHeader {
        &self.cli_header
    }

    pub fn metadata_header(&self) -> &MetadataHeader {
        &self.metadata_header
    }

    pub fn stream_headers(&self) -> &Vec<MetadataStreamHeader> {
        &self.stream_headers
    }

    pub fn get_stream(&'a mut self, name: &str) -> Option<StreamReader<'a>> {
        if let Some(header) = self.stream_headers.iter().find(|s| s.name == name) {
            let section_reader = self.get_metadata_section_reader();

            // Seek to the stream offset
            if let Ok(_) = section_reader.seek(SeekFrom::Start(header.offset)) {
                // Wrap it in a stream reader
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_metadata_section_reader(&'a mut self) -> Option<SectionReader<'a>> {
        self.pe_image.create_reader(self.cli_header.metadata.rva)
    }
}

fn load_cli_header(pe: &mut PeImage) -> Result<CliHeader, Error> {
    let cli_header_rva = pe.get_directory(DirectoryType::CliHeader)
        .map(|d| d.range.rva)
        .ok_or(Error::CliHeaderNotFound)?;

    let mut reader = pe.create_reader(cli_header_rva)
        .ok_or(Error::CliHeaderNotFound)?;

    Ok(CliHeader::read(&mut reader)?)
}
