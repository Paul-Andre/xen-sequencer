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
} // End of impl Note

//--------------------- BUILDER --------------------------//
//--------------------------------------------------------//

struct NoteBuilder {
    start: f64,
    end: f64,
    effect: String,
    f: f64,
    amp: f64,
}

impl NoteBuilder {
    fn new() -> NoteBuilder {
        NoteBuilder { start: 0.0, end: 0.0, effect: "".to_string(), f: 0.0, amp: 0.0, }
    }

    fn start(&mut self, s: f64) -> &mut NoteBuilder {
        self.start = s;
        self
    }

    fn end(&mut self, e: f64) -> &mut NoteBuilder {
        self.end = e;
        self
    }

    fn effect(&mut self, name: &str) -> &mut NoteBuilder {
        self.effect = name.to_string();
        self
    }

    fn f(&mut self, freq: f64) -> &mut NoteBuilder {
        self.f = freq;
        self
    }

    fn amp(&mut self, a: f64) -> &mut NoteBuilder {
        self.amp = a;
        self
    }

    fn finalize(self) -> Note {
        Note { start: self.start, end: self.end, 
               effect: self.effect, f: self.f, amp: self.amp, }
    }
} // End of impl NoteBuilder

//------------------------------------------------------------//

#[test]
#[should_panic]
fn basic_setter_test() {
    let mut n = NoteBuilder::new().finalize();

    println!("{:?}", n.set_length(46.0));
    println!("{:?}", n.set_start(36.0));
    println!("{:?}", n.set_start(76.0));
    println!("{:?}", n.set_end(45.0));
    println!("{:?}", n.set_end(2.0));
}

fn main() {
    let mut n = NoteBuilder::new().finalize();
    println!("{}", n.set_length(46.0));
    println!("{}", n.set_start(36.0));
    //println!("{}", n.set_start(76.0));
    println!("{}", n.set_end(45.0));
    //println!("{}", n.set_end(2.0));
}
