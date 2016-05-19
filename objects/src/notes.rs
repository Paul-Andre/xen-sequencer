/// notes.rs
/// This is the module for `Notes`.

// Note that start/end fields represent times, and should thus have a different type than the
// currently specified one

/// # Note `Object`
struct Note {
	/// # Fields: 
	/// * `start`: the time representing the start
	start:     f64,
	/// * `end`: the time representing the end
	end:       f64,
	/// * `effect`: the name of the effect of the note
	effect: String,
	/// * `f`: the frequency
	f:         f64,
	/// * `amp`: the amplitude
	amp: f64,
}

/// # Methods:
impl Note {
	/// ## set_length(&mut self, f64) -> f64
	/// Sets the length of the `Note`.
	///
	/// * Input: `f64` representing the length
	/// * Output: `f64` representing the length
	/// ### Panics
	/// * if the new length is negative
	fn set_length(&mut self, l: f64) -> f64 {
		if l >= 0.0 {
			self.end = self.start + l
		} else {
			panic!("note starts after it ends!"); 
		}
		l
	}

	/// ## set_start(&mut self, f64) -> f64
	/// Sets the `start` of the `Note`.
	///
	/// * Input: `f64` representing the new `start`
	/// * Ouput: `f64` representing the new `start`
	/// ### Panics
	/// * if the new start is later than the end
	fn set_start(&mut self, s: f64) -> f64 {
		if self.end >= s {
			self.start = s;
		} else {
			panic!("note starts after it ends!");
		}
		self.start
	}

	/// ## set_end(&mut self, f64) -> f64
	/// Sets the `end` of the `Note`.
	///
	/// * Input: `f64` representing the new `end`
	/// * Ouput: `f64` representing the new `end`
	/// ### Panics
	/// * if the new end is later than the start
	fn set_end(&mut self, e: f64) -> f64 {
		if self.start <= e {
			self.end = e;
		} else {
			panic!("note starts after it ends!");
		}
		self.end
	}
}

#[test]
fn basic_settter_test() {

	let mut n = Note { start: 5.0, end: 16.0, f: 500.0, amp: 60.0 };


	println!("{:?}", n.set_length(46.0));
	println!("{:?}", n.set_start(36.0));
	println!("{:?}", n.set_start(76.0));
	println!("{:?}", n.set_end(45.0));
	println!("{:?}", n.set_end(2.0));
}

fn main() {
	let mut n = Note { start: 5.0, end: 16.0, f: 500.0, amp: 60.0 };
	println!("{}", n.set_length(46.0));
	println!("{}", n.set_start(36.0));
	//println!("{}", n.set_start(76.0));
	println!("{}", n.set_end(45.0));
	//println!("{}", n.set_end(2.0));
}
