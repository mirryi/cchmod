use crate::{Mode, Perm};

pub trait AsNum {
    fn as_num(&self) -> String;
}

pub trait AsSym {
    fn as_sym(&self) -> String;
}

impl AsNum for Mode {
    #[inline]
    fn as_num(&self) -> String {
        self.as_num()
    }
}

impl AsSym for Mode {
    #[inline]
    fn as_sym(&self) -> String {
        self.as_sym()
    }
}

impl AsNum for Perm {
    #[inline]
    fn as_num(&self) -> String {
        self.as_num()
    }
}

impl AsSym for Perm {
    #[inline]
    fn as_sym(&self) -> String {
        self.as_sym()
    }
}
