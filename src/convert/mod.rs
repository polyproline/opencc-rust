use crate::dict::DictEntry;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ConverterBuild{
	res:Vec<Vec<DictEntry>>,
	tmp:Vec<DictEntry>,
	min:usize,
}
#[wasm_bindgen]
pub struct Converter{
	dict:Vec<Vec<DictEntry>>,
	min:u8,
}
#[wasm_bindgen]
impl ConverterBuild{
	pub fn new()->Self{
		Self{res:Vec::new(),tmp:Vec::new(),min:usize::max_value()}
	}
	pub fn adddict(&mut self,data:&str){
		let mut min = usize::max_value();
		let mut keys = Vec::new();
		let mut values = Vec::new();

		for line in data.lines(){
			let key_v:Vec<&str> = line.split('\t').collect();
			if key_v.is_empty(){
				continue;
			}
			assert_eq!(key_v.len(),2);
			let (key,value) = unsafe{
				let key = key_v.get_unchecked(0).to_string();
				let value = key_v.get_unchecked(1);
				let values:Vec<&str>  = value.split(' ').collect();
				let value:Vec<String> = values.into_iter().map(|t|t.to_string()).collect();
				(key,value)
			};
			assert!(key.len()>0,"key\t{}\t{:?}",key,value);
			assert!(value.len()>0,"value\t{}\t{:?}",key,value);
			min = min.min(key.len());
			keys.push(key);
			values.push(value);
		}
		assert!(min < u8::max_value() as usize);
		let t = DictEntry::new(min as u8,keys,values);
		self.min = self.min.min(min);
		self.tmp.push(t);
	}
	pub fn group(&mut self){
		assert_ne!(self.tmp.len(),0);
		let t = self.tmp.split_off(0);
		self.res.push(t);
	}
	pub fn build(&mut self)->Converter{
		assert_eq!(self.tmp.len(),0);
		// dbg!("Converter min",self.min);
		Converter{
			min:self.min as u8,
			dict:self.res.split_off(0)
		}
	}
}
#[wasm_bindgen]
impl Converter{
	pub fn convert(&self,data:&str)->String{
		let mut data = data.to_string();
		if data.len() < self.min as usize{
			return data;
		}
		// dbg!(&data);
		'a : for dicts in &self.dict{
			for dict in dicts{
				let (res,flag) = dict.convert(&data);
				dbg!(&res);
				data = res;
				// if flag{
				// 	continue 'a;
				// }
			}
		}
		return data;
	}
	pub fn delete(&mut self){
		self.min = u8::max_value();
		self.dict=Vec::new();
	}
}