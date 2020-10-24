use std::collections::BTreeSet;
use lazy_static::lazy_static;
lazy_static!{
    static ref PUNCTUATIONS:BTreeSet<char> = "，。？《》〈〉“”‘’；：；！、（）".chars().collect();
    static ref INSIDE:BTreeSet<char> = "LNPQSU".chars().collect();
}
pub(crate) fn exclude_char(c: char) -> bool {
    let n = c as usize;
    INSIDE.get(&c).is_none()
        && (c.is_ascii()
            || (c.is_whitespace() || c.is_control())
            || PUNCTUATIONS.get(&c).is_some()
            || (!((n >= 12295 && n <= 40916) || n == 65294 || (n >= 131134 && n <= 201372))))
}
