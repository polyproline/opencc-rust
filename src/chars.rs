static PUNCTUATIONS: &str = "，。？《》〈〉“”‘’；：；！、（）";

pub(crate) fn exclude_char(c: char) -> bool {
    let n = c as usize;
    (c.is_ascii() || c.is_whitespace() || c.is_control())
        || PUNCTUATIONS.find(c).is_some()
        || (!((n >= 12295 && n <= 40916) || n == 65294 || (n >= 131134 && n <= 201372)))
}
