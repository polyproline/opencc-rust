use futures::executor::block_on;
use futures::future::join_all;
use wasm_bindgen::prelude::*;

use std::sync::Arc;

use crate::chars::exclude_char;
use crate::dict::DictEntry;

/// 对于字符串进行分割，避免比较序列过长
/// 也可并行处理(未实现)
const MIN_SYNCOPATION: usize = 512;
const MAX_SYNCOPATION: usize = 1024;

#[wasm_bindgen]
pub struct ConverterBuild {
    res: Vec<Vec<DictEntry>>,
    tmp: Vec<DictEntry>,
    min: usize,
}
#[wasm_bindgen]
pub struct Converter {
    dict: Arc<Vec<Vec<DictEntry>>>,
    min: u8,
}
#[wasm_bindgen]
impl ConverterBuild {
    pub fn new() -> Self {
        Self {
            res: Vec::new(),
            tmp: Vec::new(),
            min: usize::max_value(),
        }
    }
    pub fn adddict(&mut self, data: &str) {
        let mut min = usize::max_value();
        let mut keys = Vec::new();
        let mut values = Vec::new();

        for line in data.lines() {
            let key_v: Vec<&str> = line.split('\t').collect();
            if key_v.is_empty() {
                continue;
            }
            assert_eq!(key_v.len(), 2);
            let (key, value) = unsafe {
                let key = key_v.get_unchecked(0).to_string();
                let value = key_v.get_unchecked(1);
                let values: Vec<&str> = value.split(' ').collect();
                let value: String = values.into_iter().map(|t| t.to_string()).next().unwrap();
                (key, value)
            };
            assert!(key.len() > 0, "key\t{}\t{:?}", key, value);
            assert!(key.chars().filter(|c|exclude_char(*c)).next().is_none(),"key 值非法，可能未更新");
            min = min.min(key.len());
            keys.push(key);
            values.push(value);
        }
        assert!(min < u8::max_value() as usize);
        let t = DictEntry::new(min as u8, keys, values);
        self.min = self.min.min(min);
        self.tmp.push(t);
    }
    pub fn group(&mut self) {
        assert_ne!(self.tmp.len(), 0);
        let t = self.tmp.split_off(0);
        self.res.push(t);
    }
    pub fn build(&mut self) -> Converter {
        assert_eq!(self.tmp.len(), 0);
        assert_ne!(self.res.len(), 0);
        Converter {
            min: self.min as u8,
            dict: Arc::new(self.res.split_off(0)),
        }
    }
}
#[wasm_bindgen]
impl Converter {
    pub fn convert(&self, data: &str) -> String {
        if data.len() < self.min as usize {
            return data.to_string();
        }
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
        /// 这里可以用 async-std 或 tokio 库多线程 并行加速
        block_on(join_all(
            datas.map(|s| convert_slice(self.dict.clone(), s.to_string())),
        ))
        .into_iter()
        .collect()
    }
    pub fn delete(&mut self) {
        self.min = u8::max_value();
        self.dict = Arc::new(Vec::new());
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
