use std::ops;
#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub struct Note {
    pub semitones: i32,
}

impl Note {
    pub const NAMES: [&'static str; 12] = [
        "c", "c#", "d", "d#", "e", "f", "f#", "g", "g#", "a", "a#", "b",
    ];
    /// Try to parse a string to semitone
    ///
    pub fn from_string(name: &str) -> Option<Note> {
        match name {
            "c" => Some(Note { semitones: 0 }),
            "c#" | "db" => Some(Note { semitones: 1 }),
            "d" => Some(Note { semitones: 2 }),
            "d#" | "eb" => Some(Note { semitones: 3 }),
            "e" => Some(Note { semitones: 4 }),
            "f" => Some(Note { semitones: 5 }),
            "f#" | "gb" => Some(Note { semitones: 6 }),
            "g" => Some(Note { semitones: 7 }),
            "g#" | "ab" => Some(Note { semitones: 8 }),
            "a" => Some(Note { semitones: 9 }),
            "a#" | "bb" => Some(Note { semitones: 10 }),
            "b" => Some(Note { semitones: 11 }),
            _ => None,
        }
    }

    pub fn octave() -> Note {
        return Note { semitones: 12 };
    }

    pub fn no_octaves(self) -> Note {
        return Note {
            semitones: self.semitones % 12,
        };
    }
    pub fn regauge_if_negative(&self) -> Note {
        if self.semitones < 0 {
            return self.regauge();
        } else {
            return *self;
        }
    }
    pub fn regauge(&self) -> Note {
        return (Note::octave() - self.no_octaves()).no_octaves();
    }
    pub fn to_string(&self) -> &str {
        /* Returns string of raw note.
         */
        let st = (self.semitones % 12).abs();
        return Note::NAMES[st as usize];
    }
}

impl ops::Add<Note> for Note {
    type Output = Note;
    fn add(self, _rhs: Note) -> Note {
        Note {
            semitones: (self.semitones + _rhs.semitones),
        }
    }
}
impl ops::Sub<Note> for Note {
    type Output = Note;
    fn sub(self, _rhs: Note) -> Note {
        Note {
            semitones: (self.semitones - _rhs.semitones),
        }
    }
}
impl ops::Mul<Note> for i32 {
    type Output = Note;
    fn mul(self, _rhs: Note) -> Self::Output {
        Note {
            semitones: self * _rhs.semitones,
        }
    }
}

#[test]
fn test_order() {
    let mut sequence: Vec<Note> = vec![5, 3, 7, 1]
        .iter()
        .map(|s| Note { semitones: *s })
        .collect();
    sequence.sort();
    assert_eq!(sequence[0], Note { semitones: 1 });
    assert_eq!(sequence[1], Note { semitones: 3 });
    assert_eq!(sequence[2], Note { semitones: 5 });
    assert_eq!(sequence[3], Note { semitones: 7 });
}
