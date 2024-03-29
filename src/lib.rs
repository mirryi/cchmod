pub mod perm;
pub mod traits;

use std::str::Chars;

use thiserror::Error;

/// File system object mode.
#[derive(Debug, PartialEq)]
pub struct Mode {
    /// Permission set for the owning user.
    pub user: Perm,
    /// Permission set for the group.
    pub group: Perm,
    /// Permission set for all other users.
    pub other: Perm,
}

/// File system object permissions.
///
/// See [`perm`] for predefined constant values.
#[derive(Debug, PartialEq)]
pub struct Perm {
    /// Flag indicating whether *read* permission is granted.
    pub read: bool,
    /// Flag indicating whether *write* permission is granted.
    pub write: bool,
    /// Flag indicating whether *execute* permission is granted.
    pub execute: bool,
}

/// Error encountered when parsing a string into a [`Mode`] or [`Perm`].
#[derive(Debug, PartialEq, Error)]
pub enum ParseError {
    /// An unexpected character was encountered while parsing (e.g. an 'r' when an 'x' or '-' was
    /// expected, or any character after EOI was expected).
    #[error("invalid character encountered")]
    UnexpectedChar {
        /// The position (zero-indexed) of the unexpected character.
        pos: usize,
        /// The character encountered.
        c: char,
        /// A list of expected characters at this position. If [`None`], EOI was expected.
        expected: Option<Vec<char>>,
    },
    /// End-of-input was encountered when more input was expected.
    #[error("unexepected end-of-input")]
    UnexpectedEoi {
        /// The position (zero-indexed) where another character was expected.
        pos: usize,
    },
}

impl Mode {
    /// Create a new [`Mode`].
    #[inline]
    pub const fn new(user: Perm, group: Perm, other: Perm) -> Self {
        Self { user, group, other }
    }

    /// Get the octal representation the [`Mode`].
    ///
    /// # Examples
    ///
    /// ```
    /// use cchmod::{Mode, Perm};
    ///
    /// let m = Mode::new(Perm::new(true, true, true),
    ///                   Perm::new(true, false, true),
    ///                   Perm::new(true, false, true));
    ///
    /// assert_eq!("755", m.as_num());
    /// ```
    #[inline]
    pub fn as_num(&self) -> String {
        format!(
            "{}{}{}",
            self.user.as_num(),
            self.group.as_num(),
            self.other.as_num()
        )
    }

    /// Get the symbolic representation the [`Mode`].
    ///
    /// # Examples
    ///
    /// ```
    /// use cchmod::{Mode, Perm};
    ///
    /// let m = Mode::new(Perm::new(true, true, true),
    ///                   Perm::new(true, false, true),
    ///                   Perm::new(true, false, true));
    ///
    /// assert_eq!("rwxr-xr-x", m.as_sym());
    /// ```
    #[inline]
    pub fn as_sym(&self) -> String {
        format!(
            "{}{}{}",
            self.user.as_sym_full(),
            self.group.as_sym_full(),
            self.other.as_sym_full()
        )
    }

    /// Create a [`Mode`] from its octal form, returning [`ParseError`] if the input is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use cchmod::{Mode, Perm, ParseError};
    ///
    /// assert_eq!(
    ///     Mode::new(Perm::new(true, true, true),
    ///               Perm::new(true, false, true),
    ///               Perm::new(true, false, true)),
    ///     Mode::from_num("755").unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     ParseError::UnexpectedChar { pos: 3, c: '8', expected: None },
    ///     Mode::from_num("6008").unwrap_err()
    /// );
    /// ```
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
        let user = next_val(&mut pos, &mut chars)?;
        let group = next_val(&mut pos, &mut chars)?;
        let other = next_val(&mut pos, &mut chars)?;

        if let Some(c) = chars.next() {
            Err(ParseError::UnexpectedChar {
                pos,
                c,
                expected: None,
            })
        } else {
            Ok(Self { user, group, other })
        }
    }

    /// Create a [`Mode`] from its symbolic form, returning [`ParseError`] if the input is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use cchmod::{Mode, Perm, ParseError};
    ///
    /// assert_eq!(
    ///     Mode::new(Perm::new(true, true, true),
    ///               Perm::new(true, false, true),
    ///               Perm::new(true, false, true)),
    ///     Mode::from_sym("rwxr-xr-x").unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     ParseError::UnexpectedEoi { pos: 6 },
    ///     Mode::from_sym("rwxr-x").unwrap_err()
    /// );
    /// assert_eq!(
    ///     ParseError::UnexpectedChar { pos: 9, c: 'r', expected: None },
    ///     Mode::from_sym("rwxr-xr-xr").unwrap_err()
    /// );
    /// ```
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

    /// Compute the diff ([`ModeDiff`]) between two modes.
    ///
    /// # Examples
    ///
    /// ```
    /// use cchmod::{Mode, ModeDiff, PermDiff, DiffOp::*};
    ///
    /// let a = Mode::from_num("777").unwrap();
    /// let b = Mode::from_num("644").unwrap();
    ///
    /// assert_eq!(
    ///     ModeDiff {
    ///         user: PermDiff { read: Same, write: Same, execute: Minus },
    ///         group: PermDiff { read: Same, write: Minus, execute: Minus },
    ///         other: PermDiff { read: Same, write: Minus, execute: Minus },
    ///     },
    ///     a.diff(&b)
    /// );
    /// ```
    #[inline]
    pub const fn diff(&self, other: &Self) -> ModeDiff {
        ModeDiff {
            user: self.user.diff(&other.user),
            group: self.group.diff(&other.group),
            other: self.other.diff(&other.other),
        }
    }
}

impl Perm {
    /// Create a new [`Perm`].
    #[inline]
    pub const fn new(read: bool, write: bool, execute: bool) -> Self {
        Self {
            read,
            write,
            execute,
        }
    }

    /// Get the octal representation the [`Perm`].
    ///
    /// # Examples
    ///
    /// ```
    /// use cchmod::Perm;
    ///
    /// let p1 = Perm::new(true, true, false);
    /// assert_eq!("6", p1.as_num());
    ///
    /// let p2 = Perm::new(true, false, true);
    /// assert_eq!("5", p2.as_num());
    /// ```
    #[inline]
    pub fn as_num(&self) -> String {
        ((if self.read { 4 } else { 0 })
            + (if self.write { 2 } else { 0 })
            + (if self.execute { 1 } else { 0 }))
        .to_string()
    }

    /// Get the symbolic representation, with ungranted permissions omitted, of the [`Perm`].
    ///
    /// # Examples
    ///
    /// ```
    /// use cchmod::Perm;
    ///
    /// let p1 = Perm::new(true, true, false);
    /// assert_eq!("rw", p1.as_sym());
    ///
    /// let p2 = Perm::new(true, false, true);
    /// assert_eq!("rx", p2.as_sym());
    /// ```
    #[inline]
    pub fn as_sym(&self) -> String {
        let r = if self.read { "r" } else { "" };
        let w = if self.write { "w" } else { "" };
        let x = if self.execute { "x" } else { "" };
        format!("{}{}{}", r, w, x)
    }

    /// Get the symbolic representation, with ungranted permissions as '-', of the [`Perm`].
    ///
    /// # Examples
    ///
    /// ```
    /// use cchmod::Perm;
    ///
    /// let p1 = Perm::new(true, true, false);
    /// assert_eq!("rw-", p1.as_sym_full());
    ///
    /// let p2 = Perm::new(true, false, true);
    /// assert_eq!("r-x", p2.as_sym_full());
    /// ```
    #[inline]
    pub fn as_sym_full(&self) -> String {
        let r = if self.read { "r" } else { "-" };
        let w = if self.write { "w" } else { "-" };
        let x = if self.execute { "x" } else { "-" };
        format!("{}{}{}", r, w, x)
    }

    /// Create a [`Perm`] from its octal form, returning [`ParseError`] if the input is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use cchmod::{Perm, ParseError};
    ///
    /// assert_eq!(Perm::new(true, true, true), Perm::from_num("7").unwrap());
    /// assert_eq!(Perm::new(true, false, false), Perm::from_num("4").unwrap());
    ///
    /// assert_eq!(
    ///     ParseError::UnexpectedEoi { pos: 0 },
    ///     Perm::from_num("").unwrap_err()
    /// );
    /// assert_eq!(
    ///     ParseError::UnexpectedChar {
    ///         pos: 0,
    ///         c: '8',
    ///         expected: Some(vec!['0', '1', '2', '3', '4', '5', '6', '7'])
    ///     },
    ///     Perm::from_num("8").unwrap_err()
    /// );
    /// ```
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

    /// Create a [`Perm`] from its symbolic form, returning [`ParseError`] if the input is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use cchmod::{Perm, ParseError};
    ///
    /// assert_eq!(Perm::new(true, true, true), Perm::from_sym_full("rwx").unwrap());
    /// assert_eq!(Perm::new(true, false, false), Perm::from_sym_full("r--").unwrap());
    ///
    /// assert_eq!(
    ///     ParseError::UnexpectedEoi { pos: 2 },
    ///     Perm::from_sym_full("rw").unwrap_err()
    /// );
    /// assert_eq!(
    ///     ParseError::UnexpectedChar { pos: 3, c: 'r', expected: None },
    ///     Perm::from_sym_full("rwxr").unwrap_err()
    /// );
    /// ```
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

    /// Compute the diff ([`PermDiff`]) between two [`Perm`]s.
    ///
    /// # Examples
    ///
    /// ```
    /// use cchmod::{Perm, PermDiff, DiffOp::*};
    ///
    /// let a = Perm::from_num("7").unwrap();
    /// let b = Perm::from_num("6").unwrap();
    ///
    /// assert_eq!(
    ///     PermDiff { read: Same, write: Same, execute: Minus },
    ///     a.diff(&b)
    /// );
    /// ```
    #[inline]
    pub const fn diff(&self, other: &Self) -> PermDiff {
        PermDiff {
            read: bool_diff(self.read, other.read),
            write: bool_diff(self.write, other.write),
            execute: bool_diff(self.execute, other.execute),
        }
    }
}

impl From<(bool, bool, bool)> for Perm {
    /// Create a [`Perm`] from a tuple of boolean with form `(user, group, other)`.
    #[inline]
    fn from(tup: (bool, bool, bool)) -> Self {
        Self {
            read: tup.0,
            write: tup.1,
            execute: tup.2,
        }
    }
}

/// The diff between two Modes.
#[derive(Debug, PartialEq)]
pub struct ModeDiff {
    pub user: PermDiff,
    pub group: PermDiff,
    pub other: PermDiff,
}

/// The diff between two Perms.
#[derive(Debug, PartialEq)]
pub struct PermDiff {
    pub read: DiffOp,
    pub write: DiffOp,
    pub execute: DiffOp,
}

/// Enum for diff between two values.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiffOp {
    Plus,
    Same,
    Minus,
}

#[inline]
const fn bool_diff(a: bool, b: bool) -> DiffOp {
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
            Perm::new($r, $w, $x)
        };
    }

    macro_rules! mode {
        ($u:expr, $g:expr, $o:expr) => {
            Mode::new($u, $g, $o)
        };
        ($ur:expr, $uw:expr, $ux:expr, $gr:expr, $gw:expr, $gx:expr, $or:expr, $ow:expr, $ox: expr) => {
            Mode::new(
                perm!($ur, $uw, $ux),
                perm!($gr, $gw, $gx),
                perm!($or, $ow, $ox),
            )
        };
    }

    // TODO: Add failing tests
    #[test]
    fn test_perm_num() -> Result<(), Box<dyn std::error::Error>> {
        use perm::*;

        macro_rules! test_perm_num {
            ($c:expr, $p:expr) => {{
                assert_eq!($c, $p.as_num());
                assert_eq!($p, Perm::from_num($c)?)
            }};
            ($c:expr, $r:expr, $w:expr, $x:expr) => {
                test_perm_num!($c, perm!($r, $w, $x))
            };
        }

        test_perm_num!("7", RWX);
        test_perm_num!("6", RW);
        test_perm_num!("5", RX);
        test_perm_num!("4", R);
        test_perm_num!("3", WX);
        test_perm_num!("2", W);
        test_perm_num!("1", X);
        test_perm_num!("0", EMPTY);

        Ok(())
    }

    #[test]
    fn test_perm_sym() -> Result<(), Box<dyn std::error::Error>> {
        use perm::*;

        macro_rules! test_perm_sym {
            ($s:expr, $fs:expr, $p:expr) => {
                assert_eq!($s, $p.as_sym());
                assert_eq!($fs, $p.as_sym_full());
                assert_eq!($p, Perm::from_sym_full($fs)?)
            };
            ($s:expr, $fs:expr, $r:expr, $w:expr, $x:expr) => {
                test_perm_sym!($s, $fs, perm!($r, $w, $x))
            };
        }

        macro_rules! test_perm_sym_e {
            ($fs:expr, $err:expr) => {
                assert_eq!($err, Perm::from_sym_full($fs).unwrap_err())
            };
        }

        test_perm_sym!("rwx", "rwx", RWX);
        test_perm_sym!("rw", "rw-", RW);
        test_perm_sym!("rx", "r-x", RX);
        test_perm_sym!("r", "r--", R);
        test_perm_sym!("wx", "-wx", WX);
        test_perm_sym!("w", "-w-", W);
        test_perm_sym!("x", "--x", X);
        test_perm_sym!("", "---", EMPTY);

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

    #[test]
    fn test_perm_diff() {
        use DiffOp::*;

        macro_rules! test_perm_diff {
            ($rd:expr, $wd:expr, $xd:expr; $r1:expr, $w1:expr, $x1:expr; $r2:expr, $w2:expr, $x2:expr) => {
                assert_eq!(
                    PermDiff {
                        read: $rd,
                        write: $wd,
                        execute: $xd
                    },
                    perm!($r1, $w1, $x1).diff(&perm!($r2, $w2, $x2))
                )
            };
        }

        test_perm_diff!(Minus, Minus, Minus; true, true, true; false, false, false);
        test_perm_diff!(Plus, Same, Plus; false, true, false; true, true, true);
        test_perm_diff!(Minus, Plus, Same; true, false, false; false, true, false);
        test_perm_diff!(Same, Same, Same; false, false, true; false, false, true);
    }
}
