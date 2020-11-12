use lazy_static::lazy_static;
use std::collections::BTreeSet;
lazy_static! {
    static ref PUNCTUATIONS: BTreeSet<char> = "，。？《》〈〉“”‘’；：；！、（）".chars().collect();
    static ref INSIDE: BTreeSet<char> = "LNPQSU".chars().collect();
}
/// 根据 opencc-data@1.0.5 key值 推出
/// 硬编码
/// 加快匹配速度
pub(crate) fn exclude_char(c: char) -> bool {
    let n = c as usize;
    if c.is_ascii() && INSIDE.get(&c.to_ascii_uppercase()).is_some() {
        false
    } else {
        c.is_ascii()
            || (c.is_whitespace() || c.is_control())
            || PUNCTUATIONS.get(&c).is_some()
            || (!((n >= 12295 && n <= 40916) || n == 65294 || (n >= 131134 && n <= 201372)))
    }
}