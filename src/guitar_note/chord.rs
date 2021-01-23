extern crate enum_utils;
extern crate itertools;

use super::note::Note;
use itertools::Itertools;

/* Chords
 */
#[derive(Clone, enum_utils::FromStr, enum_utils::IterVariants, PartialEq, Debug)]
enum Triad {
    minor,
    major,
    sus2,
    sus4,
    minor_diminished,
    major_diminished,
}
#[derive(Clone, enum_utils::FromStr, enum_utils::IterVariants, PartialEq, Debug)]
enum ChordInterval {
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
    major_9,
    minor_9,
    perfect_11,
    augmented_11
}
#[derive(Clone, PartialEq, Debug)]
enum ChordType {
    twotone { t: ChordInterval },
    triad { t: Triad },
    extended_triad {t: Triad, e: ChordInterval},
    four_chord { t: Triad, e: ChordInterval },
    nine_chord { t: Triad, e1: ChordInterval, e2: ChordInterval },
    eleven_chord { t: Triad, e1: ChordInterval, e2: ChordInterval, e3: ChordInterval},
}
#[derive(Clone)]
pub struct Chord {
    type_: ChordType,
    notes: Vec<Note>,
}
impl Chord {
    const MINOR_2: i32 = 1;
    const MAJOR_2: i32 = 2;
    const MINOR_3: i32 = 3;
    const MAJOR_3: i32 = 4;
    const PERFECT_4: i32 = 5;
    const FLATTENED_5: i32 = 6;
    const PERFECT_5: i32 = 7;
    const MINOR_6: i32 = 8;
    const MAJOR_6: i32 = 9;
    const MINOR_7: i32 = 10;
    const MAJOR_7: i32 = 11;
    const MINOR_9: i32 = 13;
    const MAJOR_9: i32 = 14;
    const PERFECT_11: i32 = 17;
    const AUGMENTED_11: i32 = 18;

    fn build_inversions(intervals: &Vec<Note>) -> Vec<Vec<Note>> {
        let n_notes = intervals.len(); // n-1 inversion
        let mut result: Vec<Vec<Note>> = vec![];
        for i in 1..n_notes {
            let front = intervals.iter().take(i).map(|s| *s + Note::octave());
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
            ChordType::twotone { t } => format!("{:?}", t),
            ChordType::triad { t } => format!("{:?}", t),
            ChordType::extended_triad { t, e} => format!("{:?}add{:?}", t, e),
            ChordType::four_chord { t, e } => format!("{:?}-{:?}", t, e),
            ChordType::nine_chord { t, e1, e2} => format!("9{:?}", t),
            ChordType::eleven_chord { t, e1, e2, e3} => format!("11{:?}", t),
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
    fn match_twotone(root: &Note, intervals: &Vec<i32>) -> Option<Chord> {
        if Chord::all_contained_in(&[Chord::PERFECT_5], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::twotone { t: ChordInterval::perfect_5 },
            ));
        } else if Chord::all_contained_in(&[Chord::MAJOR_2], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::twotone {
                    t: ChordInterval::major_2,
                },
            ));
        } else if Chord::all_contained_in(&[Chord::MINOR_2], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::twotone {
                    t: ChordInterval::minor_2,
                },
            ));
        } else if Chord::all_contained_in(&[Chord::MINOR_3], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::twotone {
                    t: ChordInterval::minor_3,
                },
            ));
        } else if Chord::all_contained_in(&[Chord::MAJOR_3], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::twotone {
                    t: ChordInterval::major_3,
                },
            ));
        } else if Chord::all_contained_in(&[Chord::PERFECT_4], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::twotone {
                    t: ChordInterval::perfect_4,
                },
            ));
        } else if Chord::all_contained_in(&[Chord::MINOR_6], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::twotone {
                    t: ChordInterval::minor_6,
                },
            ));
        } else if Chord::all_contained_in(&[Chord::MINOR_7], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::twotone {
                    t: ChordInterval::minor_7,
                },
            ));
        } else if Chord::all_contained_in(&[Chord::MAJOR_7], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::twotone {
                    t: ChordInterval::major_7,
                },
            ));
        } else if Chord::all_contained_in(&[Chord::MINOR_9], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::twotone {
                    t: ChordInterval::minor_9,
                },
            ));
        } else if Chord::all_contained_in(&[Chord::MAJOR_9], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::twotone {
                    t: ChordInterval::major_9,
                },
            ));
        } else if Chord::all_contained_in(&[Chord::PERFECT_11], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::twotone { t: ChordInterval::perfect_11 },
            ));
        } else if Chord::all_contained_in(&[Chord::AUGMENTED_11], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::twotone { t: ChordInterval::augmented_11 },
            ));
        } else {
            return None;
        }
    }
    fn match_triad(root: &Note, intervals: &Vec<i32>) -> Option<Chord> {
        if Chord::all_contained_in(&[Chord::MINOR_3, Chord::PERFECT_5], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::triad { t: Triad::minor },
            ));
        } else if Chord::all_contained_in(&[Chord::MAJOR_3, Chord::PERFECT_5], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::triad { t: Triad::major },
            ));
        } else if Chord::all_contained_in(&[Chord::MAJOR_2, Chord::PERFECT_5], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::triad { t: Triad::sus2 },
            ));
        } else if Chord::all_contained_in(&[Chord::PERFECT_4, Chord::PERFECT_5], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::triad { t: Triad::sus4 },
            ));
        } else if Chord::all_contained_in(&[Chord::MINOR_3, Chord::FLATTENED_5], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::triad {
                    t: Triad::minor_diminished,
                },
            ));
        } else if Chord::all_contained_in(&[Chord::MAJOR_3, Chord::FLATTENED_5], intervals) {
            return Some(Chord::from_intervals(
                root.semitones,
                intervals,
                ChordType::triad {
                    t: Triad::major_diminished,
                },
            ));
        } else {
            return None;
        }
    }
    fn match_triad_extension(partial_chord: &Chord, intervals: &Vec<i32>) -> Option<Chord> {
        let mut new_chord = partial_chord.clone(); // lazy workaround
        if let ChordType::triad { t } = &partial_chord.type_ {
            if Chord::all_contained_in(&[Chord::MINOR_2], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::minor_2,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::MINOR_2,
                });
                return Some(new_chord);
            } else if Chord::all_contained_in(&[Chord::MAJOR_2], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::major_2,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::MAJOR_2,
                });
                return Some(new_chord);
            } else if Chord::all_contained_in(&[Chord::MINOR_6], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::minor_6,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::MINOR_6,
                });
                return Some(new_chord);
            } else if Chord::all_contained_in(&[Chord::MAJOR_6], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::major_6,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::MAJOR_6,
                });
                return Some(new_chord);
            } else if Chord::all_contained_in(&[Chord::MAJOR_7], &intervals) {
                new_chord.type_ = ChordType::four_chord {
                    t: t.clone(),
                    e: ChordInterval::major_7,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::MAJOR_7,
                });
                return Some(new_chord);
            } else if Chord::all_contained_in(&[Chord::MINOR_7], &intervals) {
                new_chord.type_ = ChordType::four_chord {
                    t: t.clone(),
                    e: ChordInterval::minor_7,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::MINOR_7,
                });
                return Some(new_chord);
            } else if Chord::all_contained_in(&[Chord::MINOR_9], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::minor_9,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::MINOR_9,
                });
                return Some(new_chord);
            } else if Chord::all_contained_in(&[Chord::MAJOR_9], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::major_9,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::MAJOR_9,
                });
                return Some(new_chord);
            } else if Chord::all_contained_in(&[Chord::PERFECT_11], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::perfect_11,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::PERFECT_11,
                });
                return Some(new_chord);
            } else if Chord::all_contained_in(&[Chord::AUGMENTED_11], &intervals) {
                new_chord.type_ = ChordType::extended_triad {
                    t: t.clone(),
                    e: ChordInterval::augmented_11,
                };
                new_chord.notes.push(Note {
                    semitones: Chord::PERFECT_11,
                });
                return Some(new_chord);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn find_chord(notes: &Vec<Note>) -> Vec<Option<Chord>> {
        let mut results: Vec<Option<Chord>> = Vec::new();
        let mut steps_to_root = vec![notes
            .iter()
            .map(|s| *s)
            .unique_by(|s1| s1.no_octaves()) // delete duplicate notes
            .collect::<Vec<_>>()];
        let mut inversions = Chord::build_inversions(&steps_to_root[0]);

        steps_to_root.append(&mut inversions);

        for inversion in &mut steps_to_root {
            let root = inversion.iter().min().unwrap();
            let relative = inversion.iter().map(|s| (*s - *root)).collect::<Vec<_>>();
            println!("{:?}", relative);
            match inversion.len() as i32 {
                2 => {
                    /* Probably a POWER CHORD */
                    let twotone = relative
                        .iter()
                        .map(|s| s.no_octaves().semitones)
                        .collect::<Vec<_>>();
                    let opt_chord = Chord::match_twotone(root, &twotone);
                    results.push(opt_chord);
                }
                3 => {
                    /* TRIAD CHORD */
                    let triad = relative
                        .iter()
                        .map(|s| s.no_octaves().semitones)
                        .collect::<Vec<_>>();
                    let opt_chord = Chord::match_triad(root, &triad);
                    results.push(opt_chord);
                }
                4 => {
                    /* TRIAD CHORD + some other tone */
                    let intervals = relative
                        .iter()
                        .map(|s| s.no_octaves().semitones)
                        .collect::<Vec<_>>();

                    let opt_chord = Chord::match_triad(root, &intervals);
                    if let Some(partial_chord) = opt_chord {
                        // in order to be able to differentiate, e.g. a 2 from a 9,
                        // we need to keep octave information
                        let full_intervals =
                            relative.iter().map(|s| s.semitones).collect::<Vec<_>>();
                        let opt_extended_triad =
                            Chord::match_triad_extension(&partial_chord, &full_intervals);
                        results.push(opt_extended_triad);
                    } else {
                        results.push(None);
                    }
                },
                5 => {

                },
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
    let intervals = [0, Chord::MINOR_3, Chord::PERFECT_5]
        .iter()
        .map(|s| Note { semitones: *s })
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
    let notes1 = [0, Chord::MINOR_3, Chord::PERFECT_5]
        .iter()
        .map(|s| Note { semitones: *s + 3 })
        .collect::<Vec<_>>();
    let notes2 = [0, Chord::MAJOR_2, Chord::PERFECT_5]
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
    let notes5 = vec![0, 7]
        .iter()
        .map(|s| Note { semitones: *s + 0 })
        .collect::<Vec<_>>();

    let chord1 = Chord::find_chord(&notes1);
    let chord2 = Chord::find_chord(&notes2);
    let chord3 = Chord::find_chord(&notes3);
    let chord4 = Chord::find_chord(&notes4);
    let chord5 = Chord::find_chord(&notes5);
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
        ChordType::twotone { t: ChordInterval::perfect_5 }
    );
}
