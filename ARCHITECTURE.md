# Metronome-RS: Cross-Platform Python Audio Library

## Project Summary

Metronome-RS is a high-performance audio metronome library written in Rust with Python bindings. The project is designed to handle cross-platform audio compatibility through PyO3 and CPAL.

## Architecture

### Core Components

1. **Rust Core (`src/`)**
   - `lib.rs` - Main library interface with high-level functions
   - `metronome.rs` - Core metronome functionality with singleton pattern
   - `audio.rs` - Audio device detection and configuration
   - `tone.rs` - Low-level audio tone generation
   - `accent.rs` - Accent patterns and configuration
   - `tests.rs` - Comprehensive test suite

2. **Python Bindings (`src/python.rs`)**
   - PyO3-based bindings for all core functionality
   - Python-friendly API with snake_case naming
   - Comprehensive error handling
   - Support for all Rust features

3. **Cross-Platform Compatibility**
   - **Linux**: ALSA backend (requires `libasound2-dev`)
   - **Windows**: WASAPI backend (built-in)
   - **macOS**: CoreAudio backend (built-in)

## Key Features

### Rust API
- Simple metronome functions (`start_simple_metronome`, etc.)
- Advanced subdivision support (eighth notes, sixteenth notes, triplets)
- Custom accent configurations with different wave types
- Timed metronome functions
- Global singleton to prevent audio conflicts

### Python API
- All Rust functionality exposed through `py_*` functions
- Object-oriented configuration classes (`PyAccentConfig`, `PyWaveType`)
- Comprehensive examples and documentation

### Cross-Platform Distribution
- GitHub Actions workflow for multi-platform wheel building
- Maturin-based Python packaging
- Platform-specific optimizations through conditional compilation

## Usage Examples

### Rust
```rust
use metronome_rs::{start_simple_metronome, stop_global_metronome};
use std::{thread, time::Duration};

start_simple_metronome(120.0)?;
thread::sleep(Duration::from_secs(5));
stop_global_metronome();
```

### Python
```python
import metronome_rs
import time

metronome_rs.py_start_simple_metronome(120.0)
time.sleep(5)
metronome_rs.py_stop_global_metronome()
```

## Building and Distribution

### Development
```bash
# Rust development
cargo build
cargo test
cargo run --example simple_metronome

# Python development
maturin develop --features python
python python_example.py
```

### Production Builds
```bash
# Platform-specific Python wheels
maturin build --features python --release

# Multi-platform via GitHub Actions
git tag v0.1.0
git push origin v0.1.0  # Triggers CI/CD
```

## Design Decisions

### Why Platform-Specific Builds?
CPAL uses conditional compilation to select audio backends at compile time. This provides:
- Optimal performance per platform
- Native audio API integration
- Smaller binary sizes
- Better system compatibility

### Why PyO3 0.25.1?
- Latest stable API with `Bound<>` types
- Better memory safety
- Improved error handling
- Active maintenance and support

### Why Singleton Pattern?
- Prevents audio device conflicts
- Simplifies API for common use cases
- Ensures only one metronome plays at a time
- Matches user expectations

## Files Structure

```
metronome-rs/
├── src/
│   ├── lib.rs              # Main library exports
│   ├── metronome.rs        # Core metronome logic
│   ├── audio.rs            # Audio device utilities
│   ├── tone.rs             # Tone generation
│   ├── accent.rs           # Accent configurations
│   ├── python.rs           # Python bindings
│   └── tests.rs            # Test suite
├── examples/               # Rust examples
├── .github/workflows/      # CI/CD automation
├── Cargo.toml              # Rust configuration
├── pyproject.toml          # Python packaging
├── python_example.py       # Python demo
├── tkinter_demo.py         # Full-featured GUI demo
├── simple_tkinter_demo.py  # Minimal GUI demo
├── README_PYTHON.md        # Python documentation
└── CROSS_PLATFORM_GUIDE.md # Distribution guide
```

## Testing

The project includes comprehensive testing:
- Rust unit tests for all components
- Python integration tests
- Audio output verification
- Cross-platform CI/CD testing

## License

Dual-licensed under MIT OR Apache-2.0, following Rust ecosystem conventions.
