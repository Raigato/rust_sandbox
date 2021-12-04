mod arrays;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod pointer_refs;
mod print;
mod strings;
mod structs;
mod tuples;
mod types;
mod vars;
mod vectors;

fn main() {
    print::run();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer_refs::run();
    structs::run();
    enums::run();
}
