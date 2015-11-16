// Theme: Rust basics.

fn main() {
    let mut vec = Vec::new();       /*
            ~~~   ~~~~~~~~~~~        *
             |         |             *
             |     Creates and       *
             |     returns a vector  *
             |                       *
         Vector is owned             *
         by this stack frame         */

    vec.push(22u8);
    vec.push(44);
    vec.push(66);

    println!("Vector has length `{}` and contents `{:?}`", vec.len(), vec); /*
    ~~~~~~~~                    ~~~~              ~~~~~~   ~~~~~~~~~~~~~~    *
       |                         |                  |             |          *
    Macro that takes a          Placeholders -------+             |          *
    format string.                                            Arguments      */

    let string = format!("Vector has length `{}` and contents `{}`", vec.len(), vec);
    println!("{}", string);

} // <-- Here, `vec` goes out of scope, destructor will run and it will be freed.
