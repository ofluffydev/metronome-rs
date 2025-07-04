# Metronome-RS Python Bindings

Python bindings for the metronome-rs Rust library, providing high-performance audio metronome functionality.

## Installation

### Option 1: Install from PyPI (when available)
```bash
pip install metronome-rs
```

### Option 2: Build from source

1. Install Rust and maturin:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
pip install maturin
```

2. Clone and build:
```bash
git clone https://github.com/ofluffydev/metronome-rs
cd metronome-rs
maturin develop --features python
```

### Option 3: Build wheel for distribution
```bash
maturin build --features python --release
```

## Cross-Platform Compatibility

Due to CPAL's platform-specific audio backends, the Python wheel is platform-specific:

- **Linux**: Uses ALSA backend
- **Windows**: Uses WASAPI backend  
- **macOS**: Uses CoreAudio backend

This means you need to build separate wheels for each platform you want to support. The library handles this automatically when built on the target platform.

## Quick Start

```python
import metronome_rs
import time

# Play a simple beep
metronome_rs.py_beep()

# Start a 120 BPM metronome
metronome_rs.py_start_simple_metronome(120.0)
time.sleep(5)  # Let it play for 5 seconds
metronome_rs.py_stop_global_metronome()

# Play a metronome for a specific duration (blocking)
metronome_rs.py_play_metronome_for_duration(100.0, 4, 3000)  # 100 BPM, 4/4 time, 3 seconds
```

## Advanced Usage

### Custom Wave Types

```python
import metronome_rs

# Create different wave types
sine_wave = metronome_rs.PyWaveType.sine()
square_wave = metronome_rs.PyWaveType.square()
triangle_wave = metronome_rs.PyWaveType.triangle()
sawtooth_wave = metronome_rs.PyWaveType.sawtooth()

# Use custom wave type in metronome
config = metronome_rs.PyAccentConfig.with_wave_type(square_wave)
metronome_rs.py_start_custom_metronome(110.0, 4, config)
```

### Accent Configurations

```python
import metronome_rs

# Use preset configurations
subtle_config = metronome_rs.PyAccentConfig.subtle()
strong_config = metronome_rs.PyAccentConfig.strong()

# Create custom configuration
custom_config = metronome_rs.PyAccentConfig(
    accent_frequency=880.0,
    regular_frequency=440.0,
    accent_duration=150,
    regular_duration=100,
    subdivisions=2  # Eighth note subdivisions
)

# Apply builder pattern
config = (metronome_rs.PyAccentConfig.default()
          .set_subdivisions(4)
          .set_subdivision_volume(0.8))
```

### Subdivisions

```python
import metronome_rs

# Common subdivision patterns
metronome_rs.py_start_metronome_with_eighth_notes(100.0, 4)    # 2 subdivisions per beat
metronome_rs.py_start_metronome_with_sixteenth_notes(80.0, 4)  # 4 subdivisions per beat
metronome_rs.py_start_metronome_with_triplets(90.0, 4)        # 3 subdivisions per beat

# Custom subdivisions
metronome_rs.py_start_metronome_with_subdivisions(120.0, 4, 6, 0.6)  # 6 subdivisions, 60% volume
```

## API Reference

### Functions

- `py_beep()` - Play a simple beep sound
- `py_beep_frequency(frequency)` - Play a beep at specific frequency
- `py_start_simple_metronome(bpm)` - Start basic metronome
- `py_start_metronome_with_time_signature(bpm, beats_per_measure)` - Start metronome with accents
- `py_start_practice_metronome(bpm, beats_per_measure)` - Start practice metronome (subtle accents)
- `py_start_performance_metronome(bpm, beats_per_measure)` - Start performance metronome (strong accents)
- `py_start_custom_metronome(bpm, beats_per_measure, config)` - Start with custom configuration
- `py_play_metronome_for_duration(bpm, beats_per_measure, duration_ms)` - Timed metronome (blocking)
- `py_stop_global_metronome()` - Stop any currently playing metronome

### Classes

#### PyWaveType
- `PyWaveType.sine()` - Pure sine wave
- `PyWaveType.square()` - Square wave (harsh, digital)
- `PyWaveType.triangle()` - Triangle wave (softer than square)
- `PyWaveType.sawtooth()` - Sawtooth wave (bright, buzzy)

#### PyAccentConfig
Configuration class for metronome accents and subdivisions.

**Static Methods:**
- `PyAccentConfig.default()` - Default configuration
- `PyAccentConfig.subtle()` - Subtle accent differences
- `PyAccentConfig.strong()` - Strong accent differences
- `PyAccentConfig.with_wave_type(wave_type)` - Same wave type for all sounds
- `PyAccentConfig.with_wave_types(accent_wave, regular_wave)` - Different waves for accent/regular
- `PyAccentConfig.with_eighth_notes()` - Preset for eighth note subdivisions
- `PyAccentConfig.with_sixteenth_notes()` - Preset for sixteenth note subdivisions
- `PyAccentConfig.with_triplets()` - Preset for triplet subdivisions

**Builder Methods:**
- `.set_subdivisions(count)` - Set number of subdivisions per beat
- `.set_subdivision_frequency(freq)` - Set subdivision frequency
- `.set_subdivision_volume(volume)` - Set subdivision volume (0.0-1.0)
- `.set_subdivision_wave_type(wave_type)` - Set subdivision wave type

## Development

### Building for Multiple Platforms

To create wheels for different platforms:

```bash
# Linux
maturin build --features python --release

# Windows (on Windows machine)
maturin build --features python --release

# macOS (on macOS machine)  
maturin build --features python --release
```

### Testing

```bash
# Run the example
python python_examples/python_example.py

# Or test manually
python -c "import metronome_rs; metronome_rs.py_beep()"
```

## Platform-Specific Notes

### Linux
- Requires ALSA development libraries: `sudo apt-get install libasound2-dev`
- May require PulseAudio for some systems

### Windows
- Uses WASAPI (available on Windows Vista+)
- No additional dependencies required

### macOS
- Uses CoreAudio framework
- Available on macOS 10.7+

## Troubleshooting

### Audio Issues
- **No sound**: Check system audio settings and ensure audio device is working
- **Crackling/distortion**: Try adjusting volume levels in configurations
- **High latency**: This is normal for this library - it's designed for metronome use, not real-time audio

### Build Issues
- **Missing Rust**: Install Rust from https://rustup.rs/
- **Missing maturin**: Install with `pip install maturin`
- **Feature errors**: Ensure you're building with `--features python`

## License

This project is licensed under either of:
- Apache License, Version 2.0
- MIT License

at your option.

## GUI Examples

### Tkinter Demos

Two GUI examples are provided to demonstrate integration with desktop applications:

#### Full-Featured Demo (`python_examples/tkinter_demo.py`)
```python
python python_examples/tkinter_demo.py
```
Features:
- BPM and time signature input
- Multiple metronome types (Simple, With Accents, Practice, Performance)
- Start/Stop controls
- Test beep functionality
- Status display

#### Simple Demo (`python_examples/simple_tkinter_demo.py`)
```python
python python_examples/simple_tkinter_demo.py
```
Minimal interface with just:
- BPM input
- Start/Stop button
- Status display

Both demos handle proper cleanup when the window is closed and include error handling for invalid inputs
