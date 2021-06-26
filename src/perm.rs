use crate::Perm;

macro_rules! const_perm {
    ($name:ident, $s:expr, $read:expr, $write:expr, $execute:expr) => {
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
    };
}

const_perm!(_7, "rwx", true, true, true);
const_perm!(_6, "rw-", true, true, false);
const_perm!(_5, "r-x", true, false, true);
const_perm!(_4, "r--", true, false, false);
const_perm!(_3, "-wx", false, true, true);
const_perm!(_2, "-w-", false, true, false);
const_perm!(_1, "--x", false, false, true);
const_perm!(_0, "---", false, false, false);
