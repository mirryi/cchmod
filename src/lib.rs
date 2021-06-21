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
        let w = if self.read { "w" } else { "" };
        let x = if self.read { "x" } else { "" };
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
