// Theme: Borrowing.

#![feature(slicing_syntax)]         /*
~~ ~~~~~~~                           *
|    |                               *
|  Acknowledge use of incomplete     *
|  feature.                          *
|                                    *
Attach attribute to enclosing item.  *
In this case, the module.            */

fn main() {
    let vec = vec![22, 44, 66, 88];

    let sum = sum(&vec);       /*
                  ~~~~          *
                    |           *
            Borrow the vector   */

    println!("The sum of `{}` is `{}`", vec, sum);
    println!("{}", binary_search(vec[], 22));
    println!("{}", binary_search(vec[], 42));
    println!("{}", binary_search_rec(vec[], 22));
    println!("{}", binary_search_rec(vec[], 42));
}

fn sum(v: &Vec<int>) -> int {    /*
          ~~~~~~~~~               *
              |                   *
       Request a borrowed vector  */

    let (mut i, c, mut sum) = (0, v.len(), 0);

    while i < c {
        sum += v[i];
        i += 1;
    }

    sum
}

fn binary_search(mut haystack: &[int], needle: int) -> bool {
    while haystack.len() > 0 {
        let i = haystack.len() / 2;

        if haystack[i] == needle {
            return true;
        }

        haystack = if haystack[i] < needle {
            haystack[i + 1..]
        } else {
            haystack[..i]
        };
    }

    false
}

fn binary_search_rec(haystack: &[int], needle: int) -> bool {
    use std::cmp::Ord;
    if haystack.len() == 0 {
        false
    } else {
        let i = haystack.len() / 2;
        match Ord::cmp(&needle, &haystack[i]) {
            Equal => true,
            Greater => binary_search_rec(haystack[i + 1..], needle),
            Less => binary_search_rec(haystack[..i], needle)
        }
    }
}

// Walthrough 1. Convert to use slices.

// Exercise 2. Write a binary search function.
