use std::sync::Arc;

mod keytree;

use self::keytree::{KeyPoint};
use crate::chars::exclude_char;
pub(crate) use self::keytree::KeyTree;

pub(crate) struct DictEntry {
    keys: KeyTree, //可以有更好的方案
    values: Vec<String>,
}
impl DictEntry {
    pub(crate) fn new(keys:KeyTree,value:Vec<String>) -> Self {
        Self {
            keys: keys,
            values: value,
        }
    }
    pub(crate) fn convert(&self, data: &str) -> (String, bool) {
        let mut iter = data.chars().peekable();
        let mut res = String::with_capacity(data.len());
        let mut point = KeyPoint::new(&self.keys);
        let mut flag = true;
        while let Some(c) = iter.peek() {
            if exclude_char(*c) {
                res.push(*c);
                iter.next();
            } else {
                if let Some((index, t)) = point.matchchars(iter.clone()) {
                    res.push_str(unsafe { self.values.get_unchecked(index) });
                    iter = t;
                    flag = false;
                } else {
                    if let Some(c) = iter.next() {
                        res.push(c);
                    } else {
                        break;
                    }
                }
            }
            point = KeyPoint::new(&self.keys);
        }
        (res, flag)
    }
}
