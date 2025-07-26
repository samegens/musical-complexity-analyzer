# Music Complexity Analyzer

This tool takes a MusicXML file and determines the musical complexity.

## Metrics

### Note density

Notes per second. Separate average and peak. For now, discard tracks where peak is significantly higher than average.
Later mark these tracks as 'challenge'.

#### Easiest

- [Twinkle twinkle](test-files/twinkle-little-star-60bpm.musicxml) at 60 BPM

#### Hardest

- [Prelude Op. 28 No. 16 - Chopin](https://musescore.com/classicman/scores/73000)
- [La Campanella - Liszt](test-files/La_Campanella_-_Grandes_Etudes_de_Paganini_No._3_-_Franz_Liszt.musicxml) ([original](https://github.com/musetrainer/library/blob/master/scores/La_Campanella_-_Grandes_Etudes_de_Paganini_No._3_-_Franz_Liszt.mxl))

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

#### Easiest

4/4

#### Hardest

complex changing rhythms hardest (The Dance of Eternity - Dream Theatre)
