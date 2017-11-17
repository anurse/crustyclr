extern crate ecma355metadata;

use std::env;
use std::str;
use std::fs::File;
use std::io::Cursor;

use ecma355metadata::{MetadataReader, PeImage};
use ecma355metadata::cli::CliHeader;
use ecma355metadata::cli::tables::TableIndex;
use ecma355metadata::pe::DirectoryType;
use ecma355metadata::Guid;

fn load_cli_header(pe: &PeImage) -> CliHeader {
    let (rva, size) = pe.get_directory(DirectoryType::CliHeader)
        .map(|d| (d.range.rva, d.range.size))
        .unwrap();

    let mut reader = Cursor::new(pe.load(rva, size as usize).unwrap());

    CliHeader::read(&mut reader).unwrap()
}

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: dump_table <file> <table>");
    } else {
        let file_path = &args[1];

        let mut file = File::open(file_path).unwrap();
        let pe = PeImage::read(&mut file).unwrap();
        let cli_header = load_cli_header(&pe);
        let assembly = MetadataReader::new(
            pe.load(cli_header.metadata.rva, cli_header.metadata.size as usize)
                .unwrap(),
        ).unwrap();

        if args.len() < 3 {
            dump_table_names(&assembly);
        }
        else {
            let table_name = &args[2];
            let table: TableIndex = table_name.parse().expect("Unknown metadata table");

            match table {
                TableIndex::Module => dump_module_table(&assembly),
                TableIndex::TypeRef => dump_type_ref_table(&assembly),
                x => println!("Table not yet implemented: {}", x),
            }
        }
    }
}

pub fn dump_table_names(assembly: &MetadataReader) {
    println!("Table Row Counts:");
    for idx in TableIndex::each() {
        println!("  {}: {} rows", idx, assembly.tables().metadata_sizes().row_count(idx));
    }
}

pub fn dump_type_ref_table(assembly: &MetadataReader) {
    let type_ref_table = assembly.tables().type_ref();
    println!("TypeRef Table: {} rows", type_ref_table.len());
    for row in type_ref_table.iter() {
        let row = row.unwrap();
        let name = assembly.get_string(row.name).unwrap_or(b"<null>");
        let namespace = assembly.get_string(row.namespace);

        if let Some(ns) = namespace {
            println!(
                " * {}.{} (Scope: {})",
                str::from_utf8(ns).unwrap(),
                str::from_utf8(name).unwrap(),
                row.resolution_scope
            );
        } else {
            println!(
                " * {} (Scope: {})",
                str::from_utf8(name).unwrap(),
                row.resolution_scope
            );
        }
    }
    println!()
}

pub fn dump_module_table(assembly: &MetadataReader) {
    let module_table = assembly.tables().module();

    println!("Module Table: {} rows", module_table.len());
    for row in module_table.iter() {
        let row = row.unwrap();
        println!("  Generation: {}", row.generation);
        println!(
            "  Name: {}",
            str::from_utf8(assembly.get_string(row.name).unwrap()).unwrap()
        );
        println!(
            "  MVID: {}",
            assembly.get_guid(row.mvid).unwrap_or(&Guid::EMPTY)
        );
    }
    println!();
}
