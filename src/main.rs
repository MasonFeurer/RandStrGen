#![feature(let_chains)]

use cli_clipboard::{ClipboardContext, ClipboardProvider};
use getrandom::getrandom;
use termion::{color::*, style};

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const LETTERS_LC: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const LETTERS_UC: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const SYMBOLS: [char; 7] = ['-', '.', '_', '!', '*', '&', '#'];

const HELP: &'static str = r"
rand-str-gen [args] [len] [entries]

len: the number of characters in the generated string, must be a positive integer

arg:
  --help   : display this help dialog
  --copy   : put the generated string in the OS clipboard
  --repeat : set number of strings to generate, must be a positive integer

entry: [+|-][set-name]
  +  adds a pre-defined set to the character pool
  -  removes a pre-defined set to the character pool
  
  set-name:
     digits|d      : decimal digits, 0-9
     letters_uc|uc : uppercase english alphabet, A-Z
     letters_lc|lc : lowercase english alphabet, a-z
     symbols|s     : ['-', '.'. '_', '!', '*', '&', '#']
  
  by default, all sets are enabled.
";

const USE_HELP_MSG: &'static str = "use `--help` for valid args";

macro_rules! err {
    ($msg:expr,$help:expr) => {{
        println!("{}ERROR: {}{}", Fg(LightRed), $msg, style::Reset);
        println!("{}HELP: {}{}", Fg(LightBlue), $help, style::Reset);
        return;
    }};
}

fn gen_rand_string(pool: &[char], len: usize) -> String {
    let mut indices = vec![0; len];
    getrandom(&mut indices).unwrap();

    // indices are in range 0..256, they need to be mapped to 0..pool.len()

    let scale = (pool.len() - 1) as f32 / 255.0;

    indices
        .into_iter()
        .map(|idx| pool[(idx as f32 * scale) as usize])
        .collect()
}

fn main() {
    let mut args = std::env::args();
    let _bin_path = args.next().unwrap();

    let mut copy_cond = false;
    let mut len: Option<usize> = None;
    let mut repeat = 1;

    while let Some(arg) = args.next().clone() {
        if arg.starts_with("--") {
            let arg = &arg[2..];

            match arg {
                "copy" => copy_cond = true,
                "repeat" => {
                    let Some(arg) = args.next() else {
                		err!("expected arg: repeat count", USE_HELP_MSG);
                	};
                    repeat = match arg.parse() {
                        Ok(count) => count,
                        Err(e) => err!(&format!("invalid count: {e}"), USE_HELP_MSG),
                    };
                }
                "help" => {
                    println!("{}", HELP);
                    return;
                }
                e => err!(&format!("invalid arg: '{}'", e), USE_HELP_MSG),
            }
            continue;
        }
        // arg doesn't start with --, so it should be the len arg

        len = match arg.parse() {
            Ok(len) => Some(len),
            Err(e) => err!(&format!("invalid length: {e}"), USE_HELP_MSG),
        };
        break;
    }

    let Some(len) = len else {
    	err!("expected arg: length of password", USE_HELP_MSG)	
    };

    let mut pool_entries = [true, true, true, true];

    while let Some(arg) = args.next() {
        let mut chars = arg.chars();
        let prefix = chars.next().expect("expected + or -");

        let entry_state = match prefix {
            '+' => true,
            '-' => false,
            e => err!(&format!("invalid entry prefix: '{e}'"), "expected + or -"),
        };

        let entry: String = chars.collect();
        let entry_idx = match entry.as_str() {
            "digits" | "d" => 0,
            "letters_lc" | "lc" => 1,
            "letters_uc" | "uc" => 2,
            "symbols" | "s" => 3,
            e => err!(
                &format!("invalid pool entry: '{e}'"),
                "can be one of: [digits, letters_lc, letters_uc, symbols]"
            ),
        };

        pool_entries[entry_idx] = entry_state;
    }

    let mut pool = Vec::new();
    if pool_entries[0] {
        pool.extend(DIGITS);
    }
    if pool_entries[1] {
        pool.extend(LETTERS_LC);
    }
    if pool_entries[2] {
        pool.extend(LETTERS_UC);
    }
    if pool_entries[3] {
        pool.extend(SYMBOLS);
    }

    let strings: Vec<_> = (0..repeat).map(|_| gen_rand_string(&pool, len)).collect();

    for string in &strings {
        println!("{}", string);
    }

    if let Some(string) = strings.last() && copy_cond {
        let mut cb = ClipboardContext::new().expect("failed to create OS clipboard context");
        cb.set_contents(string.clone())
            .expect("failed to set OS clipboard contents");
    }
}
