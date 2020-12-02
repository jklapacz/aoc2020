pub mod utils {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub fn read_lines<P>(filename: P) -> String
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename);
        let mut contents = String::new();

        if let Ok(mut file) = file {
            match file.read_to_string(&mut contents) {
                Err(e) => println!("{:?}", e),
                _ => (),
            }
        }
        return contents;
    }
}
