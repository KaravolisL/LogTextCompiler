use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Class responsible for outputting compiled code
pub struct Emitter<'a> {
    full_path: Box<&'a std::path::Path>,
    compiled_code: String
}

impl<'a> Emitter<'a> {
    pub fn new(full_path: &'a str) -> Emitter {
        Emitter {
            full_path: Box::new(Path::new(full_path)),
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
        let mut file = match File::open(&*self.full_path) {
            Err(why) => {
                panic!("Couldn't open {}: {}", self.full_path.display(), why);
            },
            Ok(file) => file,
        };

        match file.write_all(self.compiled_code.as_bytes()) {
            Err(why) => {
                panic!("Couldn't write to {}: {}", self.full_path.display(), why)
            },
            Ok(_) => ()
        }
    }
}