use crate::processor::memory_constants::MAX_PROGRAM_SIZE;
use std::io;
use std::fs;
use std::fs::File;
use std::io::Read;


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
        let meta_data = fs::metadata(self.file_path.clone()).expect("cannot read file");

        assert!(meta_data.len() < MAX_PROGRAM_SIZE as u64);
        let mut buffer = vec![0; meta_data.len() as usize];
        file.read(&mut buffer).expect("buffer overflow");
        
        for (i, iter) in buffer.iter().enumerate() {
            self.filecontent[i] = *iter;
        }    
        Ok(())
    }

    pub fn get_file_content(&mut self) -> [u8; MAX_PROGRAM_SIZE] {
        return self.filecontent.clone();
    }
}