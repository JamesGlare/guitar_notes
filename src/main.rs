extern crate clap;
use clap::{App, Arg};
mod guitar_note;

fn main() {
    use guitar_note::guitar_note;
    let matches = App::new("GuitarNotes")
                                .version("1.0")
                                .author("Jannes G. <jannesgla@gmail.com>")
                                .about("Prints out scales to the shell and identifies chords on the fretboard.")
                                .arg(Arg::with_name("note")
                                    .short("n")
                                    .long("note")
                                    .multiple(true)
                                    .help("Prints out corresponding notes and identifies chords. Example: guitar_notes -n A3 D2 G0 B1 E0")
                                    .takes_value(true))
                                .arg(Arg::with_name("scale")
                                     .short("s")
                                     .long("scale")
                                     .help("Print out any scale for a given root. 
                                            Example: guitar_notes -s E {major, minor, minor_blues, major_blues, minor_pentatonic, major_pentatonic, dorian, lydian, mixolydin}")
                                     .number_of_values(2) )
                                .arg(Arg::with_name("all_notes")
                                    .short("a")
                                    .long("all")
                                    .help("Print all positions of the note given on the fretboard.")
                                    .takes_value(true))
                                .arg(Arg::with_name("tuning")
                                    .short("t")
                                    .long("tuning")
                                    .help("Set a guitar tuning different from EADGBE: {DROPD,...}")
                                    .global(true)
                                    .default_value("eadgbe"))
                                .get_matches();
    // See if the user set a tuning
    let opt_tuning =
        guitar_note::parse_tuning(&matches.value_of("tuning").unwrap().to_lowercase()[..]);
    let tuning = match opt_tuning {
        Some(tuning_) => tuning_,
        None => {
            println!(
                "WARNING: could not parse the tuning name. It should be one of (any case):
                 eadgbe,
                 dropd,
                 doubledropd,
                 dadgad,
                 dgcdcg,
                 openc6,
                 eeeebe,
                 opend,
                 opene,
                 openg,
                 opena,
                 openc"
            );
            println!("I will continue and assume standard tuning: eadgbe");
            guitar_note::parse_tuning("eadgbe").unwrap()
        }
    };
    // parse note arguments
    match matches.values_of("note") {
        Some(note_str) => {
            print!("Notes: ");

            let notes = note_str.map(|s| s.to_lowercase()).collect::<Vec<_>>();
            let note_results = guitar_note::from_tab_notation(&notes, &tuning);
            if let Some(results) = note_results {
                println!("{}", results);
                let fret_numbers = guitar_note::print_fret_numbers();
                let fret_markers = guitar_note::print_fret_markers();
                let (chord_strings, fretboard) =
                    guitar_note::chord_from_tab_notation(&notes, &tuning);

                if chord_strings.is_empty() || chord_strings.iter().all(|s| s.is_none()) {
                    println!("This is not a chord that I know.");
                } else {
                    for (idx, opt_chord_string) in chord_strings.iter().enumerate() {
                        if let Some(chord_string) = opt_chord_string {
                            if idx == 0 {
                                println!("Chord: {}", chord_string);
                            } else {
                                println!("{}. inversion: {}", idx, chord_string);
                            }
                        }
                    }
                    // print the notes on the fretboard
                    println!("{}", fret_numbers);
                    print!("\n");
                    println!("{}", fretboard);
                    print!("\n");
                    println!("{}", fret_markers);
                }
            } else {
                println!(
                    "I had trouble parsing some notes. Are they in tab notation (e.g. E0, A13)?"
                );
            }
        }
        None => {}
    }
    match matches.value_of("all_notes") {
        Some(note_name) => {
            let fret_numbers = guitar_note::print_fret_numbers();
            let fret_markers = guitar_note::print_fret_markers();
            let opt_fretboard =
                guitar_note::all_notes_on_fretboard(&note_name.to_lowercase()[..], &tuning);
            if let Some(fretboard) = opt_fretboard {
                println!("{}", fret_numbers);
                print!("\n");
                println!("{}", fretboard);
                print!("\n");
                println!("{}", fret_markers);
            } else {
                println!("Sorry, I could not parse the note you want me to find.");
            }
        }
        None => {}
    }
    match matches.values_of("scale") {
        Some(mut vals) => {
            let fret_numbers = guitar_note::print_fret_numbers();
            let fret_markers = guitar_note::print_fret_markers();
            let scale_name = &vals.nth(0).unwrap().to_lowercase()[..];
            let root_name = &vals.nth(0).unwrap().to_lowercase()[..];
            if let Some(result) = guitar_note::scale_on_fretboard(scale_name, root_name, &tuning) {
                println!("{}", result.2);
                println!("{}", result.1);
                print!("\n");
                println!("{}", fret_numbers);
                print!("\n");
                println!("{}", result.0);
                print!("\n");
                println!("{}", fret_markers);
            } else if let Some(result) =
                guitar_note::scale_on_fretboard(root_name, scale_name, &tuning)
            {
                println!("{}", result.2);
                println!("{}", result.1);
                print!("\n");
                println!("{}", fret_numbers);
                print!("\n");
                println!("{}", result.0);
                print!("\n");
                println!("{}", fret_markers);
            } else {
                println!("Sorry, I could not parse scale and/or root input.");
                println!(
                    "Scale should be one of: 
                minor,
                major,
                minor_blues,
                major_blues,
                minor_pentatonic,
                major_pentatonic,
                dorian,
                phrygian,
                lydian,
                mixolydian."
                )
            }
        }
        None => {}
    }
}
