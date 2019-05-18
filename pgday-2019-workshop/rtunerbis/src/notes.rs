pub const NOTES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

/// Calculate the ratio between a certain note and the start of the octave
pub fn temperament_ratio(note_number: usize) -> f64 {
    let mut start = 1f64;

    for _ in 0..note_number {
        start = start * 2f64.powf(1f64 / 12f64)
    }

    start
}

/// Given the diapason frequency (the frequency of A4), find the frequency
/// of the note C0, defined as the first C whose frequency is lower than 20 hz
pub fn find_c0(diapason: f64) -> f64 {
    let mut back_to_c = diapason / temperament_ratio(9);
    while back_to_c > 20f64 {
        back_to_c = back_to_c / 2_f64;
    }
    back_to_c
}

/// Calculate the difference in cents between two frequencies
pub fn diff_cents(a: f64, b: f64) -> f64 {
    1200f64 * (a / b).abs().log2()
}

pub struct EqualTemperamentScale {
    current_octave: usize,
    current_note: usize,
    current_frequency: f64,
    reference_c: f64,
}

impl EqualTemperamentScale {
    pub fn new() -> EqualTemperamentScale {
        EqualTemperamentScale {
            current_octave: 0,
            current_note: 0,
            current_frequency: find_c0(440_f64),
            reference_c: find_c0(440_f64)
        }
    }

    fn next_note(&mut self) {
        self.current_note = self.current_note + 1;
        if self.current_note >= 12 {
            self.current_note = 0;
            self.current_octave = self.current_octave + 1;
            self.reference_c = self.reference_c * 2_f64;
        }

        self.current_frequency = self.reference_c * temperament_ratio(self.current_note);
    }
}

impl Iterator for EqualTemperamentScale {
    type Item = (String, f64);

    fn next(&mut self) -> Option<(String, f64)> {
        if self.current_octave >= 8 {
            None
        } else {
            let result = (
                format!("{}{}", NOTES[self.current_note], self.current_octave), 
                self.current_frequency
            );
            self.next_note();
            Some(result)
        }
    }
}