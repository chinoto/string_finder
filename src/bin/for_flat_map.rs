use rayon::prelude::*;
// Heap allocations are expensive and reusing string buffers gets complicated with threads.
// Probably more efficient on CPU cache since the stack has to already be in there.
use arrayvec::ArrayString;
use string_finder::{STRLEN, get_chars};

fn main() {
	let (chars, _)=get_chars();

	println!("{:?}",
		// All lengths that will be tested.
		(0..=STRLEN)
			// For each STRLEN, get all possible iterations of that length
			.flat_map(|strlen| -> Box<dyn Send+Iterator<Item=(usize, ArrayString<[_; 20]>)>> {
				//Zero length string is handled specially
				if strlen==0 {return Box::new(std::iter::once((0, ArrayString::new())));}

				// Iterator of each letter in a string
				Box::new(chars.clone().map(move |c| {
					let mut string=ArrayString::new();
					string.push(c);
					(strlen-1, string)
				}))
			})
			.par_bridge()
			.flat_map(|(strlen, string)| {
				// Type must be specified, otherwise the compiler looks for a struct instead of trait.
				let mut iter: Box<dyn Send+Iterator<Item=ArrayString<_>>>
					=Box::new(std::iter::once(string));

				for _ in 0..strlen {
					iter=Box::new(iter.flat_map(|string| {
						chars.clone().map(move |c| {
							let mut string=string.clone();
							string.push(c);
							string
						})
					}));
				}

				iter.par_bridge()
			})
			.find_any(|string| string.as_str()=="passw")
	);
}