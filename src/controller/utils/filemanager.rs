use crate::defines::memory_constants::{MAX_PROGRAM_SIZE, MEMORYSIZE};
use crate::edit;

use std::{
    convert::TryInto,
    fs::{metadata, File},
    io::{BufWriter, Read, Result, Write},
    path::Path,
    process::Command,
};

pub const MEMORY_DUMP_PATH: &str = "TempFiles/Memory_Content.bin";

#[derive(Default, Clone)]
pub struct FileInfo {
    pub file_name: String,
    pub file_size: u64,
}

pub struct FileManager {
    filecontent: Vec<u8>,
    file_path: String,
    file_info: FileInfo,
}

impl FileManager {
    pub fn new(path: String) -> FileManager {
        FileManager {
            filecontent: Vec::new(),
            file_path: path,
            file_info: FileInfo::default(),
        }
    }

    pub fn load_file(&mut self) -> Result<()> {
        let mut file = File::open(self.file_path.clone())?;
        let meta_data = metadata(self.file_path.clone())?;

        assert!(meta_data.len() < MAX_PROGRAM_SIZE as u64);
        self.file_info.file_name = Path::new(&self.file_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let file_size = meta_data.len();
        let mut buffer = vec![0; file_size as usize];
        file.read_exact(&mut buffer)?;
        self.filecontent = buffer;
        self.file_info.file_size = file_size;

        Ok(())
    }

    pub fn load_file_if_possible(&mut self, file_path: &str) -> Result<()> {
        let old_file_name = self.file_info.file_name.clone();
        let old_file_content = self.filecontent.clone();
        self.filecontent = vec![0; MAX_PROGRAM_SIZE];

        self.file_path = String::from(file_path);
        let success = self.load_file();

        if success.is_err() {
            self.file_info.file_name = old_file_name;
            self.filecontent = old_file_content;
        }

        success
    }

    pub fn dump_memory(&mut self, memory: Vec<u8>) {
        println!("Dump memory");
        let file = File::create(MEMORY_DUMP_PATH).expect("Unable to create file");
        let mut file = BufWriter::new(file);
        file.write_all(&memory).expect("Unable to write data");

        let editor = edit::get_editor().unwrap();
        //let mut file_path = temp_dir();
        //file_path.push("editable");
        File::create(&MEMORY_DUMP_PATH).expect("Could not create file");

        Command::new(editor)
            .arg(MEMORY_DUMP_PATH)
            .status()
            .expect("Something went wrong");

        /* let mut editable = String::new();
        File::open(MEMORY_DUMP_PATH)
            .expect("Could not open file")
            .read_to_string(&mut editable);

        println!("File content:\n{}", editable); */
    }

    pub fn get_file_content(&mut self) -> Vec<u8> {
        self.filecontent.clone()
    }

    pub fn get_file_name(&mut self) -> String {
        self.file_info.file_name.clone()
    }

    pub fn get_file_info(&mut self) -> FileInfo {
        self.file_info.clone()
    }
}
