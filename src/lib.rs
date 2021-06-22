pub mod traits;

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
    pub fn as_num(&self) -> String {
        format!(
            "{}{}{}",
            self.user.as_num(),
            self.group.as_num(),
            self.other.as_num()
        )
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
    pub fn from_num(num: &str) -> Result<Self, ParseError> {
        #[inline]
        fn next_val(pos: &mut usize, chars: &mut Chars) -> Result<Perm, ParseError> {
            let c = chars
                .next()
                .ok_or_else(|| ParseError::UnexpectedEoi { pos: *pos })?;
            *pos += 1;
            Perm::from_num(&c.to_string()).map_err(|err| match err {
                ParseError::UnexpectedChar {
                    c,
                    pos: p,
                    expected,
                } => ParseError::UnexpectedChar {
                    c,
                    pos: p + *pos,
                    expected,
                },
                ParseError::UnexpectedEoi { pos: p } => ParseError::UnexpectedEoi { pos: p + *pos },
            })
        }

        let mut chars = num.chars();
        let mut pos = 0;
        Ok(Self {
            user: next_val(&mut pos, &mut chars)?,
            group: next_val(&mut pos, &mut chars)?,
            other: next_val(&mut pos, &mut chars)?,
        })
    }

    #[inline]
    pub fn from_sym(sym: &str) -> Result<Self, ParseError> {
        #[inline]
        fn shift_err(diff: usize) -> impl Fn(ParseError) -> ParseError {
            move |err: ParseError| match err {
                ParseError::UnexpectedEoi { pos } => ParseError::UnexpectedEoi { pos: pos + diff },
                ParseError::UnexpectedChar { c, pos, expected } => ParseError::UnexpectedChar {
                    c,
                    pos: pos + diff,
                    expected,
                },
            }
        }

        let user = Perm::from_sym_full(&sym.chars().take(3).collect::<String>())?;
        let group = Perm::from_sym_full(&sym.chars().skip(3).take(3).collect::<String>())
            .map_err(shift_err(3))?;
        let other = Perm::from_sym_full(&sym.chars().skip(6).take(3).collect::<String>())
            .map_err(shift_err(6))?;

        if let Some(c) = sym.chars().nth(9) {
            Err(ParseError::UnexpectedChar {
                c,
                pos: 9,
                expected: None,
            })
        } else {
            Ok(Self { user, group, other })
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
    pub fn as_num(&self) -> String {
        ((if self.read { 4 } else { 0 })
            + (if self.write { 2 } else { 0 })
            + (if self.execute { 1 } else { 0 }))
        .to_string()
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
    pub fn from_num(num: &str) -> Result<Self, ParseError> {
        let tup = match num {
            "7" => (true, true, true),
            "6" => (true, true, false),
            "5" => (true, false, true),
            "4" => (true, false, false),
            "3" => (false, true, true),
            "2" => (false, true, false),
            "1" => (false, false, true),
            "0" => (false, false, false),
            "" => return Err(ParseError::UnexpectedEoi { pos: 0 }),
            c => {
                return Err(ParseError::UnexpectedChar {
                    c: c.chars().next().unwrap(),
                    pos: 0,
                    expected: Some(
                        (0..=7)
                            .map(|n| std::char::from_digit(n, 10).unwrap())
                            .collect(),
                    ),
                })
            }
        };

        Ok(tup.into())
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
                    expected: Some(vec![e, '-']),
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

        if let Some(c) = chars.next() {
            Err(ParseError::UnexpectedChar {
                c,
                pos,
                expected: None,
            })
        } else {
            Ok(Self {
                read,
                write,
                execute,
            })
        }
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

impl From<(bool, bool, bool)> for Perm {
    #[inline]
    fn from(tup: (bool, bool, bool)) -> Self {
        Self {
            read: tup.0,
            write: tup.1,
            execute: tup.2,
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

    // TODO: Add failing tests
    #[test]
    fn test_perm_num() -> Result<(), Box<dyn std::error::Error>> {
        macro_rules! test_perm_num {
            ($c:expr, $r: expr, $w:expr, $x:expr) => {{
                let p = perm!($r, $w, $x);
                assert_eq!($c, p.as_num());
                assert_eq!(p, Perm::from_num($c)?)
            }};
        }

        test_perm_num!("7", true, true, true);
        test_perm_num!("6", true, true, false);
        test_perm_num!("5", true, false, true);
        test_perm_num!("4", true, false, false);
        test_perm_num!("3", false, true, true);
        test_perm_num!("2", false, true, false);
        test_perm_num!("1", false, false, true);
        test_perm_num!("0", false, false, false);

        Ok(())
    }

    #[test]
    fn test_perm_sym() -> Result<(), Box<dyn std::error::Error>> {
        macro_rules! test_perm_sym {
            ($s:expr, $fs:expr, $r: expr, $w:expr, $x:expr) => {
                assert_eq!($s, perm!($r, $w, $x).as_sym());
                assert_eq!(perm!($r, $w, $x), Perm::from_sym_full($fs)?)
            };
        }

        macro_rules! test_perm_sym_e {
            ($fs:expr, $err:expr) => {
                assert_eq!($err, Perm::from_sym_full($fs).unwrap_err())
            };
        }

        test_perm_sym!("rwx", "rwx", true, true, true);
        test_perm_sym!("rw", "rw-", true, true, false);
        test_perm_sym!("rx", "r-x", true, false, true);
        test_perm_sym!("r", "r--", true, false, false);
        test_perm_sym!("wx", "-wx", false, true, true);
        test_perm_sym!("w", "-w-", false, true, false);
        test_perm_sym!("x", "--x", false, false, true);

        test_perm_sym_e!("", ParseError::UnexpectedEoi { pos: 0 });
        test_perm_sym_e!("r", ParseError::UnexpectedEoi { pos: 1 });
        test_perm_sym_e!("rw", ParseError::UnexpectedEoi { pos: 2 });
        test_perm_sym_e!(
            "x",
            ParseError::UnexpectedChar {
                pos: 0,
                c: 'x',
                expected: Some(vec!['r', '-'])
            }
        );
        test_perm_sym_e!(
            "rr",
            ParseError::UnexpectedChar {
                pos: 1,
                c: 'r',
                expected: Some(vec!['w', '-'])
            }
        );
        test_perm_sym_e!(
            "rwxr",
            ParseError::UnexpectedChar {
                pos: 3,
                c: 'r',
                expected: None
            }
        );

        Ok(())
    }

    // TODO: Add failing tests
    #[test]
    fn test_mode_num() -> Result<(), Box<dyn std::error::Error>> {
        macro_rules! test_mode_num {
            ($c:expr; $ur:expr, $uw:expr, $ux:expr; $gr:expr, $gw:expr, $gx:expr; $or:expr, $ow:expr, $ox: expr) => {
                assert_eq!(
                    $c,
                    mode!($ur, $uw, $ux, $gr, $gw, $gx, $or, $ow, $ox).as_num()
                );
                assert_eq!(
                    mode!($ur, $uw, $ux, $gr, $gw, $gx, $or, $ow, $ox),
                    Mode::from_num($c)?
                )
            };
        }

        test_mode_num!("777"; true, true, true; true, true, true; true, true, true);
        test_mode_num!("755"; true, true, true; true, false, true; true, false, true);
        test_mode_num!("666"; true, true, false; true, true, false; true, true, false);
        test_mode_num!("644"; true, true, false; true, false, false; true, false, false);
        test_mode_num!("400"; true, false, false; false, false, false; false, false, false);

        Ok(())
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

        macro_rules! test_mode_sym_e {
            ($fs:expr, $err:expr) => {
                assert_eq!($err, Mode::from_sym($fs).unwrap_err())
            };
        }

        test_mode_sym!("rwxrwxrwx"; true, true, true; true, true, true; true, true, true);
        test_mode_sym!("rwxr-xr-x"; true, true, true; true, false, true; true, false, true);
        test_mode_sym!("rw-rw-rw-"; true, true, false; true, true, false; true, true, false);
        test_mode_sym!("rw-r--r--"; true, true, false; true, false, false; true, false, false);
        test_mode_sym!("r--------"; true, false, false; false, false, false; false, false, false);

        test_mode_sym_e!("r", ParseError::UnexpectedEoi { pos: 1 });
        test_mode_sym_e!("rwx", ParseError::UnexpectedEoi { pos: 3 });
        test_mode_sym_e!(
            "rwxrx",
            ParseError::UnexpectedChar {
                pos: 4,
                c: 'x',
                expected: Some(vec!['w', '-'])
            }
        );
        test_mode_sym_e!(
            "rwxr-xr-x-",
            ParseError::UnexpectedChar {
                pos: 9,
                c: '-',
                expected: None
            }
        );

        Ok(())
    }
}
