use futures::executor::block_on;
use futures::future::join_all;

use std::sync::Arc;
use std::collections::HashMap;
use crate::chars::exclude_char;
use crate::dict::{DictEntry,KeyTree};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// 对于字符串进行分割，避免比较序列过长
/// 也可并行处理(未实现)
const MIN_SYNCOPATION: usize = 512;
const MAX_SYNCOPATION: usize = 1024;

pub struct ConverterBuild {
    res: Vec<Vec<DictEntry>>,
    tmp: Vec<DictEntry>,
}

pub struct Converter {
    dict: Arc<Vec<Vec<DictEntry>>>,
}

impl ConverterBuild {
    pub fn new() -> Self {
        Self {
            res: Vec::new(),
            tmp: Vec::new(),
        }
    }
    pub fn adddict(&mut self, data: &str) {
        let mut keytree = KeyTree::new();
        let mut map:HashMap<String,usize> = HashMap::new();
        let mut values = Vec::new();
        for line in data.lines() {
            let key_v: Vec<&str> = line.split('\t').collect();
            if key_v.is_empty() {
                continue;
            }
            assert_eq!(key_v.len(), 2,"没有对应的 key-value 值");
            let (key, value) = unsafe {
                let key = key_v.get_unchecked(0);
                let value = key_v.get_unchecked(1);
                if let Some(value) = value.split(' ').filter(|x|x != key).next(){
                    (key, value)
                }else{
                    continue;
                }
            };
            assert!(key.len() > 0, "key\t{}\t{:?}", key, value);
            assert!(key.chars().all(exclude_char),"key 值非法，可能未更新");
            if let Some(index) = map.get(&key.to_string()){
                keytree.insert(key.chars().peekable(), *index);
            }else{
                let len = map.len();
                map.insert(key.to_string(),len);
                values.push(value.to_string());
                keytree.insert(key.chars().peekable(), len);
            }
        }
        self.tmp.push(DictEntry::new(keytree, values));
    }
    pub fn group(&mut self) {
        assert_ne!(self.tmp.len(), 0);
        let t = self.tmp.split_off(0);
        self.res.push(t);
    }
    pub fn build(mut self) -> Converter {
        assert_ne!(self.res.len(), 0);
        let mut t = Vec::new();
        core::mem::swap(&mut self.res, &mut t);
        Converter {
            dict: Arc::new(t),
        }
    }

}
impl Converter {
    async fn convert_future(&self, data: &str) -> String {
        let datas = {
            let mut count = 0;
            data.split_inclusive(move |c: char| {
                count += 1;
                if count < MIN_SYNCOPATION {
                    return false;
                }
                if count > MAX_SYNCOPATION {
                    count = 0;
                    return true;
                }
                if exclude_char(c) {
                    count = 0;
                    true
                } else {
                    false
                }
            })
        };
        // 这里可以用 async-std 或 tokio 库多线程 并行加速
        join_all(datas.map(|s| convert_slice(self.dict.clone(), s.to_string()))).await.into_iter().collect()
    }
    
    pub fn delete(&mut self) {
        self.dict = Arc::new(Vec::new());
    }
    pub fn convert(&self,data:&str)->String{
        block_on(self.convert_future(data))
    }
}
async fn convert_slice(dicts: Arc<Vec<Vec<DictEntry>>>, mut data: String) -> String {
    'a: for dicts in dicts.iter() {
        for dict in dicts {
            let (res, flag) = dict.convert(&data);
            data = res;
            // if flag{
            // 	continue 'a;
            // }
        }
    }
    return data;
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct ConvertorBuild(ConverterBuild);

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ConvertorBuild{
    pub fn new()->Self{
        Self(ConverterBuild::new())
    }
    pub fn group(&mut self){
        self.0.group();
    }
    pub fn adddict(&mut self,data: &str){
        self.0.adddict(data);
    }
    pub fn build(self)->Convertor{
        Convertor(self.0.build())
    }
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct Convertor(Converter);

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Convertor{
    pub fn convert(&self,data:&str)->String{
        self.0.convert(data)
    }
}
