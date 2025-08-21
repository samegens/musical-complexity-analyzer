# Music Complexity Analyzer

[![CI](https://github.com/samegens/musical-complexity-analyzer/actions/workflows/ci.yml/badge.svg)](https://github.com/samegens/musical-complexity-analyzer/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/samegens/musical-complexity-analyzer/branch/main/graph/badge.svg)](https://codecov.io/gh/samegens/musical-complexity-analyzer)

This tool takes a MusicXML file and determines the musical complexity.

## Prerequisites

### Linux

Packages for musicxml-analyzer:

- libfontconfig1-dev
- pkg-config

## Metrics

### Note density

Notes per second. Separate average and peak. For now, discard tracks where peak is significantly higher than average.
Later mark these tracks as 'challenge'.

#### Easiest

- [Twinkle twinkle](test-files/twinkle-little-star-60bpm.musicxml) at 60 BPM

#### Hardest

- [Prelude Op. 28 No. 16 - Chopin](https://musescore.com/classicman/scores/73000)
- [La Campanella - Liszt](test-files/La_Campanella_-_Grandes_Etudes_de_Paganini_No._3_-_Franz_Liszt.musicxml) ([original](https://github.com/musetrainer/library/blob/master/scores/La_Campanella_-_Grandes_Etudes_de_Paganini_No._3_-_Franz_Liszt.mxl))

### Note diversity

How many different notes a piece contains. We use notational diversity,
which means that enharmonically equivalent notes (like C-sharp and D-flat)
are counted as different pitches based on how they're written in the score.

- easiest: [Hot Cross Buns](test-files/hot_cross_buns.musicxml)
- hardest: [Furiant No. 1](test-files/Furiant_No1.musicxml) ([original](https://musescore.com/user/29416258/scores/14177191))

### Harmonic complexity

Chord structures, key changes, accidentals

### Hand Independence

Different rhythms/melodies between hands

### Rhythmic Variety

Note value diversity, syncopation, cross-rhythms

### Dynamic Range

Expression markings, articulation complexity

### Sight Reading Complexity

Ledger lines, clef changes, visual density

### Rhythmic Complexity

How challenging the rhythmic patterns are to execute

- easiest: 4/4
- hardest: complex changing rhythms hardest (The Dance of Eternity - Dream Theatre)
