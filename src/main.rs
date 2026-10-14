#![recursion_limit = "4096"]

#[macro_use]
mod macros;

fn main() {
    // Prints "Hello World!"
    brainfuck!(++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.);
}
