# Cross-Platform Distribution Guide

This document explains how to build and distribute metronome-rs Python wheels for multiple platforms.

## Why Platform-Specific Builds Are Needed

The metronome-rs library uses CPAL (Cross-Platform Audio Library) for audio output, which compiles to different audio backends on different platforms:

- **Linux**: ALSA (Advanced Linux Sound Architecture)
- **Windows**: WASAPI (Windows Audio Session API)
- **macOS**: CoreAudio

These backends are compiled into the binary, so a wheel built on Linux won't work on Windows or macOS.

## Building for Multiple Platforms

### Local Development

For local development, build on your target platform:

```bash
# Install dependencies
pip install maturin

# Build and install in development mode
maturin develop --features python

# Or build a wheel
maturin build --features python --release
```

### Cross-Platform Distribution

For distributing your package, you have several options:

#### Option 1: GitHub Actions (Recommended)

The included `.github/workflows/build-wheels.yml` will automatically build wheels for Linux, Windows, and macOS when you push a tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This will:
1. Build wheels for all three platforms
2. Test installation on each platform
3. Upload artifacts
4. Optionally publish to PyPI (if `PYPI_API_TOKEN` secret is configured)

#### Option 2: Manual Building

Build on each platform manually:

```bash
# On Linux
sudo apt-get install libasound2-dev  # Install ALSA development libraries
maturin build --features python --release

# On Windows
maturin build --features python --release

# On macOS
maturin build --features python --release
```

#### Option 3: Cross-Compilation

While Rust supports cross-compilation, audio libraries often require platform-specific system libraries, making this approach complex. GitHub Actions is recommended instead.

## System Dependencies

### Linux
- ALSA development libraries: `libasound2-dev` (Ubuntu/Debian) or `alsa-lib-devel` (RHEL/Fedora)

### Windows
- No additional dependencies (WASAPI is built into Windows Vista+)

### macOS
- No additional dependencies (CoreAudio is built into macOS)

## Testing Your Builds

After building wheels, test them on the target platform:

```bash
# Install the wheel
pip install dist/metronome_rs-*.whl

# Test basic functionality
python -c "
import metronome_rs
import time

print('Testing beep...')
metronome_rs.py_beep()
time.sleep(1)

print('Testing metronome...')
metronome_rs.py_start_simple_metronome(120.0)
time.sleep(2)
metronome_rs.py_stop_global_metronome()
print('Success!')
"
```

## Publishing to PyPI

1. Build wheels for all platforms (using GitHub Actions or manually)
2. Install twine: `pip install twine`
3. Upload to PyPI: `twine upload dist/*.whl`

Or use the automated GitHub Actions workflow by pushing a version tag.

## Troubleshooting

### Build Failures

**Linux**: Missing ALSA libraries
```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev

# RHEL/Fedora
sudo dnf install alsa-lib-devel
```

**Windows**: Usually works out of the box with Visual Studio Build Tools

**macOS**: Usually works out of the box with Xcode command line tools

### Runtime Issues

**No Audio Output**: Check that:
1. System audio is working
2. Audio device is not muted
3. No other applications are blocking audio access

**Import Errors**: Ensure you're installing the correct wheel for your platform and Python version.
