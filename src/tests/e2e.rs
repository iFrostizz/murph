#[cfg(test)]
mod e2e_test {
    use crate::{formatter, parser};
    use std::{
        fs::{self, File},
        io::{self, Write},
        path::Path,
        process::Command,
    };

    #[test]
    fn is_transpiling() {
        let source = "src/tests/data/";
        for entry in fs::read_dir(source).unwrap() {
            let dir = entry.unwrap();
            let path = dir.path();
            let file = path.file_name().unwrap();
            let (huffc, murph) = transpile_back(source, file.to_str().unwrap());

            assert_eq!(huffc, murph);
        }
    }

    // check if huff -> runtime bytecode == murph -> huff -> runtime bytecode
    fn transpile_back(root: &str, file: &str) -> (String, String) {
        let root = Path::new(root);
        let file_path = root.join(file);
        let mut temp_file = String::from("TEMP_");
        temp_file.push_str(file);
        let temp_path = root.join(temp_file);

        let huffc_bytecode = get_bytecode_from_path(get_path_str(&file_path));
        let murph_code = formatter::to_huff(
            &mut parser::parse(hex::decode(huffc_bytecode.clone()).unwrap(), false).unwrap(),
        );
        save_temp_murph_file(&murph_code, get_path_str(&temp_path)).unwrap();
        let murph_bytecode = get_bytecode_from_path(get_path_str(&temp_path));

        (huffc_bytecode, murph_bytecode)
    }

    fn get_path_str(path: &Path) -> &str {
        path.to_str().unwrap()
    }

    fn get_bytecode_from_path(path: &str) -> String {
        let output = Command::new("huffc").args([path, "-r"]).output().unwrap();
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
