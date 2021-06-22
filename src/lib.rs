use std::str::Chars;

use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Mode {
    pub user: Perm,
    pub group: Perm,
    pub other: Perm,
}

#[derive(Debug, PartialEq)]
pub struct Perm {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

#[derive(Debug, PartialEq, Error)]
pub enum ParseError {
    #[error("invalid character encountered")]
    UnexpectedChar {
        pos: usize,
        c: char,
        expected: Option<Vec<char>>,
    },
    #[error("unexepected end-of-input")]
    UnexpectedEoi { pos: usize },
}

impl Mode {
    #[inline]
    pub fn new(user: Perm, group: Perm, other: Perm) -> Self {
        Self { user, group, other }
    }

    #[inline]
    pub fn as_num(&self) -> u16 {
        100 * self.user.as_num() + 10 * self.group.as_num() + self.other.as_num()
    }

    #[inline]
    pub fn as_sym(&self) -> String {
        format!(
            "{}{}{}",
            self.user.as_sym_full(),
            self.group.as_sym_full(),
            self.other.as_sym_full()
        )
    }

    #[inline]
    pub fn from_sym(sym: &str) -> Result<Self, ParseError> {
        match sym.len() {
            9 => {
                let user = Perm::from_sym_full(sym)?;
                let group = Perm::from_sym_full(&sym.chars().skip(3).collect::<String>())?;
                let other = Perm::from_sym_full(&sym.chars().skip(6).collect::<String>())?;

                Ok(Self { user, group, other })
            }
            pos @ 0..=8 => Err(ParseError::UnexpectedEoi { pos }),
            _ => Err(ParseError::UnexpectedChar {
                pos: 9,
                c: sym.chars().nth(9).unwrap(),
                expected: None,
            }),
        }
    }

    #[inline]
    pub fn diff(&self, other: &Self) -> ModeDiff {
        ModeDiff {
            user: self.user.diff(&other.user),
            group: self.group.diff(&other.group),
            other: self.other.diff(&other.other),
        }
    }
}

impl Perm {
    #[inline]
    pub fn as_num(&self) -> u16 {
        (if self.read { 4 } else { 0 })
            + (if self.write { 2 } else { 0 })
            + (if self.execute { 1 } else { 0 })
    }

    #[inline]
    pub fn as_sym(&self) -> String {
        let r = if self.read { "r" } else { "" };
        let w = if self.write { "w" } else { "" };
        let x = if self.execute { "x" } else { "" };
        format!("{}{}{}", r, w, x)
    }

    #[inline]
    pub fn as_sym_full(&self) -> String {
        let r = if self.read { "r" } else { "-" };
        let w = if self.write { "w" } else { "-" };
        let x = if self.execute { "x" } else { "-" };
        format!("{}{}{}", r, w, x)
    }

    #[inline]
    pub fn from_sym_full(sym: &str) -> Result<Self, ParseError> {
        #[inline]
        fn process_char(e: char, pos: &mut usize, chars: &mut Chars) -> Result<bool, ParseError> {
            let r = match chars.next() {
                None => Err(ParseError::UnexpectedEoi { pos: *pos }),
                Some(c) if c == e => Ok(true),
                Some('-') => Ok(false),
                Some(c) => Err(ParseError::UnexpectedChar {
                    c,
                    pos: *pos,
                    expected: Some(vec![c, '-']),
                }),
            };
            *pos += 1;
            r
        }

        let mut chars = sym.chars();

        let mut pos = 0;
        let read = process_char('r', &mut pos, &mut chars)?;
        let write = process_char('w', &mut pos, &mut chars)?;
        let execute = process_char('x', &mut pos, &mut chars)?;

        Ok(Self {
            read,
            write,
            execute,
        })
    }

    #[inline]
    pub fn diff(&self, other: &Self) -> PermDiff {
        PermDiff {
            read: diff(self.read, other.read),
            write: diff(self.write, other.write),
            execute: diff(self.execute, other.execute),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ModeDiff {
    pub user: PermDiff,
    pub group: PermDiff,
    pub other: PermDiff,
}

#[derive(Debug, PartialEq)]
pub struct PermDiff {
    pub read: DiffOp,
    pub write: DiffOp,
    pub execute: DiffOp,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiffOp {
    Plus,
    Same,
    Minus,
}

#[inline]
fn diff(a: bool, b: bool) -> DiffOp {
    use DiffOp::*;

    if a ^ b {
        if a {
            Minus
        } else {
            Plus
        }
    } else {
        Same
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! perm {
        ($r:expr, $w:expr, $x:expr) => {
            Perm {
                read: $r,
                write: $w,
                execute: $x,
            }
        };
    }

    macro_rules! mode {
        ($u:expr, $g:expr, $o:expr) => {
            Mode {
                user: $u,
                group: $g,
                other: $o,
            }
        };
        ($ur:expr, $uw:expr, $ux:expr, $gr:expr, $gw:expr, $gx:expr, $or:expr, $ow:expr, $ox: expr) => {
            Mode {
                user: perm!($ur, $uw, $ux),
                group: perm!($gr, $gw, $gx),
                other: perm!($or, $ow, $ox),
            }
        };
    }

    #[test]
    fn test_perm_as_num() {
        macro_rules! test_perm_num {
            ($c:expr, $r: expr, $w:expr, $x:expr) => {
                assert_eq!($c, perm!($r, $w, $x).as_num())
            };
        }

        test_perm_num!(7, true, true, true);
        test_perm_num!(6, true, true, false);
        test_perm_num!(5, true, false, true);
        test_perm_num!(4, true, false, false);
        test_perm_num!(3, false, true, true);
        test_perm_num!(2, false, true, false);
        test_perm_num!(1, false, false, true);
    }

    #[test]
    fn test_perm_as_sym() -> Result<(), Box<dyn std::error::Error>> {
        macro_rules! test_perm_sym {
            ($s:expr, $fs:expr, $r: expr, $w:expr, $x:expr) => {
                assert_eq!($s, perm!($r, $w, $x).as_sym());
                assert_eq!(perm!($r, $w, $x), Perm::from_sym_full($fs)?)
            };
        }

        test_perm_sym!("rwx", "rwx", true, true, true);
        test_perm_sym!("rw", "rw-", true, true, false);
        test_perm_sym!("rx", "r-x", true, false, true);
        test_perm_sym!("r", "r--", true, false, false);
        test_perm_sym!("wx", "-wx", false, true, true);
        test_perm_sym!("w", "-w-", false, true, false);
        test_perm_sym!("x", "--x", false, false, true);

        Ok(())
    }

    #[test]
    fn test_mode_as_num() {
        macro_rules! test_mode_num {
            ($c:expr; $ur:expr, $uw:expr, $ux:expr; $gr:expr, $gw:expr, $gx:expr; $or:expr, $ow:expr, $ox: expr) => {
                assert_eq!(
                    $c,
                    mode!($ur, $uw, $ux, $gr, $gw, $gx, $or, $ow, $ox).as_num()
                )
            };
        }

        test_mode_num!(777; true, true, true; true, true, true; true, true, true);
        test_mode_num!(755; true, true, true; true, false, true; true, false, true);
        test_mode_num!(666; true, true, false; true, true, false; true, true, false);
        test_mode_num!(644; true, true, false; true, false, false; true, false, false);
        test_mode_num!(400; true, false, false; false, false, false; false, false, false);
    }

    #[test]
    fn test_mode_sym() -> Result<(), Box<dyn std::error::Error>> {
        macro_rules! test_mode_sym {
            ($c:expr; $ur:expr, $uw:expr, $ux:expr; $gr:expr, $gw:expr, $gx:expr; $or:expr, $ow:expr, $ox: expr) => {
                assert_eq!(
                    $c,
                    mode!($ur, $uw, $ux, $gr, $gw, $gx, $or, $ow, $ox).as_sym()
                );
                assert_eq!(
                    mode!($ur, $uw, $ux, $gr, $gw, $gx, $or, $ow, $ox),
                    Mode::from_sym($c)?
                )
            };
        }

        test_mode_sym!("rwxrwxrwx"; true, true, true; true, true, true; true, true, true);
        test_mode_sym!("rwxr-xr-x"; true, true, true; true, false, true; true, false, true);
        test_mode_sym!("rw-rw-rw-"; true, true, false; true, true, false; true, true, false);
        test_mode_sym!("rw-r--r--"; true, true, false; true, false, false; true, false, false);
        test_mode_sym!("r--------"; true, false, false; false, false, false; false, false, false);

        Ok(())
    }
}
