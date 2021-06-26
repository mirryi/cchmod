use crate::Perm;

macro_rules! const_perm {
    ($name:ident, $read:expr, $write:expr, $execute:expr) => {
        pub const $name: Perm = Perm {
            read: $read,
            write: $write,
            execute: $execute,
        };
    };
}

const_perm!(_7, true, true, true);
const_perm!(_6, true, true, false);
const_perm!(_5, true, false, true);
const_perm!(_4, true, false, false);
const_perm!(_3, false, true, true);
const_perm!(_2, false, true, false);
const_perm!(_1, false, false, true);
const_perm!(_0, false, false, false);
