use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// Class responsible for outputting compiled code
pub struct Emitter<'a> {
    full_path: &'a std::path::Path,
    compiled_code: String
}

impl<'a> Emitter<'a> {
    pub fn new(full_path: &'a str) -> Emitter<'a> {
        Emitter {
            full_path: Path::new(full_path),
            compiled_code: String::new()
        }
    }

    pub fn emit(&mut self, chunk: &str) {
        self.compiled_code += chunk;
    }

    pub fn emit_line(&mut self, chunk: &str) {
        self.compiled_code += chunk;
        self.compiled_code += "\n";
    }

    pub fn write_file(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&*self.full_path)
            .expect("Couldn't open file");

        if let Err(why) = file.write_all(self.compiled_code.as_bytes()) {
            panic!("Couldn't write to {}: {}", self.full_path.display(), why);
        }
    }
}