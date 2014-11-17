// Theme: Borrowing.

fn main() {
    let vec = vec![22, 44, 66];

    let sum = sum(vec.as_slice());       /*
                  ~~~~          *
                    |           *
            Borrow the vector   */

    println!("The sum of `{}` is `{}`", vec, sum);
}

fn sum(v: &[int]) -> int {    /*
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

fn binary_search(haystack: &[int], needle: int) -> bool {
    false // FIXME
}

// Walthrough 1. Convert to use slices.

// Exercise 2. Write a binary search function.
