#[derive(Debug)]
pub struct Mode {
    pub user: Perm,
    pub group: Perm,
    pub other: Perm,
}

#[derive(Debug)]
pub struct Perm {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
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
            self.user.as_sym(),
            self.group.as_sym(),
            self.other.as_sym()
        )
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
    pub fn diff(&self, other: &Self) -> PermDiff {
        PermDiff {
            read: diff(self.read, other.read),
            write: diff(self.write, other.write),
            execute: diff(self.execute, other.execute),
        }
    }
}

#[derive(Debug)]
pub struct ModeDiff {
    pub user: PermDiff,
    pub group: PermDiff,
    pub other: PermDiff,
}

#[derive(Debug)]
pub struct PermDiff {
    pub read: DiffOp,
    pub write: DiffOp,
    pub execute: DiffOp,
}

#[derive(Debug)]
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
    fn test_perm_num() {
        macro_rules! perm_num {
            ($r:expr, $w:expr, $x:expr) => {
                perm!($r, $w, $x).as_num()
            };
        }

        macro_rules! test_perm_num {
            ($c:expr, $r: expr, $w:expr, $x:expr) => {
                assert_eq!($c, perm_num!($r, $w, $x))
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
    fn test_perm_sym() {
        macro_rules! perm_sym {
            ($r:expr, $w:expr, $x:expr) => {
                perm!($r, $w, $x).as_sym()
            };
        }

        macro_rules! test_perm_sym {
            ($c:expr, $r: expr, $w:expr, $x:expr) => {
                assert_eq!($c, perm_sym!($r, $w, $x))
            };
        }

        test_perm_sym!("rwx", true, true, true);
        test_perm_sym!("rw", true, true, false);
        test_perm_sym!("rx", true, false, true);
        test_perm_sym!("r", true, false, false);
        test_perm_sym!("wx", false, true, true);
        test_perm_sym!("w", false, true, false);
        test_perm_sym!("x", false, false, true);
    }

    #[test]
    fn test_mode_num() {
        macro_rules! mode_num {
            ($ur:expr, $uw:expr, $ux:expr, $gr:expr, $gw:expr, $gx:expr, $or:expr, $ow:expr, $ox: expr) => {
                mode!($ur, $uw, $ux, $gr, $gw, $gx, $or, $ow, $ox).as_num()
            };
        }

        macro_rules! test_mode_num {
            ($c:expr; $ur:expr, $uw:expr, $ux:expr; $gr:expr, $gw:expr, $gx:expr; $or:expr, $ow:expr, $ox: expr) => {
                assert_eq!($c, mode_num!($ur, $uw, $ux, $gr, $gw, $gx, $or, $ow, $ox))
            };
        }

        test_mode_num!(777; true, true, true; true, true, true; true, true, true);
        test_mode_num!(755; true, true, true; true, false, true; true, false, true);
        test_mode_num!(666; true, true, false; true, true, false; true, true, false);
        test_mode_num!(644; true, true, false; true, false, false; true, false, false);
        test_mode_num!(400; true, false, false; false, false, false; false, false, false);
    }
}
