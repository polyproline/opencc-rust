use std::fmt;
use std::fs::File;
use std::io::{self, Read};

use lazy_static::lazy_static;

use crate::convert::ConverterBuild;

lazy_static! {
    static ref TOSIMPLE: Vec<Vec<&'static str>> = vec![
        vec!["HKVariantsRevPhrases", "HKVariantsRev"],
        vec!["TWPhrasesRev", "TWVariantsRevPhrases", "TWVariantsRev"],
        vec!["TSPhrases", "TSCharacters"]
    ];
}
#[test]
fn test() -> io::Result<()> {
    let mut build = ConverterBuild::new();
    let mut string = String::new();

    for list in TOSIMPLE.iter() {
        for item in list {
            let file = fmt::format(format_args!("./data/{}.txt", item));
            // println!("{}",file);
            let mut file = File::open(file)?;
            file.read_to_string(&mut string)?;
            build.adddict(&string);
            string.clear();
        }
        build.group();
    }
    let converter = build.build();
    let mut s = File::open("./data/STPhrases.txt")?;
    s.read_to_string(&mut string)?;

    println!("{}", converter.convert(&string));
    Ok(())
}
