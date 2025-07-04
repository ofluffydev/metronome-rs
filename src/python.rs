//! Python bindings for metronome-rs using PyO3
//!
//! This module provides Python bindings for the metronome library, allowing
//! Python code to use the metronome functionality.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyModule;

#[cfg(feature = "python")]
use crate::{
    accent::{AccentConfig, WaveType},
    metronome::{
        play_custom_metronome_for_duration, play_metronome_for_duration, start_custom_metronome,
        start_metronome_with_eighth_notes, start_metronome_with_sixteenth_notes,
        start_metronome_with_subdivisions, start_metronome_with_time_signature,
        start_metronome_with_triplets, start_performance_metronome, start_practice_metronome,
        start_simple_metronome, stop_global_metronome,
    },
    tone::{beep, beep_frequency},
};

#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
/// Python wrapper for WaveType enum
pub struct PyWaveType {
    inner: WaveType,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyWaveType {
    #[new]
    fn new(wave_type: &str) -> PyResult<Self> {
        let inner = match wave_type.to_lowercase().as_str() {
            "sine" => WaveType::Sine,
            "square" => WaveType::Square,
            "sawtooth" => WaveType::Sawtooth,
            "triangle" => WaveType::Triangle,
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "Invalid wave type. Must be one of: 'sine', 'square', 'sawtooth', 'triangle'",
                ))
            }
        };
        Ok(PyWaveType { inner })
    }

    #[staticmethod]
    fn sine() -> Self {
        PyWaveType {
            inner: WaveType::Sine,
        }
    }

    #[staticmethod]
    fn square() -> Self {
        PyWaveType {
            inner: WaveType::Square,
        }
    }

    #[staticmethod]
    fn sawtooth() -> Self {
        PyWaveType {
            inner: WaveType::Sawtooth,
        }
    }

    #[staticmethod]
    fn triangle() -> Self {
        PyWaveType {
            inner: WaveType::Triangle,
        }
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("PyWaveType('{:?}')", self.inner)
    }
}

#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
/// Python wrapper for AccentConfig
pub struct PyAccentConfig {
    inner: AccentConfig,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyAccentConfig {
    #[new]
    #[pyo3(signature = (accent_frequency=880.0, regular_frequency=440.0, accent_duration=150, regular_duration=100, accent_wave_type=None, regular_wave_type=None, subdivisions=1, subdivision_frequency=523.25, subdivision_duration=80, subdivision_wave_type=None, subdivision_volume=0.7))]
    fn new(
        accent_frequency: f32,
        regular_frequency: f32,
        accent_duration: u64,
        regular_duration: u64,
        accent_wave_type: Option<PyWaveType>,
        regular_wave_type: Option<PyWaveType>,
        subdivisions: u32,
        subdivision_frequency: f32,
        subdivision_duration: u64,
        subdivision_wave_type: Option<PyWaveType>,
        subdivision_volume: f32,
    ) -> Self {
        let accent_wave = accent_wave_type
            .map(|w| w.inner)
            .unwrap_or(WaveType::Sine);
        let regular_wave = regular_wave_type
            .map(|w| w.inner)
            .unwrap_or(WaveType::Sine);
        let subdivision_wave = subdivision_wave_type
            .map(|w| w.inner)
            .unwrap_or(WaveType::Sine);

        PyAccentConfig {
            inner: AccentConfig {
                accent_frequency,
                regular_frequency,
                accent_duration,
                regular_duration,
                accent_wave_type: accent_wave,
                regular_wave_type: regular_wave,
                subdivisions,
                subdivision_frequency,
                subdivision_duration,
                subdivision_wave_type: subdivision_wave,
                subdivision_volume,
            },
        }
    }

    #[staticmethod]
    fn default() -> Self {
        PyAccentConfig {
            inner: AccentConfig::default(),
        }
    }

    #[staticmethod]
    fn subtle() -> Self {
        PyAccentConfig {
            inner: AccentConfig::subtle(),
        }
    }

    #[staticmethod]
    fn strong() -> Self {
        PyAccentConfig {
            inner: AccentConfig::strong(),
        }
    }

    #[staticmethod]
    fn with_wave_type(wave_type: PyWaveType) -> Self {
        PyAccentConfig {
            inner: AccentConfig::with_wave_type(wave_type.inner),
        }
    }

    #[staticmethod]
    fn with_wave_types(accent_wave: PyWaveType, regular_wave: PyWaveType) -> Self {
        PyAccentConfig {
            inner: AccentConfig::with_wave_types(accent_wave.inner, regular_wave.inner),
        }
    }

    #[staticmethod]
    fn with_eighth_notes() -> Self {
        PyAccentConfig {
            inner: AccentConfig::with_eighth_notes(),
        }
    }

    #[staticmethod]
    fn with_sixteenth_notes() -> Self {
        PyAccentConfig {
            inner: AccentConfig::with_sixteenth_notes(),
        }
    }

    #[staticmethod]
    fn with_triplets() -> Self {
        PyAccentConfig {
            inner: AccentConfig::with_triplets(),
        }
    }

    #[staticmethod]
    fn with_custom_subdivisions(
        subdivisions: u32,
        subdivision_frequency: f32,
        subdivision_volume: f32,
    ) -> Self {
        PyAccentConfig {
            inner: AccentConfig::with_custom_subdivisions(
                subdivisions,
                subdivision_frequency,
                subdivision_volume,
            ),
        }
    }

    fn set_subdivisions(&mut self, subdivisions: u32) -> Self {
        PyAccentConfig {
            inner: self.inner.clone().set_subdivisions(subdivisions),
        }
    }

    fn set_subdivision_frequency(&mut self, frequency: f32) -> Self {
        PyAccentConfig {
            inner: self.inner.clone().set_subdivision_frequency(frequency),
        }
    }

    fn set_subdivision_volume(&mut self, volume: f32) -> Self {
        PyAccentConfig {
            inner: self.inner.clone().set_subdivision_volume(volume),
        }
    }

    fn set_subdivision_wave_type(&mut self, wave_type: PyWaveType) -> Self {
        PyAccentConfig {
            inner: self
                .inner
                .clone()
                .set_subdivision_wave_type(wave_type.inner),
        }
    }

    #[getter]
    fn accent_frequency(&self) -> f32 {
        self.inner.accent_frequency
    }

    #[getter]
    fn regular_frequency(&self) -> f32 {
        self.inner.regular_frequency
    }

    #[getter]
    fn accent_duration(&self) -> u64 {
        self.inner.accent_duration
    }

    #[getter]
    fn regular_duration(&self) -> u64 {
        self.inner.regular_duration
    }

    #[getter]
    fn subdivisions(&self) -> u32 {
        self.inner.subdivisions
    }

    #[getter]
    fn subdivision_frequency(&self) -> f32 {
        self.inner.subdivision_frequency
    }

    #[getter]
    fn subdivision_duration(&self) -> u64 {
        self.inner.subdivision_duration
    }

    #[getter]
    fn subdivision_volume(&self) -> f32 {
        self.inner.subdivision_volume
    }

    fn __str__(&self) -> String {
        format!("PyAccentConfig(accent_freq={}, regular_freq={}, subdivisions={})", 
                self.inner.accent_frequency, self.inner.regular_frequency, self.inner.subdivisions)
    }
}

#[cfg(feature = "python")]
/// Play a simple beep sound
#[pyfunction]
fn py_beep() -> PyResult<()> {
    beep().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to play beep: {}", e)))
}

#[cfg(feature = "python")]
/// Play a beep at a specific frequency
#[pyfunction]
fn py_beep_frequency(frequency: f32) -> PyResult<()> {
    beep_frequency(frequency).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to play beep: {}", e))
    })
}

#[cfg(feature = "python")]
/// Start a simple metronome without accents
#[pyfunction]
fn py_start_simple_metronome(bpm: f64) -> PyResult<()> {
    start_simple_metronome(bpm).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to start metronome: {}", e))
    })
}

#[cfg(feature = "python")]
/// Start a metronome with time signature accents
#[pyfunction]
fn py_start_metronome_with_time_signature(bpm: f64, beats_per_measure: u32) -> PyResult<()> {
    start_metronome_with_time_signature(bpm, beats_per_measure).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to start metronome: {}", e))
    })
}

#[cfg(feature = "python")]
/// Start a practice metronome with subtle accents
#[pyfunction]
fn py_start_practice_metronome(bpm: f64, beats_per_measure: u32) -> PyResult<()> {
    start_practice_metronome(bpm, beats_per_measure).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to start metronome: {}", e))
    })
}

#[cfg(feature = "python")]
/// Start a performance metronome with strong accents
#[pyfunction]
fn py_start_performance_metronome(bpm: f64, beats_per_measure: u32) -> PyResult<()> {
    start_performance_metronome(bpm, beats_per_measure).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to start metronome: {}", e))
    })
}

#[cfg(feature = "python")]
/// Start a custom metronome with full accent configuration
#[pyfunction]
fn py_start_custom_metronome(
    bpm: f64,
    beats_per_measure: Option<u32>,
    accent_config: PyAccentConfig,
) -> PyResult<()> {
    start_custom_metronome(bpm, beats_per_measure, accent_config.inner).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to start metronome: {}", e))
    })
}

#[cfg(feature = "python")]
/// Start a metronome with eighth note subdivisions
#[pyfunction]
fn py_start_metronome_with_eighth_notes(bpm: f64, beats_per_measure: u32) -> PyResult<()> {
    start_metronome_with_eighth_notes(bpm, Some(beats_per_measure)).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to start metronome: {}", e))
    })
}

#[cfg(feature = "python")]
/// Start a metronome with sixteenth note subdivisions
#[pyfunction]
fn py_start_metronome_with_sixteenth_notes(bpm: f64, beats_per_measure: u32) -> PyResult<()> {
    start_metronome_with_sixteenth_notes(bpm, Some(beats_per_measure)).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to start metronome: {}", e))
    })
}

#[cfg(feature = "python")]
/// Start a metronome with triplet subdivisions
#[pyfunction]
fn py_start_metronome_with_triplets(bpm: f64, beats_per_measure: u32) -> PyResult<()> {
    start_metronome_with_triplets(bpm, Some(beats_per_measure)).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to start metronome: {}", e))
    })
}

#[cfg(feature = "python")]
/// Start a metronome with custom subdivisions
#[pyfunction]
fn py_start_metronome_with_subdivisions(
    bpm: f64,
    beats_per_measure: u32,
    subdivisions: u32,
    subdivision_volume: f32,
) -> PyResult<()> {
    start_metronome_with_subdivisions(bpm, Some(beats_per_measure), subdivisions, subdivision_volume)
        .map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to start metronome: {}", e))
        })
}

#[cfg(feature = "python")]
/// Play a metronome for a specific duration
#[pyfunction]
fn py_play_metronome_for_duration(
    bpm: f64,
    beats_per_measure: Option<u32>,
    duration_ms: u64,
) -> PyResult<()> {
    play_metronome_for_duration(bpm, beats_per_measure, duration_ms).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to play metronome: {}", e))
    })
}

#[cfg(feature = "python")]
/// Play a custom metronome for a specific duration
#[pyfunction]
fn py_play_custom_metronome_for_duration(
    bpm: f64,
    beats_per_measure: Option<u32>,
    accent_config: PyAccentConfig,
    duration_ms: u64,
) -> PyResult<()> {
    play_custom_metronome_for_duration(bpm, beats_per_measure, accent_config.inner, duration_ms)
        .map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to play metronome: {}", e))
        })
}

#[cfg(feature = "python")]
/// Stop any currently playing metronome
#[pyfunction]
fn py_stop_global_metronome() {
    stop_global_metronome();
}

#[cfg(feature = "python")]
/// Python module initialization
#[pymodule]
fn metronome_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyWaveType>()?;
    m.add_class::<PyAccentConfig>()?;

    // Basic functions
    m.add_function(wrap_pyfunction!(py_beep, m)?)?;
    m.add_function(wrap_pyfunction!(py_beep_frequency, m)?)?;

    // Metronome control functions
    m.add_function(wrap_pyfunction!(py_start_simple_metronome, m)?)?;
    m.add_function(wrap_pyfunction!(py_start_metronome_with_time_signature, m)?)?;
    m.add_function(wrap_pyfunction!(py_start_practice_metronome, m)?)?;
    m.add_function(wrap_pyfunction!(py_start_performance_metronome, m)?)?;
    m.add_function(wrap_pyfunction!(py_start_custom_metronome, m)?)?;

    // Subdivision functions
    m.add_function(wrap_pyfunction!(py_start_metronome_with_eighth_notes, m)?)?;
    m.add_function(wrap_pyfunction!(py_start_metronome_with_sixteenth_notes, m)?)?;
    m.add_function(wrap_pyfunction!(py_start_metronome_with_triplets, m)?)?;
    m.add_function(wrap_pyfunction!(py_start_metronome_with_subdivisions, m)?)?;

    // Timed functions
    m.add_function(wrap_pyfunction!(py_play_metronome_for_duration, m)?)?;
    m.add_function(wrap_pyfunction!(py_play_custom_metronome_for_duration, m)?)?;

    // Control functions
    m.add_function(wrap_pyfunction!(py_stop_global_metronome, m)?)?;

    Ok(())
}
