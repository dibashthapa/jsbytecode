use std::{fs::File, io::Write};
pub fn define_ast(
    output_dir: String,
    base_name: &str,
    types: &[&str],
) -> Result<(), std::io::Error> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());

    let mut file = File::create(path)?;

    writeln!(&mut file, "use crate::token::{{Token, Value}};")?;
    writeln!(&mut file)?;
    writeln!(&mut file, "pub enum {} {{", base_name)?;

    for ttype in types {
        let value = ttype.trim().split_once(":").unwrap();
        let value = value.0.trim();
        writeln!(&mut file, "    {value}({value}{base_name}),")?;
    }

    writeln!(&mut file, "}}")?;

    writeln!(&mut file)?;
    define_visitor(&mut file, base_name.to_string(), types)?;

    for ttype in types {
        let split_values = ttype.split_once(":").unwrap();

        let class_name = split_values.0.trim();
        let fields = split_values.1.trim();

        define_type(
            &mut file,
            base_name.to_string(),
            class_name.to_string(),
            fields.to_string(),
        )
    }

    Ok(())
}

fn define_visitor(
    file: &mut File,
    base_name: String,
    types: &[&str],
) -> Result<(), std::io::Error> {
    writeln!(file, "pub trait Visitor<T> {{")?;

    for ttype in types {
        let type_name = ttype.trim().split_once(":").unwrap();
        let type_name = type_name.0.trim();
        writeln!(
            file,
            "    fn visit_{}_expr(&mut self, expr: &{}{}) -> T;",
            type_name.to_lowercase(),
            type_name,
            base_name
        )?;
    }
    writeln!(file, "}}")?;
    Ok(())
}

fn define_type(file: &mut File, base_name: String, class_name: String, file_list: String) {
    writeln!(file, "");
}
