# Metronome-RS

<img src="image.png" alt="Metronome-RS Logo" width="200"/>


A high-performance, cross-platform metronome library written in Rust with Python bindings. Perfect for musicians, music software developers, and anyone needing precise timing and audio generation.

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.80+-orange.svg)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org)

## Demo video

View a short video [here](https://youtu.be/t-W9WpreLVg).

## Features

- **High-Performance Audio**: Built on CPAL for low-latency, cross-platform audio
- **Multiple Languages**: Native Rust API + Python bindings
- **Advanced Rhythms**: Support for subdivisions, accents, and complex time signatures
- **Customizable Sounds**: Multiple wave types (sine, square, triangle, sawtooth)
- **Cross-Platform**: Works on Linux, Windows, and macOS
- **Precision Timing**: Accurate BPM control for professional use
- **Flexible API**: From simple beeps to complex rhythmic patterns

## Quick Start

### Rust

Add to your `Cargo.toml`:
```toml
[dependencies]
metronome-rs = "1.0.0"
```

```rust
use metronome_rs::{start_simple_metronome, stop_global_metronome};
use std::{thread, time::Duration};

// Start a 120 BPM metronome
start_simple_metronome(120.0)?;

// Let it play for 5 seconds
thread::sleep(Duration::from_secs(5));

// Stop the metronome
stop_global_metronome();
```

### Python

```bash
pip install metronome-rs
```

```python
import metronome_rs
import time

# Start a 120 BPM metronome
metronome_rs.py_start_simple_metronome(120.0)

# Let it play for 5 seconds
time.sleep(5)

# Stop the metronome
metronome_rs.py_stop_global_metronome()
```

## Documentation

### Rust Documentation
- [API Documentation](https://docs.rs/metronome-rs)
- [Examples Directory](./examples/)

### Python Documentation
- [Python API Guide](./README_PYTHON.md)
- [Cross-Platform Distribution Guide](./CROSS_PLATFORM_GUIDE.md)

## Advanced Usage

### Time Signatures and Accents

**Rust:**
```rust
use metronome_rs::{start_metronome_with_time_signature, AccentConfig};

// 4/4 time with accented first beat
start_metronome_with_time_signature(100.0, 4)?;

// Custom accent configuration
let config = AccentConfig::strong();
start_custom_metronome(120.0, Some(4), config)?;
```

**Python:**
```python
# 4/4 time with accented first beat
metronome_rs.py_start_metronome_with_time_signature(100.0, 4)

# Custom accent configuration
config = metronome_rs.PyAccentConfig.strong()
metronome_rs.py_start_custom_metronome(120.0, 4, config)
```

### Subdivisions for Practice

**Rust:**
```rust
// Eighth note subdivisions (2 per beat)
start_metronome_with_eighth_notes(100.0, Some(4))?;

// Sixteenth note subdivisions (4 per beat)
start_metronome_with_sixteenth_notes(80.0, Some(4))?;

// Triplets (3 per beat)
start_metronome_with_triplets(90.0, Some(4))?;

// Custom subdivisions
start_metronome_with_subdivisions(120.0, Some(4), 6, 0.6)?;
```

**Python:**
```python
# Eighth note subdivisions
metronome_rs.py_start_metronome_with_eighth_notes(100.0, 4)

# Sixteenth note subdivisions  
metronome_rs.py_start_metronome_with_sixteenth_notes(80.0, 4)

# Triplets
metronome_rs.py_start_metronome_with_triplets(90.0, 4)

# Custom subdivisions (6 per beat at 60% volume)
metronome_rs.py_start_metronome_with_subdivisions(120.0, 4, 6, 0.6)
```

### Different Wave Types

**Rust:**
```rust
use metronome_rs::{AccentConfig, WaveType};

let config = AccentConfig::with_wave_types(
    WaveType::Square,    // Accent beats
    WaveType::Triangle   // Regular beats
);
start_custom_metronome(110.0, Some(4), config)?;
```

**Python:**
```python
square_wave = metronome_rs.PyWaveType.square()
triangle_wave = metronome_rs.PyWaveType.triangle()

config = metronome_rs.PyAccentConfig.with_wave_types(square_wave, triangle_wave)
metronome_rs.py_start_custom_metronome(110.0, 4, config)
```

### Timed Practice Sessions

**Rust:**
```rust
// Play for exactly 30 seconds
play_metronome_for_duration(120.0, Some(4), 30000)?;
```

**Python:**
```python
# Play for exactly 30 seconds
metronome_rs.py_play_metronome_for_duration(120.0, 4, 30000)
```

## GUI Examples

### Simple Tkinter Metronome (Python)

```bash
python simple_tkinter_demo.py
```

Features a minimal GUI with:
- BPM input
- Start/Stop button
- Status display

### Full-Featured GUI Demo (Python)

```bash
python tkinter_demo.py
```

Features:
- BPM and time signature input
- Multiple metronome types
- Test beep functionality
- Advanced controls

## Installation & Building

### For Rust Projects

```toml
[dependencies]
metronome-rs = "1.0.0"
```

### For Python Projects

**From PyPI (recommended):**
```bash
pip install metronome-rs
```

**Build from source:**
```bash
# Install Rust and maturin
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
pip install maturin

# Clone and build
git clone https://github.com/arymus/metronome-rs
cd metronome-rs
maturin develop --features python
```

### System Dependencies

- **Linux**: `sudo apt-get install libasound2-dev` (ALSA development libraries)
- **Windows**: No additional dependencies
- **macOS**: No additional dependencies

## Cross-Platform Support

The library uses CPAL's platform-specific audio backends:

- **Linux**: ALSA (Advanced Linux Sound Architecture)
- **Windows**: WASAPI (Windows Audio Session API)  
- **macOS**: CoreAudio

This means Python wheels are platform-specific, but provides optimal performance and native audio integration on each platform.

## API Overview

### Core Functions (Rust)

| Function | Description |
|----------|-------------|
| `start_simple_metronome(bpm)` | Basic metronome without accents |
| `start_metronome_with_time_signature(bpm, beats)` | Metronome with time signature accents |
| `start_practice_metronome(bpm, beats)` | Optimized for practice (subtle accents) |
| `start_performance_metronome(bpm, beats)` | Optimized for performance (strong accents) |
| `start_custom_metronome(bpm, beats, config)` | Full customization control |
| `play_metronome_for_duration(bpm, beats, ms)` | Timed metronome (blocking) |
| `stop_global_metronome()` | Stop any playing metronome |

### Python Bindings

All Rust functions are available with `py_` prefix:
- `py_start_simple_metronome()`
- `py_start_metronome_with_time_signature()`
- `py_play_metronome_for_duration()`
- etc.

Plus Python-friendly classes:
- `PyWaveType` - Wave type enumeration
- `PyAccentConfig` - Accent configuration with builder pattern

## Use Cases

### Musicians
- **Practice Tool**: Subdivisions help with complex rhythms
- **Performance Aid**: Strong accents for live performance
- **Tempo Training**: Precise BPM control for technique development

### Developers
- **Music Software**: Integrate metronome into DAWs or music apps
- **Game Development**: Rhythm game mechanics
- **Audio Applications**: Timing reference for audio processing

### Education
- **Music Teaching**: Visual and audio timing reference
- **Rhythm Training**: Subdivision practice for students
- **Ensemble Practice**: Synchronized timing for groups

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/arymus/metronome-rs
cd metronome-rs

# Run Rust tests
cargo test

# Run examples
cargo run --example simple_metronome

# Build Python bindings
maturin develop --features python

# Test Python bindings
python python_example.py
```

## License

This project is licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.

## Acknowledgments

- Built with [CPAL](https://github.com/RustAudio/cpal) for cross-platform audio
- Python bindings powered by [PyO3](https://github.com/PyO3/pyo3)
- Created by [@arymus](https://github.com/arymus) for the music community

## Support

- [Report Issues](https://github.com/arymus/metronome-rs/issues)
- [Discussions](https://github.com/arymus/metronome-rs/discussions)
- [Documentation](https://docs.rs/metronome-rs)

---

**Made with care for musicians and developers**
