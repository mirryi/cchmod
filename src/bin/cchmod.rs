use cchmod::{
    traits::{AsNum, AsSym},
    Mode, Perm,
};
use clap::{crate_authors, crate_description, crate_name, crate_version, Parser};

#[derive(Parser)]
#[clap(name = crate_name!(), version = crate_version!(), author = crate_authors!(), about = crate_description!())]
pub struct Opts {
    input: String,

    #[clap(short, long, help = "Output the octal form")]
    num: bool,
    #[clap(short, long, help = "Output the symbolic form")]
    sym: bool,
}

fn main() {
    if let Err(message) = cli() {
        eprintln!("{}", message);
        std::process::exit(1);
    }
}

fn cli() -> Result<(), String> {
    let Opts { input, num, sym } = Opts::parse();

    let num = output_as_num(num, sym)?;
    let output = match try_parse(&input) {
        Some(Parsed::Mode(mode)) => convert(&mode, num),
        Some(Parsed::Perm(perm)) => convert(&perm, num),
        None => return Err(format!("{}: malformed permission or mode", input)),
    };

    println!("{}", output);

    Ok(())
}

fn output_as_num(num: bool, sym: bool) -> Result<bool, String> {
    if num && sym {
        return Err("--num and --sym are exclusive".to_string());
    } else if !(num || sym) {
        return Err("--num or --sym must be supplied".to_string());
    }

    Ok(num)
}

fn convert<T: AsNum + AsSym>(v: &T, as_num: bool) -> String {
    if as_num {
        v.as_num()
    } else {
        v.as_sym()
    }
}

#[derive(Debug, PartialEq)]
enum Parsed {
    Mode(Mode),
    Perm(Perm),
}

fn try_parse(input: &str) -> Option<Parsed> {
    if let Ok(mode) = Mode::from_num(input).or_else(|_| Mode::from_sym(input)) {
        Some(Parsed::Mode(mode))
    } else if let Ok(perm) = Perm::from_num(input).or_else(|_| Perm::from_sym_full(input)) {
        Some(Parsed::Perm(perm))
    } else {
        None
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_output_as_num() {
        macro_rules! test {
            ($c:expr, $n:expr, $s:expr) => {
                assert_eq!($c, super::output_as_num($n, $s))
            };
        }

        test!(Ok(true), true, false);
        test!(Ok(false), false, true);

        test!(Err("--num and --sym are exclusive".to_string()), true, true);
        test!(
            Err("--num or --sym must be supplied".to_string()),
            false,
            false
        );
    }

    #[test]
    fn test_try_parse() {
        use super::Parsed::*;
        use cchmod::{Mode, Perm};

        macro_rules! test {
            ($c:expr, $input:expr) => {
                assert_eq!(Some($c), super::try_parse($input))
            };
        }

        macro_rules! test_fail {
            ($input:expr) => {
                assert_eq!(None, super::try_parse($input))
            };
        }

        test!(
            Mode(Mode {
                user: Perm {
                    read: true,
                    write: true,
                    execute: true
                },
                group: Perm {
                    read: true,
                    write: true,
                    execute: true
                },
                other: Perm {
                    read: true,
                    write: true,
                    execute: true
                }
            }),
            "rwxrwxrwx"
        );
        test!(
            Mode(Mode {
                user: Perm {
                    read: true,
                    write: true,
                    execute: true
                },
                group: Perm {
                    read: true,
                    write: true,
                    execute: true
                },
                other: Perm {
                    read: true,
                    write: true,
                    execute: true
                }
            }),
            "777"
        );
        test!(
            Perm(Perm {
                read: true,
                write: true,
                execute: true
            }),
            "rwx"
        );
        test!(
            Perm(Perm {
                read: true,
                write: true,
                execute: true
            }),
            "7"
        );

        test_fail!("");
        test_fail!("rx");
        test_fail!("rwxx");
        test_fail!("rwxrwx");
        test_fail!("8");
        test_fail!("77");
        test_fail!("585");
        test_fail!("4444");
    }
}
