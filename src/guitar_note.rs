mod chord;
mod note;
mod scale;
mod tuning;

pub mod guitar_note {
    use super::chord::Chord;
    use super::note::Note;
    use super::scale::Scale;
    use super::scale::ScaleType;
    use super::tuning::Tuning;

    fn parse_tab_notation(tab_note: &Vec<String>, tuning: &Tuning) -> Vec<Note> {
        let (note_strs, frets): (Vec<_>, Vec<_>) = tab_note
            .iter()
            .map(|s| split_string_fret(s))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .unzip();
        let offsets = frets
            .iter()
            .map(|s| Note { semitones: *s })
            .collect::<Vec<_>>();
        let notes = tuning
            .tune(&note_strs, &offsets)
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        return notes;
    }
    fn split_string_fret<'a>(tab_note: &'a str) -> Option<(&'a str, i32)> {
        // take the longest match in order to prefer match a# over a
        let matched_note_str = Note::NAMES
            .iter()
            .map(|n| tab_note.matches(n))
            .flatten()
            .max_by(|m1, m2| m1.len().cmp(&m2.len()))?;
        let fret = tab_note
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<i32>();
        match fret {
            Ok(number) => {
                return Some((matched_note_str, number));
            }
            _ => {
                return None;
            }
        }
    }
    #[test]
    fn test_split_string_fret() {
        let test = "a#33";
        if let Some((n, f)) = split_string_fret(test) {
            assert_eq!(n, "a#");
            assert_eq!(f, 33);
        } else {
            panic!("This should be some");
        }
        let test = "c0";
        if let Some((n, f)) = split_string_fret(test) {
            assert_eq!(n, "c");
            assert_eq!(f, 0);
        } else {
            panic!("This should be some");
        }
        let test = "h3";
        let res = split_string_fret(test);
        assert_eq!(res, None);
    }

    pub fn from_tab_notation(note_str: &Vec<String>, tuning: &Tuning) -> Option<String> {
        /* tab_note string needs to be of format
         * <string><fret>, e.g. E3 or A10.
         */
        let opt_notes = parse_tab_notation(note_str, tuning);
        return match opt_notes.len() > 0 {
            true => Some(
                opt_notes
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
                    .to_uppercase(),
            ),
            false => None,
        };
    }

    pub fn parse_tuning(tuning_name: &str) -> Option<Tuning> {
        return Tuning::from_name(tuning_name);
    }

    pub fn chord_from_tab_notation(
        note_str: &Vec<String>,
        tuning: &Tuning,
        relative: bool,
    ) -> (Vec<Option<String>>, String) {
        let notes = parse_tab_notation(note_str, tuning);
        let chords = Chord::find_chord(&notes);
        let inversions = chords
            .iter()
            .map(|opt_c| {
                if let Some(c) = opt_c {
                    Some(c.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<Option<String>>>();

        let first_chord_match = chords.into_iter().find_map(|c| c);
        let fretboard = match first_chord_match {
            Some(chord) => join_strings(&mut layout_on_fretboard(
                &notes,
                &tuning,
                chord.get_notes().first().unwrap(),
                relative,
            )),
            None => String::from(""),
        };
        return (inversions, fretboard);
    }
    pub fn scale_on_fretboard(
        scale_name: &str,
        root: &str,
        tuning: &Tuning,
        relative: bool,
    ) -> Option<(String, String, String)> {
        // Interface
        let opt_root = Note::from_string(root);
        let parse_result = scale_name.parse::<ScaleType>();
        match (opt_root, parse_result) {
            (Some(root), Ok(scale_type)) => {
                let scale = Scale::from_type_and_root(root, scale_type);
                let mut strings = layout_on_fretboard(
                    scale.get_notes(),
                    &tuning,
                    scale.get_notes().first().unwrap(),
                    relative,
                );
                let fretboard = join_strings(&mut strings);

                let degrees = scale.degrees_in_scale().collect::<Vec<_>>().join("\t");
                let chords = scale.chords_in_scale().join("\t");
                let notes = scale.notes_in_scale().collect::<Vec<_>>().join("\t");
                return match scale.scale_type {
                    ScaleType::major_blues
                    | ScaleType::minor_blues
                    | ScaleType::major_pentatonic
                    | ScaleType::minor_pentatonic => Some((fretboard, degrees, notes)),
                    _ => Some((fretboard, degrees, chords)),
                };
            }
            _ => {
                return None;
            }
        }
    }
    fn pad_to_length(cifar: &String) -> String {
        if cifar.len() < 2 {
            return format!("{} ", cifar);
        } else {
            return format!("{}", cifar);
        }
    }
    pub fn all_notes_on_fretboard(note_names: &Vec<String>, tuning: &Tuning) -> Option<String> {
        let notes = note_names
            .iter()
            .filter_map(|note_str| Note::from_string(note_str))
            .collect::<Vec<Note>>();
        return Some(join_strings(&mut layout_on_fretboard(
            &notes, &tuning, &notes[0], false,
        )));
    }
    pub fn print_fret_numbers() -> String {
        let fret_numbers = (1..24)
            .map(|x| pad_to_length(&x.to_string()))
            .collect::<Vec<_>>()
            .join("  ");
        return "      ".to_owned() + &fret_numbers;
    }
    pub fn print_fret_markers() -> String {
        let fret_markers = vec![
            "   ", "  ", "  ", "* ", "  ", "* ", "  ", "* ", "  ", "* ", "  ", "  ", ": ",
            /*  0    1   2    3     4    5    6    7   8    9   10    11   12 */
            "  ", "  ", "* ", "  ", "* ", "  ", "* ", "  ", "* ", "  ", "* ",
            "  ", /*  13  14   15    16     17    18    19    20   21    22   23   24*/
        ]
        .join("  ");
        return " ".to_owned() + &fret_markers;
    }
    fn layout_on_fretboard(
        notes: &Vec<Note>,
        tuning: &Tuning,
        root: &Note,
        relative: bool,
    ) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let root_str = root.to_string();
        let capitalize_if_root = |s: &str| {
            if s == root_str {
                s.to_uppercase()
            } else {
                s.to_owned()
            }
        };
        let print_note = |base_note: &Note, note: &Note, relative: bool| {
            if relative {
                return Scale::note_to_degree(root, &(*base_note + *note)).to_owned();
            } else {
                return capitalize_if_root((*base_note + *note).to_string());
            }
        };
        for base_note in tuning.get_basenotes().iter() {
            // for each string on guitar
            let frets = locate_on_string(notes, base_note);
            let mut nut_string = String::from("");

            if let Some(note) = frets[0] {
                let note_str = print_note(base_note, &note, relative);
                nut_string = format!(" {}|", pad_to_length(&note_str));
            } else {
                nut_string += "   |";
            }
            let mut fret_strings = vec![nut_string];

            for f in frets.iter().skip(1) {
                if let Some(note) = f {
                    let mut note_string = print_note(base_note, note, relative);
                    note_string = pad_to_length(&note_string);
                    fret_strings.push(note_string);
                } else {
                    fret_strings.push(String::from("- "));
                }
            }
            result.push(fret_strings.join("  "));
        }
        return result;
    }

    fn locate_on_string(notes: &Vec<Note>, base_note: &Note) -> [Option<Note>; 24] {
        let mut frets: [Option<Note>; 24] = [None; 24];
        for note in notes.iter() {
            let mut fret1 = ((note.no_octaves()) - *base_note).no_octaves();
            if fret1.semitones < 0 {
                // rust modulo works differently from python modulo
                fret1 = fret1 + Note::octave();
            }
            let fret2 = fret1 + Note::octave();

            frets[fret1.semitones as usize] = Some(fret1);
            frets[fret2.semitones as usize] = Some(fret2);
        }
        return frets;
    }

    fn join_strings(fret_strings: &mut Vec<String>) -> String {
        fret_strings.reverse();
        return fret_strings.join("\n");
    }

    #[test]
    fn test_modulo() {
        // How modulo works in rust
        let d = 5;
        let p = -12;
        let r1 = p % d;
        let r2 = d % p;
        println!("{} {}", r1, r2);
        assert_eq!(r1, -2);
        assert_eq!(r2, 5);
    }

    #[test]
    fn test_scale_print() {
        let scale_name = "major_blues";
        let root = "a";
        let tuning = Tuning::from_name("eadgbe").unwrap();
        let opt_result = scale_on_fretboard(scale_name, root, &tuning, false);
        if let Some(result) = opt_result {
            println!("{}", result.2);
            println!("{}", result.1);
            println!("{}", result.0);
        } else {
            panic!("Something went wrong");
        }
        let opt_result = scale_on_fretboard(scale_name, root, &tuning, true);
        if let Some(result) = opt_result {
            println!("{}", result.2);
            println!("{}", result.1);
            println!("{}", result.0);
        } else {
            panic!("Something went wrong");
        }
    }
} //
