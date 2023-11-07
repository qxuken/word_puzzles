use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word(Cow<'static, str>);

impl Word {
    pub fn new(s: Cow<'static, str>) -> Word {
        Word(s)
    }

    pub fn value(&self) -> &Cow<'static, str> {
        &self.0
    }
}

impl From<Cow<'static, str>> for Word {
    fn from(value: Cow<'static, str>) -> Self {
        Word::new(value)
    }
}
impl From<&'static str> for Word {
    fn from(value: &'static str) -> Self {
        Word::new(Cow::Borrowed(value))
    }
}
