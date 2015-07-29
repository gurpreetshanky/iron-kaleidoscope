#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;

extern crate iron_kaleidoscope;

use iron_kaleidoscope::driver::{main_loop, Tokens, AST, IR, Exec};

//< parser-main
docopt!(Args, "
Usage: iron_kaleidoscope [(-l | -p | -i)]

Options:
    -l  Run only lexer and show its output.
    -p  Run only parser and show its output.
    -i  Run only IR builder and show its output.
");

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    let stage = if args.flag_l {
        Tokens
//> parser-main
    } else if args.flag_p {
        AST
    } else if args.flag_i {
        IR
//< parser-main
    } else {
//>parser-main
/*
//< parser-main
        AST
//> parser-main
*/
        IR
        //Exec
//< parser-main
    };
//> parser-main

    if stage == Exec {
        panic!("Not implemented");
    } else {
//< parser-main

    main_loop(stage);
//> parser-main
    }

//< parser-main
}
//> parser-main
