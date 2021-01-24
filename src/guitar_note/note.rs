use std::ops;
#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub struct Note {
    pub semitones: i32,
}

impl Note {
    pub const NAMES: [&'static str; 12] = [
        "c", "c#", "d", "d#", "e", "f", "f#", "g", "g#", "a", "a#", "b",
    ];

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

    pub fn from_string(note_str: &str) -> Option<Note> {
        let opt_semitone = Note::NAMES.iter().position(|&i| note_str == i);
        return match opt_semitone {
            Some(semitone) => Some(Note {
                semitones: semitone as i32,
            }),
            None => None,
        };
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
    let mut sequence: Vec<Note> =  vec![5,3,7,1].iter().map(|s| Note{semitones:*s}).collect();
    sequence.sort();
    assert_eq!(sequence[0], Note{semitones: 1});
    assert_eq!(sequence[1], Note{semitones: 3});
    assert_eq!(sequence[2], Note{semitones: 5});
    assert_eq!(sequence[3], Note{semitones: 7});
}