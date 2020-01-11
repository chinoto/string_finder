use rayon::prelude::*;
// Heap allocations are expensive and reusing string buffers gets complicated with threads.
// Probably more efficient on CPU cache since the stack has to already be in there.
use arrayvec::ArrayString;
use strength_reduce::StrengthReducedU64;
use string_finder::{STRLEN, get_chars};

fn main() {
    let (chars, charslen)=get_chars();
    let charslen_div=StrengthReducedU64::new(charslen);

    println!("{:?}",
        // All lengths that will be tested.
        (0..=STRLEN)
            // For each STRLEN, get all possible iterations of that length
            .flat_map(|strlen| {
                chars
                    .clone()
                    .map(move |c| {
                        let mut string=ArrayString::<[_; STRLEN]>::new();
                        string.push(c);
                        (strlen-1, string)
                    })
            })
            .par_bridge()
            .flat_map(|(strlen, string)| {
                (0..charslen.pow(strlen as u32))
                    .into_par_iter()
                    .map(move |i| (strlen, i, string))
            })
            .map(|(strlen, mut i, mut string)| {
                // Divide the iteration index to get each character index
                for _ in 0..strlen {
                    let (quot, rem)=StrengthReducedU64::div_rem(i, charslen_div);
                    i=quot;
                    string.push(chars.clone().nth(rem as usize).unwrap());
                }
                string
            })
            .find_any(|string| string.as_str()=="passw")
    );
}
