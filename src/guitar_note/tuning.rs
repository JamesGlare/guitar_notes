
use super::note::Note;
pub enum TuningType {
    eadgbe,
    dropd,
}
pub struct Tuning {
    tuning_type: TuningType,
    note_strings: Vec<String>,
    base_notes: Vec<Note>,
}
impl Tuning {
    /* semitone steps relative to lowest C */
    const EADGBE: [i32; 6] = [
        4,  /*e*/
        9,  /*a*/
        14, /*d*/
        19, /*g*/
        23, /*b*/
        28, /*e*/
    ];
    const DROPD: [i32; 6] = [
        2,  /*d*/
        9,  /*a*/
        14, /*d*/
        19, /*g*/
        23, /*b*/
        28, /*e*/
    ];
    pub fn get_basenotes<'a>(&'a self) -> &'a Vec<Note> {
        return &self.base_notes;
    }
    pub fn get_type(&self) -> &TuningType {
        return &self.tuning_type;
    }
    pub fn eadgbe() -> Tuning {
        let note_str_it = Tuning::EADGBE.iter().map(|i| {
            Note {
                semitones: i.clone(),
            }
            .to_string()
            .to_owned()
        });
        let base_notes = Tuning::EADGBE
            .iter()
            .map(|s| Note { semitones: *s })
            .collect::<Vec<_>>();
        return Tuning {
            tuning_type: TuningType::eadgbe,
            note_strings: note_str_it.collect::<Vec<_>>(),
            base_notes: base_notes,
        };
    }
    pub fn dropd() -> Tuning {
        let note_str_it = Tuning::DROPD.iter().map(|i| {
            Note {
                semitones: i.clone(),
            }
            .to_string()
            .to_owned()
        });
        let base_notes = Tuning::DROPD
            .iter()
            .map(|s| Note { semitones: *s })
            .collect::<Vec<_>>();
        return Tuning {
            tuning_type: TuningType::dropd,
            note_strings: note_str_it.collect::<Vec<_>>(),
            base_notes: base_notes,
        };
    }
    pub fn tune(&self, strings: &Vec<&str>, notes: &Vec<Note>) -> Vec<Option<Note>> {
        let mut result: Vec<Option<Note>> = vec![];
        let mut lower_string_idx = 0;
        for (s, n) in strings.iter().zip(notes) {
            let opt_pos = self
                .note_strings
                .iter()
                .skip(lower_string_idx)
                .position(|gs| gs == s);
            if let Some(pos) = opt_pos {
                result.push(Some(Note {
                    semitones: n.semitones + self.base_notes[pos + lower_string_idx].semitones,
                }));
                lower_string_idx = pos + 1;
            } else {
                result.push(None);
            }
        }
        return result;
    }
}
#[test]
fn test_tune() {
    let tuning = Tuning::eadgbe();
    // 1. test
    let strings = vec!["a", "g", "e"];
    let offsets = vec![3, 5, 7]
        .iter()
        .map(|s| Note { semitones: *s })
        .collect::<Vec<_>>();
    let result = tuning.tune(&strings, &offsets);
    println!("{:?}", result);
    assert_eq!(result[0], Some(Note { semitones: 12 }));
    assert_eq!(result[1], Some(Note { semitones: 24 }));
    assert_eq!(result[2], Some(Note { semitones: 35 }));
}
