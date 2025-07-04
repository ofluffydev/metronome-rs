# Python Examples

This directory contains Python example scripts demonstrating how to use the metronome-rs Python bindings.

## Files

- **`python_example.py`** - Comprehensive command-line example showcasing all library features including:
  - Basic beep functionality
  - Simple metronomes
  - Custom configurations and wave types
  - Subdivision patterns
  - Timed metronome playback

- **`tkinter_demo.py`** - Full-featured GUI demo with:
  - BPM and time signature input
  - Multiple metronome types (Simple, With Accents, Practice, Performance)
  - Start/Stop controls
  - Test beep functionality
  - Status display

- **`simple_tkinter_demo.py`** - Minimal GUI example with just:
  - BPM input
  - Start/Stop button
  - Status display

## Running the Examples

Make sure you have the metronome-rs Python bindings installed first:

```bash
# From the project root
maturin develop --features python
```

Then run any example:

```bash
# Command-line demo
python python_examples/python_example.py

# GUI demos
python python_examples/tkinter_demo.py
python python_examples/simple_tkinter_demo.py
```

## Requirements

- Python 3.6+
- tkinter (for GUI examples, usually included with Python)
- metronome-rs Python bindings

The GUI examples will automatically handle cleanup when the window is closed and include error handling for invalid inputs.
