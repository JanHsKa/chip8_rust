use crate::constants::MAX_PROGRAM_SIZE;
use std::io;
use std::fs::File;
use io::prelude::*;


pub struct FileManager {
    filecontent: [u8; MAX_PROGRAM_SIZE],
    file_path: String,
}

impl FileManager {
    pub fn new(path: String) -> FileManager {
        FileManager {
            filecontent: [0; MAX_PROGRAM_SIZE],
            file_path: path,
        }
    }

    pub fn load_file(&mut self) -> io::Result<()> {
        let mut file = File::open(self.file_path.clone())?;
        let mut buffer = [0; MAX_PROGRAM_SIZE];
        
        file.read(&mut buffer[..])?;

        for (i, iter) in buffer.iter().enumerate() {
             self.filecontent[i] = *iter;
        }

        Ok(())
    }

    pub fn get_file_content(&mut self) -> [u8; MAX_PROGRAM_SIZE] {
        return self.filecontent.clone();
    }
}