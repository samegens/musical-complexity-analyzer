# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Music Complexity Analyzer - A Rust workspace that analyzes musical complexity from MusicXML files. The project calculates various metrics like note density, pitch diversity, and other complexity measures for musical pieces.

## Architecture

### Workspace Structure

- **src/crates/musicxml-analysis**: Core analysis library containing:
  - `analysis/`: Metrics calculation modules (density, diversity)  
  - `extraction/`: MusicXML parsing and data extraction
  - `model/`: Data structures (pitch, time signatures, measure data)
  - `statistics/`: Statistical utilities (correlation analysis)
- **src/apps/musicxml-analyzer**: Main CLI application that generates complexity analysis and charts
- **src/tools/dump-musicxml-dom**: Utility for debugging MusicXML parsing

### Key Dependencies

- `musicxml`: MusicXML parsing library (version 1.1.2)
- `plotly` & `plotters`: Chart generation libraries
- `rstest`: Testing framework
- `assert_float_eq`: Floating point assertion utilities

## Common Commands

### Build and Test

```bash
# Build entire workspace
cargo build --verbose

# Run all tests
cargo test --verbose

# Run tests for specific crate
cargo test -p musicxml-analysis
```

### Linting and Code Quality

```bash
# Run lint checks (format + clippy)
./run-lint.sh

# Manual linting
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings -A dead_code -A clippy::module-inception
```

### Coverage

```bash
# Generate coverage report (requires cargo-llvm-cov)
./run-coverage.sh

# Coverage fails if below 90% line coverage
```

### Run Analysis

```bash
# Analyze single MusicXML file
cargo run --bin musicxml-analyzer path/to/file.musicxml

# Analyze directory with custom output
cargo run --bin musicxml-analyzer -- --output-dir ./output path/to/directory/
```

## System Dependencies

### Linux Requirements

- `libfontconfig1-dev`: Font configuration library
- `pkg-config`: Package configuration tool

Install with: `sudo apt-get install -y libfontconfig1-dev pkg-config`

## Development Notes

### Testing

- Uses `rstest` for parameterized tests
- Test files located in `test-files/` directory
- Coverage target: 90% line coverage minimum
- **Unit tests required**: Write unit tests for all new functionality using AAA structure (Arrange, Act, Assert)
- **Start simple**: Begin with minimal implementation and unit tests, then iterate

### Development Approach

- **Start small**: Implement features incrementally with the simplest version first
- **Test-driven**: Write unit tests alongside new functionality, not as an afterthought
- **AAA test structure**: Organize tests with clear Arrange, Act, Assert sections

### Code Style

- Standard Rust formatting with `cargo fmt`
- Clippy lints enforced with warnings as errors
- Dead code warnings suppressed (-A dead_code)
- Module inception lint suppressed (-A clippy::module-inception)

### CI/CD

- GitHub Actions workflow in `.github/workflows/ci.yml`
- Runs on Ubuntu with Rust stable toolchain
- Includes lint, build, test, and coverage steps
- Uploads coverage to Codecov

## Current Feature Development

### Piano Key Distance Metric (In Progress)

Currently implementing average/maximum piano key distance analysis as a new complexity metric.

**Goal**: Measure hand movement difficulty by tracking the distance between consecutive keys played by each hand.

**Implementation Plan**:

1. **Data Collection**: Extend `PieceData` struct to store upper keys played per hand per measure

   - Track sequence of upper keys for left hand per measure
   - Track sequence of upper keys for right hand per measure
   - Store as measure-indexed data structure

2. **Analysis Phase**: Calculate distance metrics from collected key sequences

   - Compute distance in half-steps between consecutive keys for each hand
   - Calculate average distance per hand
   - Calculate maximum distance per hand
   - Aggregate across all measures

**Current Status**: Need to modify data extraction and analysis modules to support per-measure, per-hand key tracking.

**Note**: This metric focuses on upper notes only and measures half-step distances. It's a simplified approach that doesn't distinguish between half-steps vs full-steps difficulty.

## Instructions for Future Claude Instances

When working on features or making significant discoveries about the codebase:

1. **Update Current Feature Development section** with your progress and any important insights
2. **Add new architectural insights** to the Architecture section if you discover important patterns
3. **Update commands** if you find or create new useful development commands
4. **Keep it concise** - focus on information that would save future instances significant time
5. **Remove completed features** from "Current Feature Development" once they're done
6. **Archive completed work context** - move detailed implementation notes to comments in code rather than CLAUDE.md

**What to add**: Key insights, gotchas, important patterns, useful commands, architectural decisions

**What to avoid**: Detailed implementation specifics, obvious information, temporary debugging notes
