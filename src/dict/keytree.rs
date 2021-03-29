use std::collections::{BTreeMap,HashMap};
use std::iter::Peekable;

use crate::chars::exclude_char;

pub(crate) struct KeyTree {
    next: HashMap<char, Box<KeyTree>>,
    result: Option<usize>,
}
impl KeyTree {
    pub(crate) fn new() -> Self {
        Self {
            next: HashMap::new(),
            result: None,
        }
    }
    pub(crate) fn insert<T: Iterator<Item = char>>(&mut self, mut iter: T, index: usize) {
        if let Some(c) = iter.next() {
            let n = self.next.entry(c).or_insert_with(|| Box::new(Self::new()));
            n.insert(iter, index);
        } else {
            assert!(self.result.is_none(), self.result);
            self.result = Some(index);
        }
    }
}
pub(super) struct KeyPoint<'a> {
    p: &'a KeyTree,
}
impl<'a> KeyPoint<'a> {
    pub(super) fn new(t: &'a KeyTree) -> Self {
        Self { p: t }
    }
    pub(super) fn matchchars<I: Iterator<Item = char> + Clone>(
        &mut self,
        mut iter: Peekable<I>,
    ) -> Option<(usize, Peekable<I>)> {
        let mut stack = Vec::new();
        while let Some(mut c) = iter.peek().map(|t| t.clone()) {
            if exclude_char(c) {
                break;
            }
            c.make_ascii_uppercase();
            if let Some(n) = self.p.next.get(&c) {
                self.p = n;
                iter.next();
                if let Some(res) = self.p.result{
                    stack.push((res, iter.clone()));
                }
            } else {
                break;
            }
        }
        stack.pop()
    }
}
