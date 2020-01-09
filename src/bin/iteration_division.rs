use rayon::prelude::*;
// Heap allocations are expensive and reusing string buffers gets complicated with threads.
// Probably more efficient on CPU cache since the stack has to already be in there.
use arrayvec::ArrayString;
use string_finder::{STRLEN,get_chars};

fn main() {
    let (chars, charslen)=get_chars();

    println!("{:?}",
        // All lengths that will be tested.
        (0..=STRLEN)
            .into_par_iter()
            // For each STRLEN, get all possible iterations of that length
            .flat_map(|strlen| {
                (0..charslen.pow(strlen as u32))
                    .into_par_iter()
                    .map(move |i| (strlen,i))
            })
            .map(|(strlen, mut i)| {
                let mut string=ArrayString::<[_; STRLEN]>::new();
                // Divide the iteration index to get each character index
                for _ in 0..strlen {
                    let (quot,rem)=div_rem(i, charslen);
                    i=quot;
                    string.push(chars.clone().nth(rem as usize).unwrap());
                }
                string
            })
            .find_any(|string| string.as_str()=="passw")
    );
}

// This will hopefully give the quotient and remainder in one instruction
fn div_rem(x: u64, y: u64) -> (u64, u64) {
    (x/y, x%y)
}
