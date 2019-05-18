use rtunerbis::notes::{find_c0, temperament_ratio, NOTES, EqualTemperamentScale};

/// Print the list of musical notes in the equal temperament
fn print_notes() {
    let mut c = find_c0(440_f64);

    for octave in 0..8 {
        for note in 0..12 {
            println!(
                "{:2}{} : {:9.3}",
                NOTES[note],
                octave,
                c * temperament_ratio(note)
            );
        }

        c = c * 2_f64;
    }
}

fn print_notes_better() {
    for (name, frequency) in EqualTemperamentScale::new() {
        println!("{}: {:9.4}", name, frequency);
    }
}

fn main() {
    print_notes_better();
}
