use std::cmp;
extern crate enum_utils;
extern crate itertools;
use super::note::Note;
use itertools::Itertools;

/* Chords
 */
#[derive(Clone, enum_utils::FromStr, enum_utils::IterVariants, PartialEq, Debug)]
enum TriadType {
    minor,
    minor_omitted_5,
    major,
    major_omitted_5,
    sus2,
    sus4,
    minor_diminished,
    major_diminished,
    plus,
}
impl TriadType {
    pub fn to_string(&self) -> &str {
        return match self {
            TriadType::minor | TriadType::minor_omitted_5 => "m",
            TriadType::major | TriadType::major_omitted_5 => "",
            TriadType::sus2 => "sus2",
            TriadType::sus4 => "sus4",
            TriadType::minor_diminished => "mdim",
            TriadType::major_diminished => "dim",
            TriadType::plus => "+",
        };
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Interval {
    note: Note,
}

impl Interval {
    const OMITTED: Interval = Interval {
        note: Note { semitones: 0 },
    };
    const MINOR_2: Interval = Interval {
        note: Note { semitones: 1 },
    };
    const MAJOR_2: Interval = Interval {
        note: Note { semitones: 2 },
    };
    const MINOR_3: Interval = Interval {
        note: Note { semitones: 3 },
    };
    const MAJOR_3: Interval = Interval {
        note: Note { semitones: 4 },
    };
    const PERFECT_4: Interval = Interval {
        note: Note { semitones: 5 },
    };
    const FLATTENED_5: Interval = Interval {
        note: Note { semitones: 6 },
    };
    const PERFECT_5: Interval = Interval {
        note: Note { semitones: 7 },
    };
    const AUGMENTED_5: Interval = Interval {
        note: Note { semitones: 8 },
    };
    const MINOR_6: Interval = Interval {
        note: Note { semitones: 8 },
    };
    const MAJOR_6: Interval = Interval {
        note: Note { semitones: 9 },
    };
    const MINOR_7: Interval = Interval {
        note: Note { semitones: 10 },
    };
    const MAJOR_7: Interval = Interval {
        note: Note { semitones: 11 },
    };
    const OCTAVE: Interval = Interval {
        note: Note { semitones: 12 },
    };
    const MINOR_9: Interval = Interval {
        note: Note { semitones: 13 },
    };
    const MAJOR_9: Interval = Interval {
        note: Note { semitones: 14 },
    };
    const PLUS_9: Interval = Interval {
        note: Note { semitones: 15 },
    };
    const FLATTENED_11: Interval = Interval {
        note: Note { semitones: 16 },
    };
    const PERFECT_11: Interval = Interval {
        note: Note { semitones: 17 },
    };
    const AUGMENTED_11: Interval = Interval {
        note: Note { semitones: 18 },
    };
    const LOWER: Interval = Interval {
        note: Note { semitones: 19 },
    };

    pub fn to_string(&self) -> &str {
        return match *self {
            Interval::OMITTED => "",
            Interval::MINOR_2 => "2m",
            Interval::MAJOR_2 => "2",
            Interval::MINOR_3 => "3m",
            Interval::MAJOR_3 => "3",
            Interval::PERFECT_4 => "4",
            Interval::FLATTENED_5 => "5-",
            Interval::PERFECT_5 => "5",
            Interval::AUGMENTED_5 => "5+",
            Interval::MINOR_6 => "6m",
            Interval::MAJOR_6 => "6",
            Interval::MINOR_7 => "7",
            /* for the seven, indicate the major as minor is default */
            Interval::MAJOR_7 => "7maj",
            Interval::MINOR_9 => "9-",
            Interval::MAJOR_9 => "9",
            Interval::PLUS_9 => "9+",
            Interval::FLATTENED_11 => "11-",
            Interval::PERFECT_11 => "11",
            Interval::AUGMENTED_11 => "11+",
            _ => "blub",
        };
    }
}
#[derive(Clone, PartialEq, Debug)]
enum ChordType {
    UNKNOWN,
    TwoTone {
        t: Interval,
    },
    Triad {
        t: TriadType,
    },
    AddChord {
        t: TriadType,
        e: Interval,
    },
    SevenChord {
        t: TriadType,
        e: Interval,
    },
    NineChord {
        t: TriadType,
        e1: Interval,
        e2: Interval,
    },
    ElevenChord {
        t: TriadType,
        e1: Interval,
        e2: Interval,
        e3: Interval,
    },
}
#[derive(Clone)]
pub struct Chord {
    type_: ChordType,
    notes: Vec<Note>,
}
impl Chord {
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
            ChordType::UNKNOWN => format!("Unknown"),
            ChordType::TwoTone { t } => format!("{}", t.to_string()),
            ChordType::Triad { t } => format!("{}", t.to_string()),
            ChordType::AddChord { t, e } => format!("{}add{}", t.to_string(), e.to_string()),
            ChordType::SevenChord { t, e } => format!("{}{}", t.to_string(), e.to_string()),
            ChordType::NineChord { t, e1, e2 } => format!("{}{}", t.to_string(), e2.to_string()),
            ChordType::ElevenChord { t, e1, e2, e3 } => format!("11{}", t.to_string()),
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

    fn match_chord(intervals: Vec<Note>) -> ChordType {
        /* Assumptions
         * 1. Intervals has been root-subtracted
         * 2. sorted
         * 3. made unique
         */
        let sequence = intervals
            .iter()
            .skip(1) // skip 0
            .map(|n| Interval { note: *n })
            .collect::<Vec<Interval>>();
        let mut candidate = ChordType::UNKNOWN;
        for interval in sequence {
            candidate = match interval {
                Interval::MINOR_2 => ChordType::TwoTone { t: interval },
                Interval::MAJOR_2 => match candidate {
                    ChordType::UNKNOWN => ChordType::TwoTone { t: interval },
                    _ => ChordType::UNKNOWN,
                },
                Interval::MINOR_3 => match candidate {
                    ChordType::UNKNOWN => ChordType::Triad {
                        t: TriadType::minor_omitted_5,
                    },
                    ChordType::TwoTone { t } => ChordType::AddChord {
                        t: TriadType::minor_omitted_5,
                        e: t,
                    },
                    _ => ChordType::UNKNOWN,
                },
                Interval::MAJOR_3 => match candidate {
                    ChordType::UNKNOWN => ChordType::Triad {
                        t: TriadType::major_omitted_5,
                    },
                    ChordType::TwoTone { t } => ChordType::AddChord {
                        t: TriadType::major_omitted_5,
                        e: t,
                    },
                    _ => ChordType::UNKNOWN,
                },
                Interval::PERFECT_4 => match candidate {
                    ChordType::UNKNOWN => ChordType::TwoTone { t: interval },
                    ChordType::Triad { t } => ChordType::AddChord { t: t, e: interval },
                    _ => ChordType::UNKNOWN,
                },
                Interval::FLATTENED_5 => match candidate {
                    ChordType::UNKNOWN => ChordType::TwoTone { t: interval },
                    ChordType::Triad {
                        t: TriadType::minor_omitted_5,
                    } => ChordType::Triad {
                        t: TriadType::minor_diminished,
                    },
                    ChordType::Triad {
                        t: TriadType::major_omitted_5,
                    } => ChordType::Triad {
                        t: TriadType::major_diminished,
                    },
                    _ => ChordType::UNKNOWN,
                },
                Interval::PERFECT_5 => match candidate {
                    ChordType::UNKNOWN => ChordType::TwoTone { t: interval },
                    ChordType::Triad {
                        t: TriadType::minor_omitted_5,
                    } => ChordType::Triad {
                        t: TriadType::minor,
                    },
                    ChordType::Triad {
                        t: TriadType::major_omitted_5,
                    } => ChordType::Triad {
                        t: TriadType::major,
                    },
                    ChordType::AddChord {
                        t: TriadType::minor_omitted_5,
                        e,
                    } => ChordType::AddChord {
                        t: TriadType::minor,
                        e,
                    },
                    ChordType::AddChord {
                        t: TriadType::major_omitted_5,
                        e,
                    } => ChordType::AddChord {
                        t: TriadType::major,
                        e,
                    },
                    ChordType::TwoTone {
                        t: Interval::MAJOR_2,
                    } => ChordType::Triad { t: TriadType::sus2 },
                    ChordType::TwoTone {
                        t: Interval::PERFECT_4,
                    } => ChordType::Triad { t: TriadType::sus4 },
                    _ => ChordType::UNKNOWN,
                },
                Interval::AUGMENTED_5 => match candidate {
                    ChordType::UNKNOWN => ChordType::TwoTone { t: interval },
                    ChordType::Triad {
                        t: TriadType::major_omitted_5,
                    } => ChordType::Triad { t: TriadType::plus },
                    _ => ChordType::UNKNOWN,
                },
                Interval::MINOR_6 | Interval::MAJOR_6 => match candidate {
                    ChordType::UNKNOWN => ChordType::TwoTone { t: interval },
                    ChordType::Triad { t } => ChordType::AddChord { t: t, e: interval },
                    _ => ChordType::UNKNOWN,
                },
                Interval::MINOR_7 | Interval::MAJOR_7 => match candidate {
                    ChordType::UNKNOWN => ChordType::TwoTone { t: interval },
                    ChordType::Triad { t } => ChordType::SevenChord { t: t, e: interval },
                    // disallow sus27 or sus47 ? explicitly disallow dim maj7 ?
                    _ => ChordType::UNKNOWN,
                },
                Interval::MINOR_9 | Interval::MAJOR_9 => match candidate {
                    ChordType::UNKNOWN => ChordType::TwoTone { t: interval },
                    ChordType::Triad { t } => ChordType::AddChord { t: t, e: interval },
                    ChordType::SevenChord { t, e } => ChordType::NineChord {
                        t,
                        e1: e,
                        e2: interval,
                    },
                    _ => ChordType::UNKNOWN,
                },
                Interval::PLUS_9 => match candidate {
                    ChordType::UNKNOWN => ChordType::TwoTone { t: interval },
                    ChordType::Triad { t } => ChordType::AddChord { t: t, e: interval },
                    ChordType::SevenChord { t, e } => ChordType::NineChord {
                        t,
                        e1: e,
                        e2: interval,
                    },
                    _ => ChordType::UNKNOWN,
                },
                Interval::PERFECT_11 => match candidate {
                    ChordType::UNKNOWN => ChordType::TwoTone { t: interval },
                    ChordType::Triad { t } => ChordType::AddChord { t: t, e: interval },
                    ChordType::SevenChord { t, e } => ChordType::ElevenChord {
                        t,
                        e1: e,
                        e2: Interval::OMITTED,
                        e3: interval,
                    },
                    ChordType::NineChord { t, e1, e2 } => ChordType::ElevenChord {
                        t,
                        e1: e1,
                        e2: e2,
                        e3: interval,
                    },
                    _ => ChordType::UNKNOWN,
                },
                _ => ChordType::UNKNOWN, //
            };
            // no transition out of unknown
            if candidate == ChordType::UNKNOWN {
                break;
            }
        }
        return candidate;
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
            let mut intervals = inversion
                .iter()
                .map(|s| {
                    let diff = *s - *root;
                    if diff > Interval::AUGMENTED_11.note {
                        diff.no_octaves()
                    } else {
                        diff
                    }
                })
                .collect::<Vec<_>>();
            intervals.sort(); //sort again - partial no octaves may have upended order
            let mut type_ = Chord::match_chord(intervals.clone());
            if type_ == ChordType::UNKNOWN {
                intervals = intervals.into_iter().map(|n| n.no_octaves()).collect();
                intervals.sort(); //sort again - no octaves may have upended order
                type_ = Chord::match_chord(intervals.clone());
            }
            match type_ {
                ChordType::UNKNOWN => {
                    results.push(None);
                }
                _ => {
                    results.push(Some(Chord::from_intervals(root.clone(), &intervals, type_)));
                }
            }
        }
        return results;
    }
}
#[test]
fn test_first_inversion() {
    let intervals = [
        Note { semitones: 0 },
        Interval::MINOR_3.note,
        Interval::PERFECT_5.note,
    ]
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
    let notes1 = [
        Note { semitones: 0 },
        Interval::MINOR_3.note,
        Interval::PERFECT_5.note,
    ]
    .iter()
    .map(|s| *s + Note { semitones: 3 })
    .collect::<Vec<_>>();
    let notes2 = [
        Note { semitones: 0 },
        Interval::MAJOR_2.note,
        Interval::PERFECT_5.note,
    ]
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
    let notes11 = vec![0, 7, 10, 15]
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
    let chord11 = Chord::find_chord(&notes11);

    assert_eq!(
        chord1[0].clone().unwrap().type_,
        ChordType::Triad {
            t: TriadType::minor
        }
    );
    assert_eq!(
        chord2[0].clone().unwrap().type_,
        ChordType::Triad { t: TriadType::sus2 }
    );
    assert_eq!(
        chord3[0].clone().unwrap().type_,
        ChordType::SevenChord {
            t: TriadType::major,
            e: Interval::MINOR_7
        }
    );
    assert_eq!(
        chord4[0].clone().unwrap().type_,
        ChordType::AddChord {
            t: TriadType::minor,
            e: Interval::MAJOR_9
        }
    );
    assert_eq!(
        chord5[0].clone().unwrap().type_,
        ChordType::TwoTone {
            t: Interval::PERFECT_5
        }
    );
    assert_eq!(
        chord6[0].clone().unwrap().type_,
        ChordType::AddChord {
            t: TriadType::major,
            e: Interval::MAJOR_6
        }
    );
    assert_eq!(
        chord7[0].clone().unwrap().type_,
        ChordType::Triad { t: TriadType::plus }
    );
    assert_eq!(
        chord8[2].clone().unwrap().type_,
        ChordType::Triad {
            t: TriadType::major
        }
    );
    assert_eq!(
        chord9[1].clone().unwrap().type_,
        ChordType::Triad {
            t: TriadType::major
        }
    );
    assert_eq!(
        chord10[0].clone().unwrap().type_,
        ChordType::SevenChord {
            t: TriadType::major_omitted_5,
            e: Interval::MINOR_7
        }
    );
    assert_eq!(
        chord11[0].clone().unwrap().type_,
        ChordType::SevenChord {
            t: TriadType::minor,
            e: Interval::MINOR_7
        }
    );
}
