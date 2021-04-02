# Guitarnotes
Command line program that helps to visualise guitar chords, notes, and scales.

## Print arbitrary scales or modes for 12 different tunings
With the `--scale` or `-s` option, `guitarnotes` will print a scale or mode to the command line and output the chords in this scale (first line of ouput).

`Syntax: guitarnotes -s <root> <scale_name> -t <tuning_name>`

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

## Find the notes and the name of the corresponding chord (incl. inversions) from a fret constellation
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
