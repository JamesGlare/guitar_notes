extern crate enum_utils;
extern crate itertools;

use super::note::Note;
/* Scales
 */
#[derive(Clone, enum_utils::FromStr, enum_utils::IterVariants)]
pub enum ScaleType {
    minor,
    major,
    minor_blues,
    major_blues,
    minor_pentatonic,
    major_pentatonic,
}

pub struct Scale {
    notes: Vec<Note>,
    scale_type: ScaleType,
}

impl Scale {
    const MAJOR_INTERVALS: [i32; 8] = [0, 2, 2, 1, 2, 2, 2, 1];
    const MINOR_INTERVALS: [i32; 8] = [0, 2, 1, 2, 2, 1, 2, 2];
    const MINOR_PENTATONIC_INTERVALS: [i32; 6] = [0, 3, 2, 2, 3, 2];
    const MAJOR_PENTATONIC_INTERVALS: [i32; 6] = [0, 2, 2, 3, 2, 3];
    const BLUES_MINOR_INTERVALS: [i32; 7] = [0, 3, 2, 1, 1, 3, 2];
    const BLUES_MAJOR_INTERVALS: [i32; 7] = [0, 2, 1, 1, 3, 2, 3];
    const DEGREE: [&'static str; 13] = [
        "1", "2b", "2", "3b", "3", "4", "5b", "5", "6b", "6", "7b", "7", "8",
    ];
    
    pub fn get_notes<'a>(&'a self) -> &'a Vec<Note> {
        return &self.notes;
    }

    fn from_intervals(root: i32, intervals: &[i32], scale_type: ScaleType) -> Scale {
        let notes = intervals
            .iter()
            .map(|x| *x)
            .scan(root, |state, n| {
                *state = *state + n;
                return Some(*state);
            })
            .map(|i| Note { semitones: i })
            .collect::<Vec<_>>();
        return Scale {
            notes: notes,
            scale_type: scale_type,
        };
    }

    pub fn from_type_and_root(root: Note, scale_type: ScaleType) -> Scale {
        return match scale_type {
            ScaleType::minor => {
                Scale::from_intervals(root.semitones, &Scale::MINOR_INTERVALS, scale_type)
            }
            ScaleType::major => {
                Scale::from_intervals(root.semitones, &Scale::MAJOR_INTERVALS, scale_type)
            }
            ScaleType::minor_pentatonic => Scale::from_intervals(
                root.semitones,
                &Scale::MINOR_PENTATONIC_INTERVALS,
                scale_type,
            ),
            ScaleType::major_pentatonic => Scale::from_intervals(
                root.semitones,
                &Scale::MAJOR_PENTATONIC_INTERVALS,
                scale_type,
            ),
            ScaleType::minor_blues => {
                Scale::from_intervals(root.semitones, &Scale::BLUES_MINOR_INTERVALS, scale_type)
            }
            ScaleType::major_blues => {
                Scale::from_intervals(root.semitones, &Scale::BLUES_MAJOR_INTERVALS, scale_type)
            }
        };
    }

    fn root(&self) -> Note {
        return self.notes[0].clone();
    }

    pub fn degrees_in_scale<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        let root_semitone = self.root().clone().semitones;
        return self
            .notes
            .iter()
            .map(move |i| Scale::DEGREE[((i.semitones - root_semitone) % 13) as usize]);
    }

    pub fn notes_in_scale<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        return self.notes.iter().map(|n| n.to_string());
    }
}
#[test]
fn test_notes_in_scale() {
    let root = Note { semitones: 4 };
    let scale = Scale::from_type_and_root(root, ScaleType::minor);
    let notes = scale.notes_in_scale().collect::<Vec<_>>();
    println!("{:?}", notes);
}
