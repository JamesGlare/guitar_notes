[![Build Status](https://travis-ci.com/JamesGlare/guitar_notes.svg?branch=main)](https://travis-ci.com/JamesGlare/guitar_notes)
# Guitarnotes 
Command line program that helps to visualise or identify guitar chords, notes, and scales.

## Print arbitrary scales or modes for 12 different tunings
With the `--scale` or `-s` option, `guitarnotes` will print a scale or mode to the command line and output the chords in this scale (first line of ouput).

`Syntax: guitarnotes -s <root> <scale_name> -t <tuning_name>`

The scale_name can be one of `minor, major, minor_blues, major_blues, minor_pentatonic, major_pentatonic, dorian, phrygian, lydian, mixolydian`.

*Example with default tuning*:

$guitarnotes --scale g major
```
G       Am      Bm      C       D       Em      F#dim   G
1       2       3       4       5       6       7       8

      1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16  17  18  19  20  21  22  23

 e |  -   f#  G   -   a   -   b   c   -   d   -   e   -   f#  G   -   a   -   b   c   -   d   -
 b |  c   -   d   -   e   -   f#  G   -   a   -   b   c   -   d   -   e   -   f#  G   -   a   -
 G |  -   a   -   b   c   -   d   -   e   -   f#  G   -   a   -   b   c   -   d   -   e   -   f#
 d |  -   e   -   f#  G   -   a   -   b   c   -   d   -   e   -   f#  G   -   a   -   b   c   -
 a |  -   b   c   -   d   -   e   -   f#  G   -   a   -   b   c   -   d   -   e   -   f#  G   -
 e |  -   f#  G   -   a   -   b   c   -   d   -   e   -   f#  G   -   a   -   b   c   -   d   -

              *       *       *       *           :           *       *       *       *       *
```

*Example with default tuning*:

$guitarnotes -s c dorian

```
Cm      Dm      D#      F       Gm      Adim    A#      Cm
1       2       3b      4       5       6       7b      8

      1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16  17  18  19  20  21  22  23

   |  f   -   g   -   a   a#  -   C   -   d   d#  -   f   -   g   -   a   a#  -   C   -   d   d#
   |  C   -   d   d#  -   f   -   g   -   a   a#  -   C   -   d   d#  -   f   -   g   -   a   a#
 g |  -   a   a#  -   C   -   d   d#  -   f   -   g   -   a   a#  -   C   -   d   d#  -   f   -
 d |  d#  -   f   -   g   -   a   a#  -   C   -   d   d#  -   f   -   g   -   a   a#  -   C   -
 a |  a#  -   C   -   d   d#  -   f   -   g   -   a   a#  -   C   -   d   d#  -   f   -   g   -
   |  f   -   g   -   a   a#  -   C   -   d   d#  -   f   -   g   -   a   a#  -   C   -   d   d#

              *       *       *       *           :           *       *       *       *       *
```

*Example with dropD tuning*:

$guitarnotes -s a minor_blues -t dropd
```
a       c       d       d#      e       g       a
1       3b      4       5b      5       7b      8

      1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16  17  18  19  20  21  22  23

 e |  -   -   g   -   A   -   -   c   -   d   d#  e   -   -   g   -   A   -   -   c   -   d   d#
   |  c   -   d   d#  e   -   -   g   -   A   -   -   c   -   d   d#  e   -   -   g   -   A   -
 g |  -   A   -   -   c   -   d   d#  e   -   -   g   -   A   -   -   c   -   d   d#  e   -   -
 d |  d#  e   -   -   g   -   A   -   -   c   -   d   d#  e   -   -   g   -   A   -   -   c   -
 A |  -   -   c   -   d   d#  e   -   -   g   -   A   -   -   c   -   d   d#  e   -   -   g   -
 d |  d#  e   -   -   g   -   A   -   -   c   -   d   d#  e   -   -   g   -   A   -   -   c   -

              *       *       *       *           :           *       *       *       *       *
```

*Example of the relative-semitone view*:
Note the ```-r``` option.

$guitarnotes -s A mixolydian -t dropD -r
```
A       Bm      C#dim   D       Em      F#m     G       A
1       2       3       4       5       6       7b      8

      1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16  17  18  19  20  21  22  23

 5 |  -   6   7b  -   1   -   2   -   3   4   -   5   -   6   7b  -   1   -   2   -   3   4   -
 2 |  -   3   4   -   5   -   6   7b  -   1   -   2   -   3   4   -   5   -   6   7b  -   1   -
 7b|  -   1   -   2   -   3   4   -   5   -   6   7b  -   1   -   2   -   3   4   -   5   -   6
 4 |  -   5   -   6   7b  -   1   -   2   -   3   4   -   5   -   6   7b  -   1   -   2   -   3
 1 |  -   2   -   3   4   -   5   -   6   7b  -   1   -   2   -   3   4   -   5   -   6   7b  -
 5 |  -   4   -   3b  2   -   1   -   2   -   3   4   -   5   -   6   7b  -   1   -   2   -   3

              *       *       *       *           :           *       *       *       *       *
```

## Find the notes and the name of the corresponding chord (incl. inversions) from a fret constellation
With `--note` or `-n` option and up to six fret positions (e.g. `A3` or `D12`), guitarnotes will return the corresponding notes as well as the chord, and its inversions.
This is useful for quick lookup of notes that correspond to fret positions or to identify chords and their inversions.

`Syntax: guitarnotes -n <fret_positions> -t <tuning_name>`

*Example (A-minor-7 chord) with standard tuning*:

$guitarnotes -n a0 d2 g0 b1 e3
```
Notes: A E G C G
Chord: Am7
3. inversion: Cadd6
      1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16  17  18  19  20  21  22  23

 e |  -   -   g   -   A   -   -   c   -   -   -   e   -   -   g   -   A   -   -   c   -   -   -
   |  c   -   -   -   e   -   -   g   -   A   -   -   c   -   -   -   e   -   -   g   -   A   -
 g |  -   A   -   -   c   -   -   -   e   -   -   g   -   A   -   -   c   -   -   -   e   -   -
   |  -   e   -   -   g   -   A   -   -   c   -   -   -   e   -   -   g   -   A   -   -   c   -
 A |  -   -   c   -   -   -   e   -   -   g   -   A   -   -   c   -   -   -   e   -   -   g   -
 e |  -   -   g   -   A   -   -   c   -   -   -   e   -   -   g   -   A   -   -   c   -   -   -

              *       *       *       *           :           *       *       *       *       *
```

*Example (D-sus2 chord) in dgc-dcg tuning*:

$guitarnotes -n d0 g2 c4 d0  -t dgcdcg
```
Notes: D A E D
Chord: Dsus2
1. inversion: Asus4
      1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16  17  18  19  20  21  22  23

 D |  -   e   -   -   -   -   a   -   -   -   -   D   -   e   -   -   -   -   a   -   -   -   -
   |  -   D   -   e   -   -   -   -   a   -   -   -   -   D   -   e   -   -   -   -   a   -   -
   |  -   a   -   -   -   -   D   -   e   -   -   -   -   a   -   -   -   -   D   -   e   -   -
   |  -   D   -   e   -   -   -   -   a   -   -   -   -   D   -   e   -   -   -   -   a   -   -
   |  -   a   -   -   -   -   D   -   e   -   -   -   -   a   -   -   -   -   D   -   e   -   -
 D |  -   e   -   -   -   -   a   -   -   -   -   D   -   e   -   -   -   -   a   -   -   -   -

              *       *       *       *           :           *       *       *       *       *
```

## Print all positions of a particular note
The `--all` or `-a` option will print all positions of a particular note (useful for note training) on the fretboard.
Again, this option can be combined with the 12 supported tunings.

*Example in default tuning:* 

$guitarnotes --all c#
```
      1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16  17  18  19  20  21  22  23

   |  -   -   -   -   -   -   -   -   C#  -   -   -   -   -   -   -   -   -   -   -   C#  -   -
   |  -   C#  -   -   -   -   -   -   -   -   -   -   -   C#  -   -   -   -   -   -   -   -   -
   |  -   -   -   -   -   C#  -   -   -   -   -   -   -   -   -   -   -   C#  -   -   -   -   -
   |  -   -   -   -   -   -   -   -   -   -   C#  -   -   -   -   -   -   -   -   -   -   -   C#
   |  -   -   -   C#  -   -   -   -   -   -   -   -   -   -   -   C#  -   -   -   -   -   -   -
   |  -   -   -   -   -   -   -   -   C#  -   -   -   -   -   -   -   -   -   -   -   C#  -   -

              *       *       *       *           :           *       *       *       *       *
```
*Example in openC tuning:*

$guitarnotes --all c# --tuning openc
```
      1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16  17  18  19  20  21  22  23

   |  -   -   -   -   -   -   -   -   C#  -   -   -   -   -   -   -   -   -   -   -   C#  -   -
   |  C#  -   -   -   -   -   -   -   -   -   -   -   C#  -   -   -   -   -   -   -   -   -   -
   |  -   -   -   -   -   C#  -   -   -   -   -   -   -   -   -   -   -   C#  -   -   -   -   -
   |  C#  -   -   -   -   -   -   -   -   -   -   -   C#  -   -   -   -   -   -   -   -   -   -
   |  -   -   -   -   -   C#  -   -   -   -   -   -   -   -   -   -   -   C#  -   -   -   -   -
   |  C#  -   -   -   -   -   -   -   -   -   -   -   C#  -   -   -   -   -   -   -   -   -   -

              *       *       *       *           :           *       *       *       *       *
```

## Supported tunings:
```
eadgbe, dropd, doubledropd, dadgad, dgcdcg, openc6, eeeebe, opend, opene, openg, opena, openc
```
