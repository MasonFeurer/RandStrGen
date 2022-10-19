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
const SEPARATORS: [char; 3] = ['-', '.', '_'];
const MISC_SYMBOLS: [char; 4] = ['!', '*', '&', '#'];

const DEFINED_SETS: [&'static [char]; 5] = [
    &DIGITS,
    &LETTERS_LC,
    &LETTERS_UC,
    &SEPARATORS,
    &MISC_SYMBOLS,
];

fn print_help() {
    println!(
        r#"
rand-str-gen [args] [len] [entries]

len: the number of characters in the generated string, must be a positive integer

args:
  --help    -h : display this help dialog
  --copy    -c : put the generated string in the OS clipboard
  --repeat  -r : set number of strings to generate, must be a positive integer

entries: [+|-][entry]
  +  adds entry to the character pool
  -  removes entry from the character pool
  
  Entries are a sequence of pre-defined and custom sets (not seperated by white-space or commas).
  
  Pre-defined sets:
     d : decimal digits, 0-9
     u : uppercase english alphabet, A-Z
     l : lowercase english alphabet, a-z
     s : separators, ['-', '.', '_']
     m : misc symbols, ['!', '*', '&', '#']
     A : alias for all sets (dulsm)
  
  Custom set: [characters]
    All UTF-8 characters between the '[' and the ']' are included in the set.
    If you want ']' in the set, too bad, because that denotes the end of the sequence and I don't feel like managing such case.
    
    If specifying a custom set, you might have to put the argument into quotes.
  
  By default, all pre-defined sets are added to the pool.
  
EXAMPLES:

{0}// Generate random string of length 10{1}
rand-str-gen 10

{0}// Without misc symbols{1}
rand-str-gen 10 -m

{0}// With custom set of characters: ['%', '$', '^', '@']{1}
rand-str-gen 10 -m "+[%$^@]"

{0}// With default sets, but without '.'{1}
rand-str-gen 10 "-[.]"

"#,
        Fg(LightBlack),
        style::Reset,
    )
}

const USE_HELP_MSG: &'static str = "use `--help` for valid args";

macro_rules! err {
    ($msg:expr,$help:expr) => {{
        println!("{}ERROR: {}{}", Fg(LightRed), $msg, style::Reset);
        println!("{}HELP: {}{}", Fg(LightBlue), $help, style::Reset);
        return;
    }};
}

fn gen_rand_string(pool: &[char], len: usize) -> String {
    if len == 0 || pool.is_empty() {
        return String::new();
    }

    let mut indices = vec![0; len];
    getrandom(&mut indices).unwrap();

    // indices are in range 0..256, they need to be mapped to 0..pool.len()

    let scale = (pool.len() - 1) as f32 / 255.0;

    indices
        .into_iter()
        .map(|idx| pool[(idx as f32 * scale).round() as usize])
        .collect()
}

fn main() {
    let mut args = std::env::args();
    let _bin_path = args.next().unwrap();

    let mut show_pool = false;
    let mut copy_cond = false;
    let mut len: Option<usize> = None;
    let mut repeat = 1;

    // --- PARSE ARGS ---
    while let Some(arg) = args.next().clone() {
        let mut matches = true;

        match arg.as_str() {
            "--copy" | "-c" => copy_cond = true,
            "--repeat" | "-r" => {
                let Some(arg) = args.next() else {
            		err!("expected arg: repeat count", USE_HELP_MSG);
            	};
                repeat = match arg.parse() {
                    Ok(count) => count,
                    Err(e) => err!(&format!("invalid count: {e}"), USE_HELP_MSG),
                };
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--show-pool" => show_pool = true,
            _ => matches = false,
        };
        if arg.starts_with("-") && !matches {
            err!(&format!("invalid arg: '{}'", arg), USE_HELP_MSG);
        }
        if matches {
            continue;
        }
        // arg doesn't start with -, so it should be the len arg

        len = match arg.parse() {
            Ok(len) => Some(len),
            Err(e) => err!(&format!("invalid length: {e}"), USE_HELP_MSG),
        };
        break;
    }

    let Some(len) = len else {
    	err!("expected arg: length of password", USE_HELP_MSG)	
    };

    let mut use_defined_sets = [true; DEFINED_SETS.len()];
    let mut add_chars = Vec::new();
    let mut remove_chars = Vec::new();

    // --- PARSE POOL MODIFIERS (ENTRIES) ---
    while let Some(arg) = args.next() {
        if arg.is_empty() {
            continue;
        }

        let mut chars = arg.chars();
        let prefix = chars.next().unwrap();

        let state = match prefix {
            '+' => true,
            '-' => false,
            e => err!(&format!("invalid entry prefix: '{e}'"), "expected + or -"),
        };

        while let Some(set) = chars.next() {
            match set {
                'd' => use_defined_sets[0] = state,
                'l' => use_defined_sets[1] = state,
                'u' => use_defined_sets[2] = state,
                's' => use_defined_sets[3] = state,
                'm' => use_defined_sets[4] = state,
                'A' => use_defined_sets = [false; DEFINED_SETS.len()],
                '[' => {
                    let mut set_chars = Vec::new();
                    while let Some(c) = chars.next() {
                        if c == ']' {
                            break;
                        }
                        set_chars.push(c);
                    }
                    if state {
                        add_chars.extend(set_chars);
                    } else {
                        remove_chars.extend(set_chars);
                    }
                }
                e => err!(
                    &format!("invalid pool entry: '{e}'"),
                    "can be one of: [d, l, u, s, m, [character, ...]]"
                ),
            };
        }
    }

    // --- CREATE POOL ---
    let mut pool = Vec::new();
    for i in 0..DEFINED_SETS.len() {
        if use_defined_sets[i] {
            pool.extend(DEFINED_SETS[i]);
        }
    }
    for c in add_chars {
        if pool.iter().position(|c2| *c2 == c).is_some() {
            err!(
                &format!("can't add character to pool, already exists: '{}'", c),
                &format!("characters in the set are: {pool:?}")
            )
        }
        pool.push(c);
    }
    for c in remove_chars {
        let Some(idx) = pool.iter().position(|c2| *c2 == c) else {
    		err!(
    			&format!("can't remove character from pool, doesn't exist: '{}'", c),
    			&format!("characters in the set are: {pool:?}")
    		)
    	};
        pool.remove(idx);
    }

    if show_pool {
        println!("pool: {pool:?}");
    }

    // --- CREATE STRING ---

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

// ---- UNIT TESTS ----

#[cfg(test)]
#[inline(always)]
fn gen_rand_char(pool: &[char]) -> char {
    // the unit test shouldn't give this an empty pool
    assert_eq!(pool.is_empty(), false);

    let mut index = [0];
    getrandom(&mut index).unwrap();

    let scale = (pool.len() - 1) as f32 / 255.0;
    pool[(index[0] as f32 * scale).round() as usize]
}

#[test]
fn test() {
    // --- SMALLER TESTS ---
    assert_eq!(gen_rand_string(&[], 5).as_str(), "");
    assert_eq!(gen_rand_string(&[], 0).as_str(), "");
    assert_eq!(gen_rand_string(&['a'], 0).as_str(), "");
    for _ in 0..100 {
        assert_eq!(gen_rand_string(&['a'], 1).as_str(), "a");
    }

    // --- BIGGER TESTS ---
    println!("\ntesting pool of size 1");

    let pool = ['a'];
    for _ in 0..10_000_000 {
        // this asserts that get_rand_char doesn't calculate an invalid index or char
        let c = gen_rand_char(&pool);
        assert_eq!(c, 'a');
    }
    println!("done\ntesting pool of size 2");

    let pool = ['a', 'b'];
    for _ in 0..10_000_000 {
        let c = gen_rand_char(&pool);
        assert!(c == 'a' || c == 'b');
    }

    println!("done\nchecking distrobution");

    // assert that the distrobutions are ~even
    let mut a_count: i32 = 0;
    let mut b_count: i32 = 0;
    for _ in 0..10_000_000 {
        let c = gen_rand_char(&pool);
        match c {
            'a' => a_count += 1,
            'b' => b_count += 1,
            _ => unreachable!(),
        }
    }
    let diff = (a_count - b_count).abs();

    println!(" - a_count: {a_count}");
    println!(" - b_count: {b_count}");
    println!(" - diff: {diff}");

    // for 10M coin flips, I'd say a difference of 10,000 (0.01%) between totals is reasonable
    assert!(diff < 10_000);

    println!("done");
}
