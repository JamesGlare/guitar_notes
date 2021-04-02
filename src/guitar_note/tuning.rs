use super::note::Note;
#[derive(Clone, enum_utils::FromStr, enum_utils::IterVariants, Debug)]
enum TuningType {
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
    openc,
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
    const DOUBLEDROPD: [i32; 6] = [
        2,  /*d*/
        9,  /*a*/
        14, /*d*/
        19, /*g*/
        23, /*b*/
        26, /*d*/
    ];
    const DADGAD: [i32; 6] = [
        2,  /*d*/
        9,  /*a*/
        14, /*d*/
        19, /*g*/
        21, /*a*/
        26, /*d*/
    ];
    const OPEND: [i32; 6] = [
        2,  /*d*/
        9,  /*a*/
        14, /*d*/
        18, /*f#*/
        21, /*a*/
        26, /*d*/
    ];
    const OPENE: [i32; 6] = [
        4,  /*e*/
        11, /*b*/
        14, /*d*/
        20, /*g#*/
        23, /*b*/
        28, /*e*/
    ];
    const OPENG: [i32; 6] = [
        2,  /*d*/
        11, /*g*/
        14, /*d*/
        19, /*g*/
        23, /*b*/
        26, /*d*/
    ];
    const OPENA: [i32; 6] = [
        4,  /*e*/
        9,  /*a*/
        16, /*e*/
        21, /*a*/
        25, /*c#*/
        28, /*e*/
    ];
    const OPENC: [i32; 6] = [
        0,  /*c*/
        7,  /*g*/
        12, /*c*/
        19, /*g*/
        24, /*c*/
        28, /*e*/
    ];
    const DGCGCD: [i32; 6] = [
        2,  /*d*/
        7,  /*g*/
        12, /*c*/
        19, /*g*/
        24, /*c*/
        26, /*d*/
    ];
    const OPENC6: [i32; 6] = [
        0,  /*c*/
        9,  /*a*/
        12, /*c*/
        19, /*g*/
        24, /*c*/
        28, /*e*/
    ];
    const EEEEBE: [i32; 6] = [
        4,  /*e*/
        4,  /*e*/
        16, /*e*/
        16, /*e*/
        23, /*b*/
        28, /*e*/
    ];
    pub fn get_basenotes<'a>(&'a self) -> &'a Vec<Note> {
        return &self.base_notes;
    }
    pub fn to_string(&self) -> String {
        return format!("{:?}", self.tuning_type);
    }
    pub fn from_name(name: &str) -> Option<Tuning> {
        let type_ = name.parse::<TuningType>();
        return match type_ {
            Ok(tuning_type) => Some(Tuning::from_type(tuning_type)),
            _ => None,
        };
    }
    fn from_type(tuning_type: TuningType) -> Tuning {
        let notes = match tuning_type {
            TuningType::dropd => &Tuning::DROPD,
            TuningType::doubledropd => &Tuning::DOUBLEDROPD,
            TuningType::dadgad => &Tuning::DADGAD,
            TuningType::dgcdcg => &Tuning::DGCGCD,
            TuningType::eeeebe => &Tuning::EEEEBE,
            TuningType::opena => &Tuning::OPENA,
            TuningType::openc => &Tuning::OPENC,
            TuningType::openc6 => &Tuning::OPENC6,
            TuningType::opend => &Tuning::OPEND,
            TuningType::opene => &Tuning::OPENE,
            TuningType::openg => &Tuning::OPENG,
            TuningType::eadgbe => &Tuning::EADGBE,
        };
        let note_str_it = notes.iter().map(|i| {
            Note {
                semitones: i.clone(),
            }
            .to_string()
            .to_owned()
        });
        let base_notes = notes
            .iter()
            .map(|s| Note { semitones: *s })
            .collect::<Vec<_>>();

        return Tuning {
            tuning_type: tuning_type,
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
    let tuning = Tuning::from_type(TuningType::eadgbe);
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
