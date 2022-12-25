// used to inspect output of rust tokenizer
macro_rules! print_tokens {
    ( $( $token:tt )* ) => {
        $(
            println!("{}", stringify!($token));
        )*
    };
}
