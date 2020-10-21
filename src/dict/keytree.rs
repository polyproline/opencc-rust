use std::collections::HashMap;
use std::iter::Peekable;

use crate::chars::exclude_char;

pub(super) struct KeyTree {
    next: HashMap<char, Box<KeyTree>>,
    result: Option<usize>,
}
impl KeyTree {
    pub(super) fn new() -> Self {
        Self {
            next: HashMap::new(),
            result: None,
        }
    }
    pub(super) fn insert<T: Iterator<Item = char>>(&mut self, mut iter: T, index: usize) {
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
        while let Some(c) = iter.peek().map(|t| t.clone()) {
            if exclude_char(c) {
                break;
            }
            if let Some(n) = self.p.next.get(&c) {
                self.p = n;
                iter.next();
                stack.push((self.p.result, iter.clone()));
            } else {
                break;
            }
        }
        while let Some((res, iter)) = stack.pop() {
            if let Some(index) = res {
                return Some((index, iter));
            }
        }
        return None;
    }
}
