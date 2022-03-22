use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config{
    pub query:String,
    pub filename: String,
}

impl Config {
    // &'static str 是字符串字面值的类型，也是目前的错误信息。
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // panic!("args array length is not enough!");
            return Err("args array length is not enough!");
        }

        let query = &args[1];
        let filename = &args[2];
        // (query, filename)
        // Ok(Config, {query, filename})
        Ok(Config {
            query: query.clone(),
            filename: filename.clone()
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines(){
        if line.contains(query) {
            result.push(line);
        }
    }

    result
    // vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}