
macro_rules! bf {
    // base case: no tokens left
    (@run $tape:ident $ptr:ident,) => {}; 
    
    // the 8 commands
    (@run $tape:ident $ptr:ident, + $($rest:tt)*) => {
        $tape[$ptr] = $tape[$ptr].wrapping_add(1);
        bf!(@run $tape $ptr, $($rest)*);
    };
    (@run $tape:ident $ptr:ident, - $($rest:tt)*) => {
        $tape[$ptr] = $tape[$ptr].wrapping_sub(1);
        bf!(@run $tape $ptr, $($rest)*);
    };
    (@run $tape:ident $ptr:ident, . $($rest:tt)*) => {
        print!("{}", $tape[$ptr] as char);
        ::std::io::Write::flush(&mut ::std::io::stdout()).unwrap();
        bf!(@run $tape $ptr, $($rest)*);
    };
    (@run $tape:ident $ptr:ident, , $($rest:tt)*) => {
        let mut buffer = String::new();
        ::std::io::Stdin::read_line(&mut ::std::io::stdin(), &mut buffer).unwrap();
        $tape[$ptr] = buffer.chars().nth(0).unwrap() as u8;
        bf!(@run $tape $ptr, $($rest)*);
        
    };
    (@run $tape:ident $ptr:ident, > $($rest:tt)*) => {
        $ptr = $ptr.wrapping_add(1);
        bf!(@run $tape $ptr, $($rest)*);
    };
    (@run $tape:ident $ptr:ident, < $($rest:tt)*) => {
        $ptr = $ptr.wrapping_sub(1);
        bf!(@run $tape $ptr, $($rest)*);
    };
    (@run $tape:ident $ptr:ident, [ $($body:tt)* ] $($rest:tt)*) => {
        while $tape[$ptr] != 0 {
            bf!(@run $tape $ptr, $($body)*);
        }
        bf!(@run $tape $ptr, $($rest)*);
    };
    // Rust's lexer glues some adjacent BF chars into one token
    // (`<-`, `->`, `<<`, `>>`, `..`, `...`). Split them back apart by
    // re-emitting the pieces spaced out, then re-run.
    (@run $tape:ident $ptr:ident, <- $($rest:tt)*) => {
        bf!(@run $tape $ptr, < - $($rest)*);
    };
    (@run $tape:ident $ptr:ident, -> $($rest:tt)*) => {
        bf!(@run $tape $ptr, - > $($rest)*);
    };
    (@run $tape:ident $ptr:ident, << $($rest:tt)*) => {
        bf!(@run $tape $ptr, < < $($rest)*);
    };
    (@run $tape:ident $ptr:ident, >> $($rest:tt)*) => {
        bf!(@run $tape $ptr, > > $($rest)*);
    };
    (@run $tape:ident $ptr:ident, ... $($rest:tt)*) => {
        bf!(@run $tape $ptr, . . . $($rest)*);
    };
    (@run $tape:ident $ptr:ident, .. $($rest:tt)*) => {
        bf!(@run $tape $ptr, . . $($rest)*);
    };
}


macro_rules! brainfuck {
    ($($code:tt)*) => {
        let mut tape = [0u8; 30_000];
        let mut ptr: usize = 0;
        bf!(@run tape ptr, $($code)*);
    };
}