//! # Metronome RS
//!
//! A Rust library for generating audio tones and metronome functionality using CPAL.
//!
//! ## Quick Start
//!
//! ```no_run
//! use metronome_rs::{start_simple_metronome, stop_global_metronome};
//! use std::{thread, time::Duration};
//!
//! // Start a simple 120 BPM metronome
//! start_simple_metronome(120.0)?;
//!
//! // Let it play for 5 seconds
//! thread::sleep(Duration::from_secs(5));
//!
//! // Stop the metronome
//! stop_global_metronome();
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## High-Level Helpers
//!
//! The library provides convenient high-level functions for common use cases:
//!
//! - `start_simple_metronome(bpm)` - Simple metronome without accents
//! - `start_metronome_with_time_signature(bpm, beats)` - Metronome with time signature accents
//! - `play_metronome_for_duration(bpm, beats, duration_ms)` - Timed metronome that auto-stops
//! - `start_practice_metronome(bpm, beats)` - Optimized for practice with subtle accents
//! - `start_performance_metronome(bpm, beats)` - Optimized for performance with strong accents
//! - `start_custom_metronome(bpm, beats, config)` - Full customization control
//!
//! ## Subdivision Support
//!
//! The library supports subdivisions for practicing complex rhythms:
//!
//! - `start_metronome_with_eighth_notes(bpm, beats)` - 2 subdivisions per beat
//! - `start_metronome_with_sixteenth_notes(bpm, beats)` - 4 subdivisions per beat
//! - `start_metronome_with_triplets(bpm, beats)` - 3 subdivisions per beat (triplets)
//! - `start_metronome_with_subdivisions(bpm, beats, subdivisions, volume)` - Custom subdivisions
//!
//! ## Modules
//!
//! - `audio` - Audio device and configuration utilities
//! - `tone` - Tone generation and playbook functionality
//! - `metronome` - Metronome implementation with accent support
//! - `accent` - Accent configuration for metronomes

// Be a perfectionist, no code is good enough!
#![deny(
    clippy::all,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery
)]
// Allow multiple crate versions as it's out of our control due to transitive dependencies
#![allow(clippy::multiple_crate_versions)]

pub mod accent;
pub mod audio;
pub mod metronome;
pub mod tone;

#[cfg(feature = "python")]
pub mod python;

#[cfg(test)]
mod tests;

// Re-export commonly used items for convenience
pub use accent::{AccentConfig, WaveType};
pub use audio::{get_default_host, get_default_output_config, get_default_output_device};
pub use metronome::{
    Metronome,
    get_global_metronome,
    play_custom_metronome_for_duration,
    play_metronome_for_duration,
    start_custom_metronome,
    // Subdivision helper functions
    start_metronome_with_eighth_notes,
    start_metronome_with_sixteenth_notes,
    start_metronome_with_subdivisions,
    start_metronome_with_time_signature,
    start_metronome_with_triplets,
    start_performance_metronome,
    start_practice_metronome,
    // High-level helper functions
    start_simple_metronome,
    stop_global_metronome,
};
pub use tone::{
    beep, beep_frequency, create_sine_wave_generator, play_beep_with_config,
    play_beep_with_config_and_params, play_beep_with_wave_type,
    play_beep_with_wave_type_and_volume, play_default_beep, play_tone, play_tone_with_wave_type,
    play_tone_with_wave_type_and_volume,
};

// Re-export Python bindings when feature is enabled
#[cfg(feature = "python")]
pub use python::{PyAccentConfig, PyWaveType};
