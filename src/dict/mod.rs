use self::keytree::{KeyPoint, KeyTree};
use crate::chars::exclude_char;
pub(crate) struct DictEntry {
    min: u8,
    keys: KeyTree, //可以有更好的方案
    values: Vec<Vec<String>>,
}
impl DictEntry {
    pub(crate) fn new(min: u8, keys: Vec<String>, values: Vec<Vec<String>>) -> Self {
        let mut keyt = KeyTree::new();
        for (n, i) in keys.into_iter().enumerate() {
            keyt.insert(i.chars(), n);
        }
        Self {
            min: min,
            keys: keyt,
            values: values,
        }
    }
    pub(crate) fn convert(&self, data: &str) -> (String, bool) {
        if data.len() < self.min as usize {
            return (data.to_string(), true);
        }
        let mut iter = data.chars().peekable();
        let mut res = String::with_capacity(data.len());
        let mut point = KeyPoint::new(&self.keys);
        let mut flag = true;
        loop {
            while let Some(c) = iter.peek().map(|t| t.clone()) {
                if exclude_char(c) {
                    res.push(c);
                    iter.next();
                } else {
                    break;
                }
            }
            if iter.peek().is_some() {
                if let Some((index, t)) = point.matchchars(iter.clone()) {
                    res.push_str(unsafe { self.values.get_unchecked(index).get_unchecked(0) });
                    iter = t;
                    flag = false;
                } else {
                    if let Some(c) = iter.next() {
                        res.push(c);
                    } else {
                        break;
                    }
                }
                point = KeyPoint::new(&self.keys);
            } else {
                break;
            }
        }
        (res, flag)
    }
}
mod keytree;
