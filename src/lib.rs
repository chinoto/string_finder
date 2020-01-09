// Only test strings up to a max length of 13
// because any higher will overflow a u64: 13.6=64/log2(26).
pub const STRLEN: usize=13;

pub fn get_chars() -> (impl Iterator<Item=char>+Clone, u64) {
	let chars=(b'a'..=b'z').map(char::from);
	let charslen=chars.clone().count() as u64;
	(chars, charslen)
}
