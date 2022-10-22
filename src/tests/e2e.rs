#[cfg(test)]
mod e2e_test {
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::{
        fs::File,
        io::{self, Write},
    };

    use crate::{formatter, parser};

    #[test]
    fn test_is_transpiling_main() {
        let (huffc, murph) = transpile_back("src/tests/data/", "main.huff");
        assert_eq!(huffc, murph);
    }

    #[test]
    fn test_is_transpiling_jump() {
        let (huffc, murph) = transpile_back("src/tests/data/", "jumps.huff");
        assert_eq!(huffc, murph);
    }

    // check if huff -> runtime bytecode == murph -> huff -> runtime bytecode
    fn transpile_back(root: &str, file: &str) -> (String, String) {
        let root = Path::new(root);
        let file_path = root.join(file);
        let mut temp_file = String::from("TEMP_");
        temp_file.push_str(file);
        let temp_path = root.join(temp_file);

        let huffc_bytecode = get_bytecode_from_path(get_path_str(&file_path));
        let murph_code = formatter::to_huff(&mut parser::parse(huffc_bytecode.clone(), false));
        save_temp_murph_file(&murph_code, get_path_str(&temp_path)).unwrap();
        let murph_bytecode = get_bytecode_from_path(get_path_str(&temp_path));

        (huffc_bytecode, murph_bytecode)
    }

    fn get_path_str(path: &Path) -> &str {
        path.to_str().unwrap()
    }

    fn get_bytecode_from_path(path: &str) -> String {
        let output = Command::new("huffc").args(&[path, "-r"]).output().unwrap();
        if !output.stderr.is_empty() {
            panic!("{:?}", String::from_utf8_lossy(&output.stderr));
        }
        String::from_utf8_lossy(&output.stdout).to_string()
    }

    fn save_temp_murph_file(code: &str, path: &str) -> Result<(), io::Error> {
        let mut temp = File::create(path)?;
        temp.write_all(code.as_bytes())?;

        Ok(())
    }
}
