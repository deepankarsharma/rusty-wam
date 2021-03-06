#[macro_use] extern crate lazy_static;
#[macro_use] extern crate downcast;
extern crate termion;

mod prolog;
#[macro_use] mod test_utils;

use prolog::io::*;
use prolog::machine::*;

#[cfg(test)]
mod tests;

pub static LISTS: &str   = include_str!("./prolog/lib/lists.pl");
pub static CONTROL: &str = include_str!("./prolog/lib/control.pl");

fn process_buffer(wam: &mut Machine, buffer: &str)
{
    match parse_code(wam, buffer) {
        Ok(tl) => {
            let result = compile(wam, &tl);
            print(wam, result);
        },
        Err(s) => println!("{:?}", s)
    };
}

fn load_init_str(wam: &mut Machine, src_str: &str)
{
    match parse_batch(wam, src_str) {
        Ok(tls) => compile_batch(wam, &tls),
        Err(_)  => panic!("failed to parse batch from string.")
    };
}

fn prolog_repl() {
    let mut wam = Machine::new();

    load_init_str(&mut wam, LISTS);
    load_init_str(&mut wam, CONTROL);
    
    loop {
        print!("prolog> ");

        let buffer = read();

        if buffer == "quit\n" {
            break;
        } else if buffer == "clear\n" {
            wam.clear();
            continue;
        }

        process_buffer(&mut wam, buffer.as_str());
        wam.reset();
    }
}

fn main() {
    prolog_repl();
}
