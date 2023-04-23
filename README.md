# Macrofrick

Expand brainfuck into rust using Macros!

Read more on my blog: [rafibayer.github.io/2023/02/26/macrofrick](https://rafibayer.github.io/2023/02/26/macrofrick.html)

# How
Macrofrick defines a macro, `frick!`, which accepts 0 or more TokenTrees of valid Brainfuck code. The macro performs some initial setup to initialize the memory and pointer that represent the brainfuck cell tape and head, before passing each input TokenTree to a second macro `instr!` which expands brainfuck input into the equivalent Rust code.

Here is a version of the `frick!` macro that has been modified for clarity.

```rust
macro_rules! frick {
    // accept 0 or more tt
    ( $( $code:tt )* ) => {
        {
            // tape setup
            let mut mem = [0_u8; 30_000];
            let mut ptr = 0;

            // expand instructions
            $(
                // rust macros are hygienic,
                // so we pass in mem and ptr to instr! 
                instr!(mem ptr $code);
            )*

             // return final state of mem / ptr
            (mem, ptr)
        }
    };
}
```

And `instr!`...

```rust
// $mem and $ptr are passed to bring mem and ptr from frick! into scope
macro_rules! instr {
    ($mem:ident $ptr:ident >) => {
        $ptr += 1;
    };
    ($mem:ident $ptr:ident <) => {
        $ptr -= 1;
    };
    ($mem:ident $ptr:ident +) => {
        $mem[$ptr] += 1;
    };
    ($mem:ident $ptr:ident -) => {
        $mem[$ptr] -= 1;
    };
    ($mem:ident $ptr:ident .) => {
        print!("{}", $mem[$ptr] as char);
    };
    ($mem:ident $ptr:ident ,) => {
        $mem[$ptr] = std::io::stdin().bytes().next().unwrap().unwrap();
    };
    ($mem:ident $ptr:ident [ $( $body:tt )* ]) => {
        while $mem[$ptr] > 0 {
            $(
                instr!($mem $ptr $body);
            )*
        }
    };
}
```

The way rust tokenizes and parses forces us to define a few additional patterns which can be found in the full code, but they are all just combinations or repetitions of the ideas above. For example, rust considers `>>` to be a single token, so we can add a rule to expand this to either 2 `$ptr += 1;` statements, or just expand to a single `$ptr += 2;`. We choose the later as a minor optimization. 

# Why?
To learn Rust macros, and to go very fast! By expanding our brainfuck to Rust, we can leverage the full power of the Rust compiler to do a lot of the work at compile-time. This does make our compilation process quite lengthy, but the resulting binary executes very fast compared to many brainfuck compilers I've seen.

check out the `unsafe` branch for a version that eschews bounds checking via unsafe rust to both compile and run faster. 
