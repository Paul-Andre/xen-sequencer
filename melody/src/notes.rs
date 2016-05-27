/// Defines a structure for Note.



pub struct Note {
    // Fields: 
    // `start`: the time representing the start
    pub start: f64,
    // `end`: the time representing the end
    pub end: f64,
    // `effect`: the name of the effect of the note
    pub effect: String,
    // `f`: the frequency
    pub f: f64,
    // `amp`: the amplitude
    pub amp: f64,
}

/// Methods
impl Note {
    // Contructs a `Note`.
    pub fn new() -> Note {
	Note {
             start: 0.0,
             end: 0.0,
             effect: "".to_string(),
             f: 0.0,
             amp: 0.0,
        }
    }
    
    // Sets the length of the `Note`.
    pub fn set_length(&mut self, l: f64) -> f64 {
        if l >= 0.0 {
            self.end = self.start + l
        } else {
            panic!("note starts after it ends!"); 
        }
            l
    }

    // Sets the `start` of the `Note`.
    pub fn set_start(&mut self, s: f64) -> f64 {
        if self.end >= s {
            self.start = s;
        } else {
            panic!("note starts after it ends!");
        }
        self.start
    }

    // Sets the `end` of the `Note`.
    pub fn set_end(&mut self, e: f64) -> f64 {
        if self.start <= e {
            self.end = e;
        } else {
            panic!("note starts after it ends!");
        }
        self.end
    }

    // Sets the `effect` of the `Note`
    pub fn set_effect(&mut self, s: &str) -> String {
        self.effect = s.to_string();
        self.effect.clone()
    }

    // Returns the length of the `Note`
    pub fn get_length(&self) -> f64 {
        self.end - self.start
    }

    // Assuming there can be more than one effect for a given Note,
    // Adds an effect to the `effect` String
    pub fn add_effect(&mut self, s: &str) -> String {
        self.effect = self.effect.clone() + ", " + s;
        self.effect.clone()
    }
} // End of impl Note

impl ToString for Note {
    fn to_string(&self) -> String {
        let output: String;

        output = "Time interval: (".to_string() + &self.start.to_string() +
                 ", " + &self.end.to_string() + 
                 "), " + "frequency: " + &self.f.to_string() +
                 ", amplitude: " + &self.amp.to_string() +
                 ", effect: \"" + &self.effect + "\".";
        output
    }
}

//--------------------- BUILDER --------------------------//
//--------------------------------------------------------//

pub struct NoteBuilder {
    start: f64,
    end: f64,
    effect: String,
    f: f64,
    amp: f64,
}

impl NoteBuilder {
    pub fn new() -> NoteBuilder {
        NoteBuilder { start: 0.0, end: 0.0, effect: "".to_string(), f: 0.0, amp: 0.0, }
    }

    // If s is after the end, end <- s
    pub fn start(&mut self, s: f64) -> &mut NoteBuilder {
        self.start = s;
        if self.end < self.start { self.end = self.start; };
        self
    }

    // If e is before the start, start <- e
    pub fn end(&mut self, e: f64) -> &mut NoteBuilder {
        self.end = e;
        if self.end < self.start { self.start = self.end; };
        self
    }

    // IMPORTANT: takes a slice as input
    pub fn effect(&mut self, name: &str) -> &mut NoteBuilder {
        self.effect = name.to_string();
        self
    }

    pub fn f(&mut self, freq: f64) -> &mut NoteBuilder {
        self.f = freq;
        self
    }

    pub fn amp(&mut self, a: f64) -> &mut NoteBuilder {
        self.amp = a;
        self
    }

    pub fn finalize(&mut self) -> Note {
        Note { start: self.start, end: self.end, 
               effect: self.effect.clone(), f: self.f, amp: self.amp, }
    }
} // End of impl NoteBuilder

//------------------------------------------------------------//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let n = NoteBuilder::new().start(6.7)
                                      .end(78.4)
                                      .effect("hello")
                                      .f(444.0)
                                      .amp(34.0)
                                      .finalize();
    }

    #[test]
    #[should_panic]
    fn test_basic_setter() {
        let mut n = NoteBuilder::new().finalize();

        println!("{:?}", n.set_length(46.0));
        println!("{:?}", n.set_start(36.0));
        println!("{:?}", n.set_start(76.0));
        println!("{:?}", n.set_end(45.0));
        println!("{:?}", n.set_end(2.0));
    }

    #[test]
    fn test_methods_effect() {
        let mut n = NoteBuilder::new().finalize();

        n.set_effect("hi");
        n.add_effect("hello");
        n.set_effect("bye");
    }

    #[test]
    fn test_tostring() {
        let mut n = NoteBuilder::new().start(4.3)
                                      .end(123.4)
                                      .effect("yomama")
                                      .f(231.5)
                                      .amp(23.4)
                                      .finalize();

        let expected = "Time interval: (4.3, 123.4), frequency: 231.5, amplitude: 23.4, effect: \"yomama\".";
        let holder = n.to_string();
        if holder != expected { panic!("Unexpected value!"); };
    }
}
