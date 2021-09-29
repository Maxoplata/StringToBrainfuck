/**
 * StringToBrainfuck.rs
 *
 * Converts a string to a Brainfuck script that will output said string.
 * usage: cargo build && ./StringToBrainfuck your string here
 *
 * @author Maxamilian Demian
 * @link https://www.maxodev.org
 * @link https://github.com/Maxoplata/StringToBrainfuck
 */
use std::{env, iter, process};

fn main() {
	let mut args: Vec<String> = env::args().collect();

	// make sure we have arguments passed to the script
	if args.len() < 2 {
		process::exit(1);
	}

	// remove file from args
	args.remove(0);

	// get all of the chars for the input string
	let input_chars: Vec<char> = args.join(" ").chars().collect();

	// the Brainfuck code we will output in the end
	let mut bf_code = "".to_string();

	/*
	 * our current location on the "tape" (pointer 1).
	 * we use pointer 0 as a multiplier for pointer 1 to shorten the output script.
	 *
	 * e.g.
	 * A(65) = ++++++[>++++++++++<-]>+++++.
	 * ++++++      = add 6 to current pointer value (pointer 0)
	 * [           = while current pointer (pointer 0)'s value > 0
	 * >           = move pointer ahead one (to pointer 1)
	 * ++++++++++  = add 10 to current pointer value (pointer 1)
	 * <           = move pointer back one (to pointer 0)
	 * -           = subtract 1 from current pointervalue (pointer 0)
	 * ]           = end while loop
	 * >           = move pointer ahead one (to pointer 1)
	 * +++++       = add 5 to current pointer value (pointer 1)
	 * .           = print out character at current pointer value (pointer 1, value 65, char 'A')
	 *
	 * instead of:
	 * +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
	 */
	let mut current_location = 0;

	// iterate through each character in the string
	for i in 0..input_chars.len() {
		// get the Unicode code for the current character
		let char_val = (input_chars[i] as char) as i32;

		if char_val > current_location {
			// move ahead on the "tape" to build the character
			bf_code = bf_code + &iter::repeat("+").take(((char_val - current_location) / 10) as usize).collect::<String>();
			bf_code = bf_code + "[>++++++++++<-]>";
			bf_code = bf_code + &iter::repeat("+").take(((char_val - current_location) % 10) as usize).collect::<String>();
		} else if char_val < current_location {
			// move backwards on the "tape" to build the character
			bf_code = bf_code + &iter::repeat("+").take(((current_location - char_val) / 10) as usize).collect::<String>();
			bf_code = bf_code + "[>----------<-]>";
			bf_code = bf_code + &iter::repeat("-").take(((current_location - char_val) % 10) as usize).collect::<String>();
		} else {
			// delete the "<" from the previous command as we are on the same character
			// and we will want to print it out again
			bf_code = bf_code[..(bf_code.len() - 1)].to_string();
		}

		// print out the current character
		bf_code = bf_code + ".";

		// if we are not on the last letter of the string, move pointer position back to 0
		if i < (input_chars.len() - 1) {
			bf_code = bf_code + "<";
		}

		current_location = char_val;
	}

	println!("{}", bf_code);
}
