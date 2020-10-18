use std::io::{self,Read};
use std::fs::File;
use std::fmt;

use lazy_static::lazy_static;

use crate::convert::{ConverterBuild,Converter};

lazy_static! {
	static ref TOSIMPLE:Vec<Vec<&'static str>> =vec![vec!["HKVariantsRevPhrases", "HKVariantsRev"], vec!["TWPhrasesRev", "TWVariantsRevPhrases", "TWVariantsRev"], vec!["TSPhrases", "TSCharacters"]];
}
#[test]
fn test()->io::Result<()>{
	let mut build = ConverterBuild::new();
	let mut string = String::new();

	for list in TOSIMPLE.iter(){
		for item in list{
			let file = fmt::format(format_args!("./data/{}.txt",item));
			// println!("{}",file);
			let mut file = File::open(file)?;
			file.read_to_string(&mut string)?;
			build.adddict(& string);
			string.clear();
		}
		build.group();
	}
	let converter = build.build();
	let trans = converter.convert("鳥站所在場域要進行電力重整，所以會有兩天的時間無法提供服務喔");
	assert_eq!(trans,"鸟站所在场域要进行电力重整，所以会有两天的时间无法提供服务喔");
	Ok(())
}