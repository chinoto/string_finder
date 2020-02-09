use rayon::prelude::*;
// Heap allocations are expensive and reusing string buffers gets complicated with threads.
// Probably more efficient on CPU cache since the stack has to already be in there.
use arrayvec::{ArrayString, ArrayVec};
use string_finder::{get_chars, STRLEN};

fn main() {
    let (chars, charslen) = get_chars();

    println!(
        "{:?}",
        // All lengths that will be tested.
        (0..=STRLEN)
            // For each STRLEN, get all possible iterations of that length
            .flat_map(|strlen| {
                let mut counter = ArrayVec::<[usize; STRLEN]>::from([0; STRLEN]);
                //strlen is always <=STRLEN
                unsafe {
                    counter.set_len(strlen);
                }
                let max = if strlen > 0 { charslen as usize } else { 1 };
                (0..max).map(move |i| {
                    let mut counter = counter.clone();
                    if let Some(elem) = counter.first_mut() {
                        *elem = i;
                    }
                    counter
                })
            })
            .par_bridge()
            .flat_map(|counter| {
                let max = if counter.len() > 1 {
                    charslen.pow(counter.len() as u32 - 1)
                } else {
                    1
                };
                (0..max)
                    .scan(counter, |acc, _| {
                        let ret = Some(acc.clone());
                        for i in acc.iter_mut().skip(1) {
                            *i += 1;
                            assert!(*i <= charslen as usize);
                            if *i == charslen as usize {
                                *i = 0;
                            } else {
                                break;
                            }
                        }
                        ret
                    })
                    .par_bridge()
            })
            .map(|counter| {
                counter
                    .into_iter()
                    .fold(ArrayString::<[_; STRLEN]>::new(), |mut acc, i| {
                        acc.push(chars.clone().nth(i).unwrap());
                        acc
                    })
            })
            .find_any(|string| string.as_str() == "passw")
    );
}
