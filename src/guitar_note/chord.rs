extern crate enum_utils;
extern crate itertools;

use super::note::Note;
use itertools::Itertools;
use std::convert::TryInto;

/* Chords
 */
#[derive(Clone, enum_utils::FromStr, enum_utils::IterVariants, PartialEq, Debug)]
enum TwoTone {
    none,
    five, // powerchord
}
#[derive(Clone, enum_utils::FromStr, enum_utils::IterVariants, PartialEq, Debug)]
enum Triad {
    none,
    minor,
    major,
    sus2,
    sus4,
    minor_diminished,
    major_diminished,
}
#[derive(Clone, enum_utils::FromStr, enum_utils::IterVariants, PartialEq, Debug)]
enum Extension {
    add_minor_2,
    add_major_2,
    minor_7,
    major_7,
    add_major_9,
    add_minor_9,
    add_11
}
#[derive(Clone, PartialEq, Debug)]
enum ChordType {
    triad { t: Triad },
    extended_triad { t: Triad, e: Extension },
    twotone { t: TwoTone },
}
#[derive(Clone)]
pub struct Chord {
    type_: ChordType,
    notes: Vec<Note>,
}
impl Chord {
    const FIVE_INTERVALS: [i32; 2] = [0, 7];
    const MINOR_INTERVALS: [i32; 3] = [0, 3, 7];
    const MAJOR_INTERVALS: [i32; 3] = [0, 4, 7];
    const MINOR_DIM_INTERVALS: [i32; 3] = [0, 3, 6];
    const MAJOR_DIM_INTERVALS: [i32; 3] = [0, 4, 6];

    const SUS_2_INTERVALS: [i32; 3] = [0, 2, 7];
    const SUS_4_INTERVALS: [i32; 3] = [0, 5, 7];
    // todo only specify major/minor 7 semitones then build by chaining
    const MINOR_TWO_INTERVAL: i32 = 1;
    const MAJOR_TWO_INTERVAL: i32 = 2;
    const MINOR_7_INTERVAL: i32 = 10;
    const MAJOR_7_INTERVAL: i32 = 11;
    const MINOR_NINTH_INTERVAL: i32 = 13;
    const MAJOR_NINTH_INTERVAL: i32 = 14;
    const ELEVENTH_INTERVAL: i32 = 17;

    fn build_inversions(intervals: &Vec<Note>) -> Vec<Vec<Note>> {
        let n_notes = intervals.len(); // n-1 inversion
        let mut result: Vec<Vec<Note>> = vec![];
        for i in 1..n_notes {
            let front = intervals.iter().take(n_notes - i).map(|s| *s);
            let back = intervals.iter().rev().take(i).map(|s| Note::octave() - *s);
            let inversion_intervals = back
                .rev()
                .chain(front)
                .map(|s| s.regauge_if_negative())
                .collect::<Vec<_>>();
            result.push(inversion_intervals);
        }
        return result;
    }

    pub fn to_string(&self) -> String {
        let type_str = match &self.type_ {
            ChordType::triad { t } => format!("{:?}", t),
            ChordType::extended_triad { t, e } => format!("{:?}-{:?}", t, e),
            ChordType::twotone { t } => format!("{:?}", t),
        };

        return format!("{}{}", self.notes[0].to_string().to_uppercase(), type_str);
    }

    fn from_intervals(root: i32, intervals: &Vec<i32>, type_: ChordType) -> Chord {
        let notes = intervals
            .iter()
            .map(|x| *x)
            .scan(root, |state, n| {
                *state = *state + n;
                return Some(*state);
            })
            .map(|i| Note { semitones: i })
            .collect::<Vec<_>>();
        return Chord {
            notes: notes,
            type_: type_,
        };
    }
    fn all_contained_in(a: &[i32], b: &Vec<i32>) -> bool {
        a.iter().all(|item| b.contains(item))
    }
    fn match_triad(root: &Note, intervals: &Vec<i32>) -> Option<Chord> {
        if Chord::all_contained_in(&Chord::MAJOR_INTERVALS, &intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                &Chord::MAJOR_INTERVALS.to_vec(),
                ChordType::triad { t: Triad::major },
            ));
        } else if Chord::all_contained_in(&Chord::MINOR_INTERVALS, &intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                &Chord::MINOR_INTERVALS.to_vec(),
                ChordType::triad { t: Triad::minor },
            ));
        } else if Chord::all_contained_in(&Chord::SUS_2_INTERVALS, &intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                &Chord::SUS_2_INTERVALS.to_vec(),
                ChordType::triad { t: Triad::sus2 },
            ));
        } else if Chord::all_contained_in(&Chord::SUS_4_INTERVALS, &intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                &Chord::SUS_4_INTERVALS.to_vec(),
                ChordType::triad { t: Triad::sus4 },
            ));
        } else if Chord::all_contained_in(&Chord::MAJOR_DIM_INTERVALS, &intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                &Chord::MAJOR_DIM_INTERVALS.to_vec(),
                ChordType::triad {
                    t: Triad::major_diminished,
                },
            ));
        } else if Chord::all_contained_in(&Chord::MINOR_DIM_INTERVALS, &intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                &Chord::MINOR_DIM_INTERVALS.to_vec(),
                ChordType::triad {
                    t: Triad::minor_diminished,
                },
            ));
        } else {
            return None;
        }
    }
    fn match_triad_extension(partial_chord: &Chord, intervals: &Vec<i32>) -> Option<Chord> {
        let mut new_chord = partial_chord.clone(); // lazy workaround
        if let ChordType::triad { t } = &partial_chord.type_ {
            if Chord::all_contained_in(&[Chord::MINOR_TWO_INTERVAL], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: Extension::add_minor_2,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::MINOR_TWO_INTERVAL,
                });
                return Some(new_chord);
            } else if Chord::all_contained_in(&[Chord::MAJOR_TWO_INTERVAL], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: Extension::add_major_2,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::MAJOR_TWO_INTERVAL,
                });
                return Some(new_chord);
            }else if Chord::all_contained_in(&[Chord::MAJOR_7_INTERVAL], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: Extension::major_7,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::MAJOR_7_INTERVAL,
                });
                return Some(new_chord);
            } else if Chord::all_contained_in(&[Chord::MINOR_7_INTERVAL], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: Extension::minor_7,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::MINOR_7_INTERVAL,
                });
                return Some(new_chord);
            } else if Chord::all_contained_in(&[Chord::MINOR_NINTH_INTERVAL], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: Extension::add_minor_9,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::MINOR_NINTH_INTERVAL,
                });
                return Some(new_chord);
            } else if Chord::all_contained_in(&[Chord::MAJOR_NINTH_INTERVAL], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: Extension::add_major_9,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::MAJOR_NINTH_INTERVAL,
                });
                return Some(new_chord);
            } else if Chord::all_contained_in(&[Chord::ELEVENTH_INTERVAL], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: Extension::add_11,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::ELEVENTH_INTERVAL,
                });
                return Some(new_chord);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn find_chord(notes: &Vec<Note>) -> Option<Chord> {
        for root in notes {
            let mut steps_to_root = vec![notes
                .iter()
                .map(|s| (*s - *root))
                .unique_by(|s1| s1.no_octaves()) // delete duplicate notes
                .collect::<Vec<_>>()];
            let mut inversions = Chord::build_inversions(&steps_to_root[0]);
            steps_to_root.append(&mut inversions);

            for inversion in &mut steps_to_root {
                inversion.sort();
                match inversion.len() as i32 {
                    2 => { /* Probably a POWER CHORD */
                        let twotone = inversion
                            .iter()
                            .map(|s| s.no_octaves().semitones)
                            .collect::<Vec<_>>();
                        match twotone.try_into().unwrap() {
                            Chord::FIVE_INTERVALS => {
                                return Some(Chord::from_intervals(
                                    root.semitones,
                                    &Chord::FIVE_INTERVALS.to_vec(),
                                    ChordType::twotone { t: TwoTone::five },
                                ));
                            }
                            _ => {}
                        }
                    }
                    3 => { /* TRIAD CHORD */
                        let triad = inversion
                            .iter()
                            .map(|s| s.no_octaves().semitones)
                            .collect::<Vec<_>>();
                        let opt_chord = Chord::match_triad(root, &triad);
                        if let Some(_) = opt_chord {
                            return opt_chord;
                        }
                    }
                    4 => {/* TRIAD CHORD + some other tone */
                        let intervals = inversion
                            .iter()
                            .map(|s| s.no_octaves().semitones)
                            .collect::<Vec<_>>();

                        let opt_chord = Chord::match_triad(root, &intervals);
                        if let Some(partial_chord) = opt_chord {
                            // in order to be able to differentiate a 2 from a 9
                            // we need to keep octave information
                            let full_intervals =
                                inversion.iter().map(|s| s.semitones).collect::<Vec<_>>();
                            if let Some(extended_triad) =
                                Chord::match_triad_extension(&partial_chord, &full_intervals)
                            {
                                return Some(extended_triad);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        return None;
    }
}
#[test]
fn test_first_inversion() {
    let intervals = Chord::MINOR_INTERVALS
        .iter()
        .map(|s| Note { semitones: *s })
        .collect::<Vec<_>>();
    let minor_inversions = Chord::build_inversions(&intervals);
    assert_eq!(
        minor_inversions[0],
        vec![5, 0, 3]
            .iter()
            .map(|s| Note { semitones: *s })
            .collect::<Vec<_>>()
    );
    assert_eq!(
        minor_inversions[1],
        vec![9, 5, 0]
            .iter()
            .map(|s| Note { semitones: *s })
            .collect::<Vec<_>>()
    );
    assert_eq!(minor_inversions.len(), 2);
}
#[test]
fn test_find_chord() {
    let notes1 = Chord::MINOR_INTERVALS
        .iter()
        .map(|s| Note { semitones: *s + 3 })
        .collect::<Vec<_>>();
    let notes2 = Chord::SUS_2_INTERVALS
        .iter()
        .map(|s| Note { semitones: *s + 3 })
        .collect::<Vec<_>>();
    let notes3 = vec![0, 4, 7, 10]
        .iter()
        .map(|s| Note { semitones: *s + 12 })
        .collect::<Vec<_>>();
    let notes4 = vec![0, 3, 7, 14]
        .iter()
        .map(|s| Note { semitones: *s + 0 })
        .collect::<Vec<_>>();
    let notes5 = vec![0, 5]
        .iter()
        .map(|s| Note { semitones: *s + 0 })
        .collect::<Vec<_>>();

    let chord1 = Chord::find_chord(&notes1).unwrap();
    let chord2 = Chord::find_chord(&notes2).unwrap();
    let chord3 = Chord::find_chord(&notes3).unwrap();
    let chord4 = Chord::find_chord(&notes4).unwrap();
    let chord5 = Chord::find_chord(&notes5).unwrap();
    assert_eq!(chord1.type_, ChordType::triad { t: Triad::minor });
    assert_eq!(chord2.type_, ChordType::triad { t: Triad::sus2 });
    assert_eq!(
        chord3.type_,
        ChordType::extended_triad {
            t: Triad::major,
            e: Extension::minor_7
        }
    );
    assert_eq!(
        chord4.type_,
        ChordType::extended_triad {
            t: Triad::minor,
            e: Extension::add_major_9
        }
    );
    assert_eq!(chord5.type_, ChordType::twotone { t: TwoTone::five });
}
