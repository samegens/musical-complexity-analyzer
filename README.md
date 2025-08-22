# Music Complexity Analyzer

[![CI](https://github.com/samegens/musical-complexity-analyzer/actions/workflows/ci.yml/badge.svg)](https://github.com/samegens/musical-complexity-analyzer/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/samegens/musical-complexity-analyzer/branch/main/graph/badge.svg)](https://codecov.io/gh/samegens/musical-complexity-analyzer)

This tool takes a MusicXML file and determines the musical complexity.

## Prerequisites

### Linux

Packages for musicxml-analyzer:

- libfontconfig1-dev
- pkg-config

## Test set

I collected about 150 mxl and musicxml files from several sources like Musescore.
Mostly classical pieces, because they are (mostly) free to use while modern (pop) music has all kinds of limiting licensing.
It also contains some practice pieces, game music, traditional folk songs and some modern music.

This test set was not meant to be representitive of all existing music. Diversity was most important
to be able to check the orthogonality of my chosen metrics.

## Metrics

### Note density (done)

Notes per second. Separate average and peak. For now, discard tracks where peak is significantly higher than average.
Later mark these tracks as 'challenge'.

- easiest: [Twinkle twinkle](test-files/twinkle-little-star-60bpm.musicxml) at 60 BPM
- hardest: [Prelude Op. 28 No. 16 - Chopin](https://musescore.com/classicman/scores/73000) or
[La Campanella - Liszt](test-files/La_Campanella_-_Grandes_Etudes_de_Paganini_No._3_-_Franz_Liszt.musicxml) ([original](https://github.com/musetrainer/library/blob/master/scores/La_Campanella_-_Grandes_Etudes_de_Paganini_No._3_-_Franz_Liszt.mxl))

### Pitch diversity (done)

How many different notes a piece contains. We use notational diversity,
which means that enharmonically equivalent notes (like C-sharp and D-flat)
are counted as different pitches based on how they're written in the score.

- easiest: [Hot Cross Buns](test-files/hot_cross_buns.musicxml)
- hardest: [Furiant No. 1](test-files/Furiant_No1.musicxml) ([original](https://musescore.com/user/29416258/scores/14177191))

#### Note

I considered both pitch diversity and piano key diversity.
The correlation between the two is interesting: up to key diversity 40, the relation is almost completely linear.
Above 40 the correlation breaks down. This means that non-experts pitch diversity and piano key diversity are
exactly the same, while for experts the pitch diversity is more interesting. This is probably because
harder pieces that contain more unique pitches/keys, the actual notation will be harder.

So in the end I settled for pitch diversity.

### Note count (done, rejected)

There's too much correlation between note count and note density (r = 0.724) so I chose note density as
the more interesting metric.

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
