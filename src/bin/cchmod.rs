use cchmod::{
    traits::{AsNum, AsSym},
    Mode, Perm,
};
use clap::Clap;

#[derive(Clap)]
pub struct Opts {
    input: String,

    #[clap(short, long, about = "Output the octal form")]
    num: bool,
    #[clap(short, long, about = "Output the symbolic form")]
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

#[derive(Debug)]
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
}
