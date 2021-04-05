use crate::processor::memory_constants::MAX_PROGRAM_SIZE;
use std::{fs, io};
use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;
use std::convert::TryInto;


pub struct FileManager {
    filecontent: Vec<u8>,
    file_path: String,
    file_name: String,
    file_size: u32
}

impl FileManager {
    pub fn new(path: String) -> FileManager {
        FileManager {
            filecontent: vec![0; MAX_PROGRAM_SIZE],
            file_path: path,
            file_name: String::new(),
            file_size: 0,
        }
    }

    pub fn load_file(&mut self) -> Result<()> {
        let mut file = File::open(self.file_path.clone())?;
        let meta_data = fs::metadata(self.file_path.clone())?;

        assert!(meta_data.len() < MAX_PROGRAM_SIZE as u64);
        self.file_name = Path::new(&self.file_path).file_name().unwrap().to_str().unwrap().to_string();
        let mut buffer = vec![0; meta_data.len() as usize];
        file.read(&mut buffer)?;
        
        self.filecontent[..buffer.len()].clone_from_slice(&buffer[0..buffer.len()]);    
        
        for i in buffer.len()..self.filecontent.len() {
            self.filecontent[i] = 0;
        }

        Ok(())
    }

    pub fn load_file_if_possible(&mut self, file_path: &String) -> Result<()>{
        let old_file_name = self.file_name.clone();
        let old_file_content = self.filecontent.clone();
        self.filecontent = vec![0; MAX_PROGRAM_SIZE];

        self.file_path = file_path.clone();
        let success = self.load_file();

        if success.is_err() {
            self.file_name = old_file_name;
            self.filecontent = old_file_content;
        } 

        success
    }

    pub fn get_file_content(&mut self) -> [u8; MAX_PROGRAM_SIZE] {
        self.filecontent.clone().try_into().unwrap()
    }

    pub fn get_file_name(&mut self) -> String {
        self.file_name.clone()
    }
}