extern crate enum_utils;
extern crate itertools;
use super::note::Note;
use itertools::Itertools;
use std::cmp;

/* Chords
 */
#[derive(Clone, enum_utils::FromStr, enum_utils::IterVariants, PartialEq, Debug)]
enum Triad {
    minor,
    minor_omitted_5,
    major,
    major_omitted_5,
    sus2,
    sus2_omitted_5,
    sus4,
    sus4_omitted_5,
    minor_diminished,
    major_diminished,
    plus,
}
impl Triad {
    pub fn to_string(&self) -> &str {
        return match self {
            Triad::minor => "m",
            Triad::minor_omitted_5 => "m",
            Triad::major => "",
            Triad::major_omitted_5 => "",
            Triad::sus2 => "sus2",
            Triad::sus2_omitted_5 => "sus2",
            Triad::sus4 => "sus4",
            Triad::sus4_omitted_5 => "sus4",
            Triad::minor_diminished => "mdim",
            Triad::major_diminished => "dim",
            Triad::plus => "+",
        };
    }
}
#[derive(Clone, enum_utils::FromStr, enum_utils::IterVariants, PartialEq, Debug)]
enum ChordInterval {
    omitted,
    minor_2,
    major_2,
    minor_3,
    major_3,
    perfect_4,
    perfect_5,
    minor_6,
    major_6,
    minor_7,
    major_7,
    minor_9,
    major_9,
    plus_9,
    perfect_11,
    augmented_11,
}
impl ChordInterval {
    pub fn to_string(&self) -> &str {
        return match self {
            ChordInterval::omitted => "",
            ChordInterval::minor_2 => "2m",
            ChordInterval::major_2 => "2",
            ChordInterval::minor_3 => "3m",
            ChordInterval::major_3 => "3",
            ChordInterval::perfect_4 => "4",
            ChordInterval::perfect_5 => "5",
            ChordInterval::minor_6 => "6m",
            ChordInterval::major_6 => "6",
            ChordInterval::minor_7 => "7",
            /* for the seven, indicate the major */
            ChordInterval::major_7 => "7maj",
            ChordInterval::minor_9 => "9-",
            ChordInterval::major_9 => "9",
            ChordInterval::plus_9 => "9+",
            ChordInterval::perfect_11 => "11",
            ChordInterval::augmented_11 => "aug11",
        };
    }
}
#[derive(Clone, PartialEq, Debug)]
enum ChordType {
    twotone {
        t: ChordInterval,
    },
    triad {
        t: Triad,
    },
    extended_triad {
        t: Triad,
        e: ChordInterval,
    },
    four_chord {
        t: Triad,
        e: ChordInterval,
    },
    nine_chord {
        t: Triad,
        e1: ChordInterval,
        e2: ChordInterval,
    },
    eleven_chord {
        t: Triad,
        e1: ChordInterval,
        e2: ChordInterval,
        e3: ChordInterval,
    },
}
#[derive(Clone)]
pub struct Chord {
    type_: ChordType,
    notes: Vec<Note>,
}
impl Chord {
    const MINOR_2: Note = Note { semitones: 1 };
    const MAJOR_2: Note = Note { semitones: 2 };
    const MINOR_3: Note = Note { semitones: 3 };
    const MAJOR_3: Note = Note { semitones: 4 };
    const PERFECT_4: Note = Note { semitones: 5 };
    const FLATTENED_5: Note = Note { semitones: 6 };
    const PERFECT_5: Note = Note { semitones: 7 };
    const AUGMENTED_5: Note = Note { semitones: 8 };
    const MINOR_6: Note = Note { semitones: 8 };
    const MAJOR_6: Note = Note { semitones: 9 };
    const MINOR_7: Note = Note { semitones: 10 };
    const MAJOR_7: Note = Note { semitones: 11 };
    const MINOR_9: Note = Note { semitones: 13 };
    const MAJOR_9: Note = Note { semitones: 14 };
    const PLUS_9: Note = Note { semitones: 15 };
    const PERFECT_11: Note = Note { semitones: 17 };
    const AUGMENTED_11: Note = Note { semitones: 18 };

    fn build_inversions(intervals: &Vec<Note>) -> Vec<Vec<Note>> {
        let mut octaves_spanned = 1;
        if !intervals.is_empty() {
            let span = *intervals.iter().max().unwrap() - *intervals.iter().min().unwrap();
            octaves_spanned = cmp::max((span.semitones as f64 / 12 as f64).ceil() as i32, 1);
        }
        let n_notes = intervals.len(); // n-1 inversion
        let mut result: Vec<Vec<Note>> = vec![];
        for i in 1..n_notes {
            let front = intervals
                .iter()
                .take(i)
                .map(|s| *s + octaves_spanned * Note::octave());
            let back = intervals.iter().skip(i).take(n_notes - i).map(|s| *s);
            let inversion_intervals = back
                .chain(front)
                .unique_by(|s| s.no_octaves())
                .collect::<Vec<_>>();
            result.push(inversion_intervals);
        }
        return result;
    }
    pub fn to_string(&self) -> String {
        let type_str = match &self.type_ {
            ChordType::twotone { t } => format!("{}", t.to_string()),
            ChordType::triad { t } => format!("{}", t.to_string()),
            ChordType::extended_triad { t, e } => format!("{}add{}", t.to_string(), e.to_string()),
            ChordType::four_chord { t, e } => format!("{}{}", t.to_string(), e.to_string()),
            ChordType::nine_chord { t, e1, e2 } => format!("{}{}", t.to_string(), e2.to_string()),
            ChordType::eleven_chord { t, e1, e2, e3 } => format!("11{}", t.to_string()),
        };
        return format!("{}{}", self.notes[0].to_string().to_uppercase(), type_str);
    }
    pub fn get_notes(&self) -> &Vec<Note> {
        return &self.notes;
    }
    fn from_intervals(root: Note, intervals: &Vec<Note>, type_: ChordType) -> Chord {
        let mut notes = intervals
            .iter()
            .map(|x| *x)
            .scan(root, |state, n| {
                *state = *state + n;
                return Some(*state);
            })
            .collect::<Vec<_>>();
        notes.insert(0, root);
        return Chord {
            notes: notes,
            type_: type_,
        };
    }
    fn intervals(&self) -> Vec<Note> {
        let mut notes = self.notes.clone();
        notes.sort();
        return notes
            .windows(2)
            .map(|slice| (slice[1] - slice[0]))
            .collect::<Vec<_>>();
    }
    fn all_contained_in(a: &[Note], b: &Vec<Note>) -> Option<Vec<Note>> {
        if a.iter().all(|item| b.contains(item)) {
            return Some(a.to_vec());
        } else {
            return None;
        }
    }
    fn match_twotone(root: &Note, intervals: &Vec<Note>) -> Option<Chord> {
        if let Some(notes) = Chord::all_contained_in(&[Chord::PERFECT_5], intervals) {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::twotone {
                    t: ChordInterval::perfect_5,
                },
            ));
        } else if let Some(notes) = Chord::all_contained_in(&[Chord::MINOR_2], intervals) {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::twotone {
                    t: ChordInterval::minor_2,
                },
            ));
        } else if let Some(notes) = Chord::all_contained_in(&[Chord::MAJOR_2], intervals) {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::twotone {
                    t: ChordInterval::major_2,
                },
            ));
        } else if let Some(notes) = Chord::all_contained_in(&[Chord::MINOR_3], intervals) {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::twotone {
                    t: ChordInterval::minor_3,
                },
            ));
        } else if let Some(notes) = Chord::all_contained_in(&[Chord::MAJOR_3], intervals) {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::twotone {
                    t: ChordInterval::major_3,
                },
            ));
        } else if let Some(notes) = Chord::all_contained_in(&[Chord::PERFECT_4], intervals) {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::twotone {
                    t: ChordInterval::perfect_4,
                },
            ));
        } else if let Some(notes) = Chord::all_contained_in(&[Chord::MINOR_6], intervals) {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::twotone {
                    t: ChordInterval::minor_6,
                },
            ));
        } else if let Some(notes) = Chord::all_contained_in(&[Chord::MINOR_7], intervals) {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::twotone {
                    t: ChordInterval::minor_7,
                },
            ));
        } else if let Some(notes) = Chord::all_contained_in(&[Chord::MAJOR_7], intervals) {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::twotone {
                    t: ChordInterval::major_7,
                },
            ));
        } else if let Some(notes) = Chord::all_contained_in(&[Chord::MINOR_9], intervals) {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::twotone {
                    t: ChordInterval::minor_9,
                },
            ));
        } else if let Some(notes) = Chord::all_contained_in(&[Chord::MAJOR_9], intervals) {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::twotone {
                    t: ChordInterval::major_9,
                },
            ));
        } else if let Some(notes) = Chord::all_contained_in(&[Chord::PERFECT_11], intervals) {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::twotone {
                    t: ChordInterval::perfect_11,
                },
            ));
        } else if let Some(notes) = Chord::all_contained_in(&[Chord::AUGMENTED_11], intervals) {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::twotone {
                    t: ChordInterval::augmented_11,
                },
            ));
        } else {
            return None;
        }
    }
    fn match_triad(root: &Note, intervals: &Vec<Note>) -> Option<Chord> {
        if let Some(notes) = Chord::all_contained_in(&[Chord::MINOR_3, Chord::PERFECT_5], intervals)
        {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::triad { t: Triad::minor },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MAJOR_3, Chord::PERFECT_5], intervals)
        {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::triad { t: Triad::major },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MAJOR_2, Chord::PERFECT_5], intervals)
        {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::triad { t: Triad::sus2 },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::PERFECT_4, Chord::PERFECT_5], intervals)
        {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::triad { t: Triad::sus4 },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MINOR_3, Chord::FLATTENED_5], intervals)
        {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::triad {
                    t: Triad::minor_diminished,
                },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MAJOR_3, Chord::FLATTENED_5], intervals)
        {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::triad {
                    t: Triad::major_diminished,
                },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MAJOR_3, Chord::AUGMENTED_5], intervals)
        {
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::triad { t: Triad::plus },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MINOR_3, Chord::MAJOR_6], intervals)
        {
            // a six with the perfect 5 omitted
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::extended_triad {
                    t: Triad::minor_omitted_5,
                    e: ChordInterval::major_6,
                },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MAJOR_3, Chord::MAJOR_6], intervals)
        {
            // a six with the perfect 5 omitted
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::extended_triad {
                    t: Triad::major_omitted_5,
                    e: ChordInterval::major_6,
                },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MINOR_3, Chord::MINOR_7], intervals)
        {
            // a seven with the perfect 5 omitted
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::four_chord {
                    t: Triad::minor_omitted_5,
                    e: ChordInterval::minor_7,
                },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MINOR_3, Chord::MAJOR_7], intervals)
        {
            // a seven with the perfect 5 omitted
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::four_chord {
                    t: Triad::minor_omitted_5,
                    e: ChordInterval::major_7,
                },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MAJOR_3, Chord::MINOR_7], intervals)
        {
            // a seven with the perfect 5 omitted
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::four_chord {
                    t: Triad::major_omitted_5,
                    e: ChordInterval::minor_7,
                },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MAJOR_3, Chord::MAJOR_7], intervals)
        {
            // a seven with the perfect 5 omitted
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::four_chord {
                    t: Triad::major_omitted_5,
                    e: ChordInterval::major_7,
                },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MINOR_3, Chord::MINOR_9], intervals)
        {
            // a seven with the perfect 5 omitted
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::extended_triad {
                    t: Triad::minor_omitted_5,
                    e: ChordInterval::minor_9,
                },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MINOR_3, Chord::MAJOR_9], intervals)
        {
            // a seven with the perfect 5 omitted
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::extended_triad {
                    t: Triad::minor_omitted_5,
                    e: ChordInterval::major_9,
                },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MAJOR_3, Chord::MINOR_9], intervals)
        {
            // a seven with the perfect 5 omitted
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::extended_triad {
                    t: Triad::major_omitted_5,
                    e: ChordInterval::minor_9,
                },
            ));
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MAJOR_3, Chord::MAJOR_9], intervals)
        {
            // a seven with the perfect 5 omitted
            return Some(Chord::from_intervals(
                *root,
                &notes,
                ChordType::extended_triad {
                    t: Triad::major_omitted_5,
                    e: ChordInterval::major_9,
                },
            ));
        } else {
            return None;
        }
    }
    fn match_four_chord(root: &Note, intervals: &Vec<Note>) -> Option<Chord> {
        if let Some(notes) = Chord::all_contained_in(
            &[Chord::MAJOR_3, Chord::MINOR_7, Chord::MAJOR_9],
            &intervals,
        ) {
            let new_chord = Chord::from_intervals(
                *root,
                &notes,
                ChordType::nine_chord {
                    t: Triad::major_omitted_5,
                    e1: ChordInterval::minor_7,
                    e2: ChordInterval::major_9,
                },
            );
            return Some(new_chord);
        } else if let Some(notes) = Chord::all_contained_in(
            &[Chord::MAJOR_3, Chord::MINOR_7, Chord::MAJOR_2],
            &intervals,
        ) {
            // this is a nine chord, omitted five on one octave
            let new_chord = Chord::from_intervals(
                *root,
                &notes,
                ChordType::nine_chord {
                    t: Triad::major_omitted_5,
                    e1: ChordInterval::minor_7,
                    e2: ChordInterval::major_9,
                },
            );
            return Some(new_chord);
        } else if let Some(notes) = Chord::all_contained_in(
            &[Chord::MINOR_3, Chord::MINOR_7, Chord::MAJOR_9],
            &intervals,
        ) {
            // this is a nine chord, omitted five
            let new_chord = Chord::from_intervals(
                *root,
                &notes,
                ChordType::nine_chord {
                    t: Triad::minor_omitted_5,
                    e1: ChordInterval::minor_7,
                    e2: ChordInterval::major_9,
                },
            );
            return Some(new_chord);
        } else if let Some(notes) = Chord::all_contained_in(
            &[Chord::MINOR_3, Chord::MINOR_7, Chord::MAJOR_2],
            &intervals,
        ) {
            // this is a nine chord, omitted five on one octave
            let new_chord = Chord::from_intervals(
                *root,
                &notes,
                ChordType::nine_chord {
                    t: Triad::minor_omitted_5,
                    e1: ChordInterval::minor_7,
                    e2: ChordInterval::major_9,
                },
            );
            return Some(new_chord);
        } else if let Some(notes) = Chord::all_contained_in(
            &[Chord::MAJOR_2, Chord::MINOR_7, Chord::MAJOR_9],
            &intervals,
        ) {
            let new_chord = Chord::from_intervals(
                *root,
                &notes,
                ChordType::nine_chord {
                    t: Triad::sus2_omitted_5,
                    e1: ChordInterval::minor_7,
                    e2: ChordInterval::major_9,
                },
            );
            return Some(new_chord);
        } else if let Some(notes) = Chord::all_contained_in(
            &[Chord::PERFECT_4, Chord::MINOR_7, Chord::MAJOR_9],
            &intervals,
        ) {
            let new_chord = Chord::from_intervals(
                *root,
                &notes,
                ChordType::nine_chord {
                    t: Triad::sus4_omitted_5,
                    e1: ChordInterval::minor_7,
                    e2: ChordInterval::major_9,
                },
            );
            return Some(new_chord);
        } else if let Some(notes) = Chord::all_contained_in(
            &[Chord::PERFECT_4, Chord::MINOR_7, Chord::MAJOR_2],
            &intervals,
        ) {
            // this is a ninesus chord, omitted five on one octave
            let new_chord = Chord::from_intervals(
                *root,
                &notes,
                ChordType::nine_chord {
                    t: Triad::sus4_omitted_5,
                    e1: ChordInterval::minor_7,
                    e2: ChordInterval::major_9,
                },
            );
            return Some(new_chord);
        } else if let Some(notes) = Chord::all_contained_in(
            &[Chord::MAJOR_3, Chord::MINOR_7, Chord::MINOR_9],
            &intervals,
        ) {
            // this is a ninesus chord, omitted five on one octave
            let new_chord = Chord::from_intervals(
                *root,
                &notes,
                ChordType::nine_chord {
                    t: Triad::major_omitted_5,
                    e1: ChordInterval::minor_7,
                    e2: ChordInterval::minor_9,
                },
            );
            return Some(new_chord);
        } else if let Some(notes) =
            Chord::all_contained_in(&[Chord::MAJOR_3, Chord::MINOR_7, Chord::PLUS_9], &intervals)
        {
            // this is a ninesus chord, omitted five on one octave
            let new_chord = Chord::from_intervals(
                *root,
                &notes,
                ChordType::nine_chord {
                    t: Triad::major_omitted_5,
                    e1: ChordInterval::minor_7,
                    e2: ChordInterval::plus_9,
                },
            );
            return Some(new_chord);
        } else if let Some(notes) = Chord::all_contained_in(
            &[Chord::MAJOR_3, Chord::MINOR_7, Chord::PERFECT_11],
            &intervals,
        ) {
            // this is a ninesus chord, omitted five on one octave
            let new_chord = Chord::from_intervals(
                *root,
                &notes,
                ChordType::eleven_chord {
                    t: Triad::major_omitted_5,
                    e1: ChordInterval::minor_7,
                    e2: ChordInterval::omitted,
                    e3: ChordInterval::perfect_11,
                },
            );
            return Some(new_chord);
        } else {
            return None;
        }
    }
    fn match_extended_triad(triad: &Chord, intervals: &Vec<Note>) -> Option<Chord> {
        let mut new_chord = triad.clone(); // lazy workaround
                                           /* These are chords that begin with a standard triad */
        if let ChordType::triad { t } = &triad.type_ {
            if let Some(_) = Chord::all_contained_in(&[Chord::MINOR_2], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::minor_2,
                };
                new_chord.notes.push(Chord::MINOR_2);
                return Some(new_chord);
            } else if let Some(_) = Chord::all_contained_in(&[Chord::MAJOR_2], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::major_2,
                };
                new_chord.notes.push(Chord::MAJOR_2);
                return Some(new_chord);
            } else if let Some(_) = Chord::all_contained_in(&[Chord::MINOR_6], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::minor_6,
                };
                new_chord.notes.push(Chord::MINOR_6);
                return Some(new_chord);
            } else if let Some(_) = Chord::all_contained_in(&[Chord::MAJOR_6], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::major_6,
                };
                new_chord.notes.push(Chord::MAJOR_6);
                return Some(new_chord);
            } else if let Some(_) = Chord::all_contained_in(&[Chord::MAJOR_7], &intervals) {
                new_chord.type_ = ChordType::four_chord {
                    t: t.clone(),
                    e: ChordInterval::major_7,
                };
                new_chord.notes.push(Chord::MAJOR_7);
                return Some(new_chord);
            } else if let Some(_) = Chord::all_contained_in(&[Chord::MINOR_7], &intervals) {
                new_chord.type_ = ChordType::four_chord {
                    t: t.clone(),
                    e: ChordInterval::minor_7,
                };
                new_chord.notes.push(Chord::MINOR_7);
                return Some(new_chord);
            } else if let Some(_) = Chord::all_contained_in(&[Chord::MINOR_9], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::minor_9,
                };
                new_chord.notes.push(Chord::MINOR_9);
                return Some(new_chord);
            } else if let Some(_) = Chord::all_contained_in(&[Chord::MAJOR_9], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::major_9,
                };
                new_chord.notes.push(Chord::MAJOR_9);
                return Some(new_chord);
            } else if let Some(_) = Chord::all_contained_in(&[Chord::PERFECT_11], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::perfect_11,
                };
                new_chord.notes.push(Chord::PERFECT_11);
                return Some(new_chord);
            } else if let Some(_) = Chord::all_contained_in(&[Chord::AUGMENTED_11], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::augmented_11,
                };
                new_chord.notes.push(Chord::PERFECT_11);
                return Some(new_chord);
            }
        }
        return None;
    }
    fn match_five_chord(
        root: &Note,
        opt_chord: &Option<Chord>,
        intervals: &Vec<Note>,
    ) -> Option<Chord> {
        if let Some(partial_chord) = opt_chord {
            let mut new_chord = partial_chord.clone(); // lazy workaround
            if let ChordType::triad { t } = &partial_chord.type_ {
                if let Some(_) =
                    Chord::all_contained_in(&[Chord::MINOR_7, Chord::MAJOR_9], &intervals)
                {
                    new_chord.type_ = ChordType::nine_chord {
                        t: t.clone(),
                        e1: ChordInterval::minor_7,
                        e2: ChordInterval::major_9,
                    };
                    new_chord.notes.push(Chord::MAJOR_2);
                    return Some(new_chord);
                } else if let Some(_) =
                    Chord::all_contained_in(&[Chord::MAJOR_7, Chord::MAJOR_9], &intervals)
                {
                    new_chord.type_ = ChordType::nine_chord {
                        t: t.clone(),
                        e1: ChordInterval::major_7,
                        e2: ChordInterval::major_9,
                    };
                    new_chord.notes.push(Chord::MAJOR_2);
                    return Some(new_chord);
                } else if let Some(_) =
                    Chord::all_contained_in(&[Chord::MAJOR_7, Chord::MINOR_9], &intervals)
                {
                    new_chord.type_ = ChordType::nine_chord {
                        t: t.clone(),
                        e1: ChordInterval::major_7,
                        e2: ChordInterval::minor_9,
                    };
                    new_chord.notes.push(Chord::MINOR_9);
                    return Some(new_chord);
                } else if let Some(_) =
                    Chord::all_contained_in(&[Chord::MINOR_7, Chord::PLUS_9], &intervals)
                {
                    new_chord.type_ = ChordType::nine_chord {
                        t: t.clone(),
                        e1: ChordInterval::major_7,
                        e2: ChordInterval::plus_9,
                    };
                    new_chord.notes.push(Chord::PLUS_9);
                    return Some(new_chord);
                }
            }
        }
        return None;
    }
    pub fn find_chord(notes: &Vec<Note>) -> Vec<Option<Chord>> {
        let mut results: Vec<Option<Chord>> = Vec::new();
        let mut steps_to_root = vec![notes
            .iter()
            .map(|s| *s)
            .unique_by(|s1| s1.no_octaves()) // delete duplicate notes
            .collect::<Vec<_>>()];
        steps_to_root.sort();
        let mut inversions = Chord::build_inversions(&steps_to_root[0]);
        steps_to_root.append(&mut inversions);

        for inversion in &mut steps_to_root {
            let root = inversion.iter().min().unwrap();

            // build relative semitone steps
            let relative = inversion
                .iter()
                .map(|s| (*s - *root))
                .map(|s| {
                    if s <= Chord::AUGMENTED_11 {
                        s
                    } else {
                        s.no_octaves()
                    }
                })
                .collect::<Vec<_>>();
            match inversion.len() as i32 {
                2 => {
                    /* Probably a POWER CHORD */
                    let twotone = relative.iter().map(|s| s.no_octaves()).collect::<Vec<_>>();
                    let opt_chord = Chord::match_twotone(root, &twotone);
                    results.push(opt_chord);
                }
                3 => {
                    /* TRIAD CHORD */
                    let triad = relative.iter().map(|s| *s).collect::<Vec<_>>();
                    let opt_chord = Chord::match_triad(root, &triad);
                    if let Some(_) = opt_chord {
                        results.push(opt_chord);
                    }
                    let triad_no_oct = relative.iter().map(|s| s.no_octaves()).collect::<Vec<_>>();
                    let opt_chord = Chord::match_triad(root, &triad_no_oct);
                    results.push(opt_chord);
                }
                4 => {
                    /* TRIAD CHORD add some other tone or a seven chord*/
                    let no_octaves = relative.iter().map(|n| n.no_octaves()).collect::<Vec<_>>();
                    let opt_chord = Chord::match_triad(root, &no_octaves);
                    // try different things with this chord
                    let mut opt_four_chord = None;
                    // in order to be able to differentiate, e.g. a 2 from a 9,
                    // we need to keep octave information at first
                    // if we don't find anything, then squash the octave and try again
                    if let Some(triad) = opt_chord {
                        // filter out remaining note
                        let fourth_note = *relative
                            .iter()
                            .find(|n| !triad.intervals().contains(*n) && n.semitones != 0)
                            .unwrap();
                        opt_four_chord = Chord::match_extended_triad(&triad, &vec![fourth_note]);
                        if opt_four_chord.is_none() {
                            // try again without octaves
                            let fourth_note = *no_octaves
                                .iter()
                                .find(|n| !triad.intervals().contains(*n) && n.semitones != 0)
                                .unwrap();
                            opt_four_chord =
                                Chord::match_extended_triad(&triad, &vec![fourth_note]);
                        }
                    }
                    if opt_four_chord.is_none() {
                        opt_four_chord = Chord::match_four_chord(root, &relative);
                        if opt_four_chord.is_none() {
                            opt_four_chord = Chord::match_four_chord(root, &no_octaves);
                        }
                    }
                    results.push(opt_four_chord);
                }
                5 => {
                    let no_octaves = relative.iter().map(|s| s.no_octaves()).collect::<Vec<_>>();
                    let opt_chord = Chord::match_triad(root, &no_octaves);

                    // in order to be able to differentiate, e.g. a 2 from a 9,
                    // we need to keep octave information
                    let mut opt_extended_triad =
                        Chord::match_five_chord(root, &opt_chord, &relative);
                    if opt_extended_triad.is_none() {
                        opt_extended_triad = Chord::match_five_chord(root, &opt_chord, &no_octaves);
                    }
                    results.push(opt_extended_triad);
                }
                _ => {
                    results.push(None);
                }
            }
        }
        return results;
    }
}
#[test]
fn test_first_inversion() {
    let intervals = [Note { semitones: 0 }, Chord::MINOR_3, Chord::PERFECT_5]
        .iter()
        .map(|n| *n)
        .collect::<Vec<_>>();
    let minor_inversions = Chord::build_inversions(&intervals);
    assert_eq!(
        minor_inversions[0],
        vec![3, 7, 12]
            .iter()
            .map(|s| Note { semitones: *s })
            .collect::<Vec<_>>()
    );
    assert_eq!(
        minor_inversions[1],
        vec![7, 12, 15]
            .iter()
            .map(|s| Note { semitones: *s })
            .collect::<Vec<_>>()
    );
    assert_eq!(minor_inversions.len(), 2);
    // now build from inversion
    let intervals2 = [5, 0, 3]
        .iter()
        .map(|s| Note { semitones: *s })
        .collect::<Vec<_>>();
    let inversion2 = Chord::build_inversions(&intervals2);
    assert_eq!(
        inversion2[0],
        vec![0, 3, 17]
            .iter()
            .map(|s| Note { semitones: *s })
            .collect::<Vec<_>>()
    );
    assert_eq!(
        inversion2[1],
        vec![3, 17, 12]
            .iter()
            .map(|s| Note { semitones: *s })
            .collect::<Vec<_>>()
    );
    assert_eq!(inversion2.len(), 2);
}
#[test]
fn test_find_chord() {
    let notes1 = [Note { semitones: 0 }, Chord::MINOR_3, Chord::PERFECT_5]
        .iter()
        .map(|s| *s + Note { semitones: 3 })
        .collect::<Vec<_>>();
    let notes2 = [Note { semitones: 0 }, Chord::MAJOR_2, Chord::PERFECT_5]
        .iter()
        .map(|s| *s + Note { semitones: 3 })
        .collect::<Vec<_>>();
    let notes3 = vec![0, 4, 7, 10]
        .iter()
        .map(|s| Note { semitones: *s + 12 })
        .collect::<Vec<_>>();
    let notes4 = vec![0, 3, 7, 14]
        .iter()
        .map(|s| Note { semitones: *s })
        .collect::<Vec<_>>();
    let notes5 = vec![0, 7]
        .iter()
        .map(|s| Note { semitones: *s + 0 })
        .collect::<Vec<_>>();
    let notes6 = vec![0, 4, 9, 7]
        .iter()
        .map(|s| Note { semitones: *s + 4 })
        .collect::<Vec<_>>();
    let notes7 = vec![0, 4, 8]
        .iter()
        .map(|s| Note { semitones: *s + 31 })
        .collect::<Vec<_>>();
    // second inversion of a major chord
    let notes8 = vec![0, 3, 8]
        .iter()
        .map(|s| Note { semitones: *s + 5 })
        .collect::<Vec<_>>();
    // first inversion of a major chord
    let notes9 = vec![0, 5, 9]
        .iter()
        .map(|s| Note { semitones: *s + 3 })
        .collect::<Vec<_>>();
    // 7 Chord, omitted five
    let notes10 = vec![0, 4, 10]
        .iter()
        .map(|s| Note { semitones: *s + 11 })
        .collect::<Vec<_>>();
    let chord1 = Chord::find_chord(&notes1);
    let chord2 = Chord::find_chord(&notes2);
    let chord3 = Chord::find_chord(&notes3);
    let chord4 = Chord::find_chord(&notes4);
    let chord5 = Chord::find_chord(&notes5);
    let chord6 = Chord::find_chord(&notes6);
    let chord7 = Chord::find_chord(&notes7);
    let chord8 = Chord::find_chord(&notes8);
    let chord9 = Chord::find_chord(&notes9);
    let chord10 = Chord::find_chord(&notes10);
    assert_eq!(
        chord1[0].clone().unwrap().type_,
        ChordType::triad { t: Triad::minor }
    );
    assert_eq!(
        chord2[0].clone().unwrap().type_,
        ChordType::triad { t: Triad::sus2 }
    );
    assert_eq!(
        chord3[0].clone().unwrap().type_,
        ChordType::four_chord {
            t: Triad::major,
            e: ChordInterval::minor_7
        }
    );
    assert_eq!(
        chord4[0].clone().unwrap().type_,
        ChordType::extended_triad {
            t: Triad::minor,
            e: ChordInterval::major_9
        }
    );
    assert_eq!(
        chord5[0].clone().unwrap().type_,
        ChordType::twotone {
            t: ChordInterval::perfect_5
        }
    );
    assert_eq!(
        chord6[0].clone().unwrap().type_,
        ChordType::extended_triad {
            t: Triad::major,
            e: ChordInterval::major_6
        }
    );
    assert_eq!(
        chord7[0].clone().unwrap().type_,
        ChordType::triad { t: Triad::plus }
    );
    assert_eq!(
        chord8[2].clone().unwrap().type_,
        ChordType::triad { t: Triad::major }
    );
    assert_eq!(
        chord9[1].clone().unwrap().type_,
        ChordType::triad { t: Triad::major }
    );
    assert_eq!(
        chord10[0].clone().unwrap().type_,
        ChordType::four_chord {
            t: Triad::major_omitted_5,
            e: ChordInterval::minor_7
        }
    );
}
