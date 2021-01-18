mod note;
mod chord;
mod tuning;
mod scale;

pub mod guitar_note {
    use super::note::Note;
    use super::tuning::Tuning;
    use super::chord::Chord;
    use super::scale::Scale;
    use super::scale::ScaleType;
    

    fn parse_tab_notation(tab_note: &Vec<String>) -> Vec<Note> {
        let (note_strs, frets): (Vec<_>, Vec<_>) = tab_note
            .iter()
            .map(|s| split_string_fret(s))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .unzip();
        let tuning = Tuning::eadgbe();
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

    pub fn from_tab_notation(note_str: &Vec<String>) -> Option<String> {
        /* tab_note string needs to be of format
         * <string><fret>, e.g. E3 or A10.
         */
        let opt_notes = parse_tab_notation(note_str);
        return match opt_notes.len() {
            _ => Some(
                opt_notes
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
                    .to_uppercase(),
            ),
            0 => None,
        };
    }

    pub fn chord_from_tab_notation(note_str: &Vec<String>) -> Vec<String> {
        let notes = parse_tab_notation(note_str);
        let chords = Chord::find_chord(&notes);
        return chords.iter().map(|c| c.to_string()).collect::<Vec<String>>();
    }
    
    
    pub fn scale_on_fretboard(scale_name: &str, root: &str) -> Option<(String, String, String)> {
        // Interface
        let tuning = Tuning::eadgbe();
        let opt_root = Note::from_string(root);
        let parse_result = scale_name.parse::<ScaleType>();
        match (opt_root, parse_result) {
            (Some(root), Ok(scale_type)) => {
                let scale = Scale::from_type_and_root(root, scale_type);
                let mut strings = layout_on_fretboard(scale.get_notes(), &tuning);
                let fretboard = join_strings(&mut strings);

                let degrees = scale.degrees_in_scale().collect::<Vec<_>>().join("\t");
                let notes = scale.notes_in_scale().collect::<Vec<_>>().join("\t");
                return Some((fretboard, degrees, notes));
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

    pub fn all_notes_on_fretboard(note_name: &str) -> Option<String> {
        let note = Note::from_string(note_name)?;
        let tuning = Tuning::eadgbe();
        return Some(join_strings(&mut layout_on_fretboard(&vec![note], &tuning)));
    }

    pub fn print_fret_numbers() -> String {
        let fret_numbers = (0..24)
            .map(|x| pad_to_length(&x.to_string()))
            .collect::<Vec<_>>()
            .join(" ");
        return "   ".to_owned() + &fret_numbers;
    }

    fn layout_on_fretboard(notes: &Vec<Note>, tuning: &Tuning) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let root_str = notes.first().unwrap().to_string();
        let capitalize_if_root = |s: &str| {
            if s == root_str {
                s.to_uppercase()
            } else {
                s.to_owned()
            }
        };
        for base_note in tuning.get_basenotes().iter() {
            // for each string on guitar
            let frets = locate_on_string(notes, base_note);
            let mut nut_string = base_note.to_string().to_string().to_uppercase();

            if let Some(note) = frets[0] {
                let note_str = capitalize_if_root((*base_note + note).to_string());
                nut_string = format!("{}| {}", nut_string, pad_to_length(&note_str));
            } else {
                nut_string += "| - ";
            }
            let mut fret_strings = vec![nut_string];

            for f in frets.iter().skip(1) {
                if let Some(note) = f {
                    let mut note_string = capitalize_if_root((*base_note + *note).to_string());
                    note_string = pad_to_length(&note_string);
                    fret_strings.push(note_string);
                } else {
                    fret_strings.push(String::from("- "));
                }
            }
            result.push(fret_strings.join(" "));
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
        let opt_result = scale_on_fretboard(scale_name, root);
        if let Some(result) = opt_result {
            println!("{}", result.2);
            println!("{}", result.1);
            println!("{}", result.0);
        } else {
            panic!("Something went wrong");
        }
    }
} //
