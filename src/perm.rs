//! [`Perm`] value constants.

use crate::Perm;

macro_rules! const_perm {
    ($name:ident, $nname:ident, $s:expr, $read:expr, $write:expr, $execute:expr) => {
        doc_comment::doc_comment! {
            concat!("Permission `", $s, "`, equivalent to",
                    " `Perm { read: ", stringify!($read), ",",
                            " write: ", stringify!($write), ",",
                            " execute: ", stringify!($execute),
                            " }",
                    "`."),
            pub const $name: Perm = Perm {
                read: $read,
                write: $write,
                execute: $execute,
            };
        }

        doc_comment::doc_comment! {
            concat!("See [`", stringify!($name), "`]."),
            pub const $nname: Perm = $name;
        }
    };
}

const_perm!(RWX, _7, "rwx", true, true, true);
const_perm!(RW, _6, "rw-", true, true, false);
const_perm!(RX, _5, "r-x", true, false, true);
const_perm!(R, _4, "r--", true, false, false);
const_perm!(WX, _3, "-wx", false, true, true);
const_perm!(W, _2, "-w-", false, true, false);
const_perm!(X, _1, "--x", false, false, true);
const_perm!(EMPTY, _0, "---", false, false, false);
