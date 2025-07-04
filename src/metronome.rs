use cpal::{Device, StreamConfig};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU64, Ordering},
};
use std::thread;
use std::time::Duration;

use crate::accent::AccentConfig;
use crate::audio::{get_default_host, get_default_output_config, get_default_output_device};

/// Global metronome instance to ensure only one metronome can play at a time
static GLOBAL_METRONOME: Mutex<Option<Arc<Metronome>>> = Mutex::new(None);

/// A metronome that can play at a specified BPM with optional measure accents.
#[derive(Clone)]
pub struct Metronome {
    bpm: f64,
    beats_per_measure: Option<u32>,
    is_playing: Arc<AtomicBool>,
    device: Arc<Device>,
    config: StreamConfig,
    accent_config: AccentConfig,
    id: Arc<AtomicU64>, // Unique ID for this metronome instance
}

static METRONOME_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

impl Metronome {
    /// Creates a new metronome with the specified BPM.
    ///
    /// # Arguments
    ///
    /// * `bpm` - Beats per minute (e.g., 120.0)
    /// * `beats_per_measure` - Optional number of beats per measure for accented first beat
    ///
    /// # Returns
    ///
    /// A new metronome instance using the default audio device and configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the default audio device or configuration cannot be obtained.
    pub fn new(
        bpm: f64,
        beats_per_measure: Option<u32>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let host = get_default_host();
        let device = get_default_output_device(&host)?;
        let config = get_default_output_config(&device)?;

        Ok(Self {
            bpm,
            beats_per_measure,
            is_playing: Arc::new(AtomicBool::new(false)),
            device: Arc::new(device),
            config: config.into(),
            accent_config: AccentConfig::default(),
            id: Arc::new(AtomicU64::new(
                METRONOME_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            )),
        })
    }

    /// Creates a new metronome with custom accent configuration.
    ///
    /// # Arguments
    ///
    /// * `bpm` - Beats per minute (e.g., 120.0)
    /// * `beats_per_measure` - Optional number of beats per measure for accented first beat
    /// * `accent_config` - Custom accent configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the default audio device or configuration cannot be obtained.
    pub fn new_with_accent(
        bpm: f64,
        beats_per_measure: Option<u32>,
        accent_config: AccentConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let host = get_default_host();
        let device = get_default_output_device(&host)?;
        let config = get_default_output_config(&device)?;

        Ok(Self {
            bpm,
            beats_per_measure,
            is_playing: Arc::new(AtomicBool::new(false)),
            device: Arc::new(device),
            config: config.into(),
            accent_config,
            id: Arc::new(AtomicU64::new(
                METRONOME_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            )),
        })
    }

    /// Creates a new metronome with custom audio device and configuration.
    pub fn with_device_config(
        bpm: f64,
        beats_per_measure: Option<u32>,
        device: Device,
        config: StreamConfig,
    ) -> Self {
        Self {
            bpm,
            beats_per_measure,
            is_playing: Arc::new(AtomicBool::new(false)),
            device: Arc::new(device),
            config,
            accent_config: AccentConfig::default(),
            id: Arc::new(AtomicU64::new(
                METRONOME_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            )),
        }
    }

    /// Gets the current BPM.
    #[must_use]
    pub const fn bpm(&self) -> f64 {
        self.bpm
    }

    /// Sets the BPM.
    pub const fn set_bpm(&mut self, bpm: f64) {
        self.bpm = bpm;
    }

    /// Gets the beats per measure.
    #[must_use]
    pub const fn beats_per_measure(&self) -> Option<u32> {
        self.beats_per_measure
    }

    /// Sets the beats per measure.
    pub const fn set_beats_per_measure(&mut self, beats_per_measure: Option<u32>) {
        self.beats_per_measure = beats_per_measure;
    }

    /// Gets the accent configuration.
    #[must_use]
    pub const fn accent_config(&self) -> &AccentConfig {
        &self.accent_config
    }

    /// Sets the accent configuration.
    pub const fn set_accent_config(&mut self, accent_config: AccentConfig) {
        self.accent_config = accent_config;
    }

    /// Checks if the metronome is currently playing.
    #[must_use]
    pub fn is_playing(&self) -> bool {
        self.is_playing.load(Ordering::Relaxed)
    }

    /// Starts the metronome. This will stop any currently playing metronome globally.
    ///
    /// # Errors
    ///
    /// Returns an error if there's an issue with thread creation or other system resources.
    ///
    /// # Panics
    ///
    /// May panic if the global metronome mutex is poisoned due to a previous panic in another thread.
    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Stop any currently playing metronome
        let current_metronome = {
            let mut global = GLOBAL_METRONOME.lock().unwrap();
            let current = global.take(); // Take ownership and clear the global
            *global = Some(Arc::new(self.clone())); // Set new metronome
            current
        }; // Release lock before calling stop()

        // Now stop the previous metronome outside the lock
        if let Some(metronome) = current_metronome {
            metronome.is_playing.store(false, Ordering::Relaxed);
        }

        self.is_playing.store(true, Ordering::Relaxed);

        let metronome = self.clone();
        thread::spawn(move || {
            metronome.run_metronome();
        });

        Ok(())
    }

    /// Stops the metronome.
    pub fn stop(&self) {
        self.is_playing.store(false, Ordering::Relaxed);

        // Remove from global if this is the current metronome
        if let Ok(mut global) = GLOBAL_METRONOME.lock() {
            if let Some(current) = global.as_ref() {
                let current_id = current.id.load(Ordering::Relaxed);
                let self_id = self.id.load(Ordering::Relaxed);
                if current_id == self_id {
                    *global = None;
                }
            }
        }
    }

    /// Internal method that runs the metronome loop.
    fn run_metronome(&self) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let beat_duration_ms = (60.0 / self.bpm * 1000.0) as u64;
        let subdivision_duration_ms = beat_duration_ms / u64::from(self.accent_config.subdivisions);
        let mut beat_count = 0u32;
        let mut subdivision_count = 0u32;

        while self.is_playing.load(Ordering::Relaxed) {
            let is_accent = self.beats_per_measure
                .is_some_and(|beats| beat_count % beats == 0 && subdivision_count == 0);

            let is_main_beat = subdivision_count == 0;

            // Determine what type of sound to play
            let (frequency, duration, wave_type, volume) = if is_accent {
                // Accent beat (first beat of measure)
                (
                    self.accent_config.accent_frequency,
                    self.accent_config.accent_duration,
                    self.accent_config.accent_wave_type.clone(),
                    1.0, // Full volume for accents
                )
            } else if is_main_beat {
                // Regular beat (non-accent main beats)
                (
                    self.accent_config.regular_frequency,
                    self.accent_config.regular_duration,
                    self.accent_config.regular_wave_type.clone(),
                    1.0, // Full volume for main beats
                )
            } else if self.accent_config.subdivisions > 1 {
                // Subdivision click
                (
                    self.accent_config.subdivision_frequency,
                    self.accent_config.subdivision_duration,
                    self.accent_config.subdivision_wave_type.clone(),
                    self.accent_config.subdivision_volume,
                )
            } else {
                // Skip if subdivisions = 1 and it's not a main beat
                subdivision_count = (subdivision_count + 1) % self.accent_config.subdivisions;
                if subdivision_count == 0 {
                    beat_count += 1;
                }
                continue;
            };

            // Play the click using the tone module with volume control
            if let Err(e) = crate::tone::play_beep_with_wave_type_and_volume(
                self.device.as_ref(),
                &self.config,
                frequency,
                duration,
                wave_type,
                volume,
            ) {
                eprintln!("Error playing metronome click: {e}");
                break;
            }

            // Update counters
            subdivision_count = (subdivision_count + 1) % self.accent_config.subdivisions;
            if subdivision_count == 0 {
                beat_count += 1;
            }

            // Sleep for the remaining time of the subdivision
            let sleep_duration = subdivision_duration_ms.saturating_sub(duration);
            if sleep_duration > 0 {
                thread::sleep(Duration::from_millis(sleep_duration));
            }
        }
    }
}

/// Stops any currently playing metronome globally.
///
/// # Panics
///
/// May panic if the global metronome mutex is poisoned due to a previous panic in another thread.
pub fn stop_global_metronome() {
    let metronome = {
        let mut global = GLOBAL_METRONOME.lock().unwrap();
        global.take() // Take ownership and clear the global
    }; // Release lock before calling stop()

    if let Some(metronome) = metronome {
        metronome.is_playing.store(false, Ordering::Relaxed);
    }
}

/// Gets a reference to the currently playing metronome, if any.
pub fn get_global_metronome() -> Option<Arc<Metronome>> {
    GLOBAL_METRONOME.lock().ok()?.as_ref().cloned()
}

/// Creates and starts a simple metronome without accents that plays indefinitely.
///
/// This is a high-level convenience function for the most common use case.
/// The metronome will play until `stop_global_metronome()` is called.
///
/// # Arguments
///
/// * `bpm` - Beats per minute (e.g., 120.0)
///
/// # Returns
///
/// Returns `Ok(())` if the metronome started successfully, or an error if audio setup failed.
///
/// # Examples
///
/// ```no_run
/// use metronome_rs::{start_simple_metronome, stop_global_metronome};
/// use std::{thread, time::Duration};
///
/// // Start a simple 120 BPM metronome
/// start_simple_metronome(120.0)?;
///
/// // Let it play for 5 seconds
/// thread::sleep(Duration::from_secs(5));
///
/// // Stop the metronome
/// stop_global_metronome();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Errors
///
/// Returns an error if the audio device or configuration cannot be obtained, or if there's an issue starting the metronome.
pub fn start_simple_metronome(bpm: f64) -> Result<(), Box<dyn std::error::Error>> {
    let metronome = Metronome::new(bpm, None)?;
    metronome.start()
}

/// Creates and starts a metronome with time signature that plays indefinitely.
///
/// This creates a metronome with accented first beats according to the time signature.
/// The metronome will play until `stop_global_metronome()` is called.
///
/// # Arguments
///
/// * `bpm` - Beats per minute (e.g., 120.0)
/// * `beats_per_measure` - Number of beats per measure (e.g., 4 for 4/4 time)
///
/// # Returns
///
/// Returns `Ok(())` if the metronome started successfully, or an error if audio setup failed.
///
/// # Examples
///
/// ```no_run
/// use metronome_rs::{start_metronome_with_time_signature, stop_global_metronome};
/// use std::{thread, time::Duration};
///
/// // Start a 4/4 time metronome at 100 BPM
/// start_metronome_with_time_signature(100.0, 4)?;
///
/// // Let it play for 8 seconds (about 2 measures)
/// thread::sleep(Duration::from_secs(8));
///
/// // Stop the metronome
/// stop_global_metronome();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Errors
///
/// Returns an error if the audio device or configuration cannot be obtained, or if there's an issue starting the metronome.
pub fn start_metronome_with_time_signature(
    bpm: f64,
    beats_per_measure: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let metronome = Metronome::new(bpm, Some(beats_per_measure))?;
    metronome.start()
}

/// Creates and starts a metronome that plays for a specific duration.
///
/// This function will block for the specified duration and then automatically stop.
/// Use this when you want a metronome to play for a specific amount of time.
///
/// # Arguments
///
/// * `bpm` - Beats per minute (e.g., 120.0)
/// * `beats_per_measure` - Optional number of beats per measure for accents
/// * `duration_ms` - How long to play the metronome in milliseconds
///
/// # Returns
///
/// Returns `Ok(())` if the metronome played successfully, or an error if audio setup failed.
///
/// # Examples
///
/// ```no_run
/// use metronome_rs::play_metronome_for_duration;
///
/// // Play a simple 120 BPM metronome for 5 seconds
/// play_metronome_for_duration(120.0, None, 5000)?;
///
/// // Play a 3/4 time metronome at 90 BPM for 10 seconds
/// play_metronome_for_duration(90.0, Some(3), 10000)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
/// # Errors
///
/// Returns an error if the audio device or configuration cannot be obtained, or if there's an issue starting the metronome.
pub fn play_metronome_for_duration(
    bpm: f64,
    beats_per_measure: Option<u32>,
    duration_ms: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let metronome = Metronome::new(bpm, beats_per_measure)?;
    metronome.start()?;

    thread::sleep(Duration::from_millis(duration_ms));

    metronome.stop();
    Ok(())
}

/// Creates and starts a custom metronome with full accent configuration that plays indefinitely.
///
/// This provides full control over the metronome's sound characteristics while maintaining
/// the convenience of a high-level helper function.
///
/// # Arguments
///
/// * `bpm` - Beats per minute (e.g., 120.0)
/// * `beats_per_measure` - Optional number of beats per measure for accents
/// * `accent_config` - Custom accent configuration for sound characteristics
///
/// # Returns
///
/// Returns `Ok(())` if the metronome started successfully, or an error if audio setup failed.
///
/// # Examples
///
/// ```no_run
/// use metronome_rs::{start_custom_metronome, AccentConfig, WaveType, stop_global_metronome};
/// use std::{thread, time::Duration};
///
/// // Create a custom configuration with square waves
/// let config = AccentConfig::with_wave_type(WaveType::Square);
///
/// // Start a 4/4 time metronome with custom sounds
/// start_custom_metronome(110.0, Some(4), config)?;
///
/// // Let it play for 6 seconds
/// thread::sleep(Duration::from_secs(6));
///
/// // Stop the metronome
/// stop_global_metronome();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
/// # Errors
///
/// Returns an error if the audio device or configuration cannot be obtained, or if there's an issue starting the metronome.
pub fn start_custom_metronome(
    bpm: f64,
    beats_per_measure: Option<u32>,
    accent_config: AccentConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let metronome = Metronome::new_with_accent(bpm, beats_per_measure, accent_config)?;
    metronome.start()
}

/// Creates and starts a custom metronome that plays for a specific duration.
///
/// This combines the flexibility of custom accent configuration with automatic timing control.
/// The function will block for the specified duration and then automatically stop.
///
/// # Arguments
///
/// * `bpm` - Beats per minute (e.g., 120.0)
/// * `beats_per_measure` - Optional number of beats per measure for accents
/// * `accent_config` - Custom accent configuration for sound characteristics
/// * `duration_ms` - How long to play the metronome in milliseconds
///
/// # Returns
///
/// Returns `Ok(())` if the metronome played successfully, or an error if audio setup failed.
///
/// # Examples
///
/// ```no_run
/// use metronome_rs::{play_custom_metronome_for_duration, AccentConfig};
///
/// // Play a strong accent metronome for 8 seconds
/// let strong_config = AccentConfig::strong();
/// play_custom_metronome_for_duration(100.0, Some(4), strong_config, 8000)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
/// # Errors
///
/// Returns an error if the audio device or configuration cannot be obtained, or if there's an issue starting the metronome.
pub fn play_custom_metronome_for_duration(
    bpm: f64,
    beats_per_measure: Option<u32>,
    accent_config: AccentConfig,
    duration_ms: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let metronome = Metronome::new_with_accent(bpm, beats_per_measure, accent_config)?;
    metronome.start()?;

    thread::sleep(Duration::from_millis(duration_ms));

    metronome.stop();
    Ok(())
}

/// Preset helper: Creates and starts a practice metronome with subtle accents.
///
/// This is optimized for music practice with gentle accents that won't be distracting.
/// Uses the subtle accent configuration by default.
///
/// # Arguments
///
/// * `bpm` - Beats per minute (e.g., 120.0)
/// * `beats_per_measure` - Number of beats per measure (e.g., 4 for 4/4 time)
///
/// # Examples
///
/// ```no_run
/// use metronome_rs::{start_practice_metronome, stop_global_metronome};
/// use std::{thread, time::Duration};
///
/// // Start a gentle practice metronome in 4/4 time
/// start_practice_metronome(80.0, 4)?;
///
/// // Practice for 2 minutes
/// thread::sleep(Duration::from_secs(120));
///
/// stop_global_metronome();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
/// # Errors
///
/// Returns an error if the audio device or configuration cannot be obtained, or if there's an issue starting the metronome.
pub fn start_practice_metronome(
    bpm: f64,
    beats_per_measure: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let metronome =
        Metronome::new_with_accent(bpm, Some(beats_per_measure), AccentConfig::subtle())?;
    metronome.start()
}

/// Preset helper: Creates and starts a performance metronome with strong accents.
///
/// This is optimized for live performance with clear, pronounced accents.
/// Uses the strong accent configuration by default.
///
/// # Arguments
///
/// * `bpm` - Beats per minute (e.g., 120.0)
/// * `beats_per_measure` - Number of beats per measure (e.g., 4 for 4/4 time)
///
/// # Examples
///
/// ```no_run
/// use metronome_rs::{start_performance_metronome, stop_global_metronome};
/// use std::{thread, time::Duration};
///
/// // Start a clear performance metronome in 3/4 time (waltz)
/// start_performance_metronome(120.0, 3)?;
///
/// // Perform for 3 minutes
/// thread::sleep(Duration::from_secs(180));
///
/// stop_global_metronome();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
/// # Errors
///
/// Returns an error if the audio device or configuration cannot be obtained, or if there's an issue starting the metronome.
pub fn start_performance_metronome(
    bpm: f64,
    beats_per_measure: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let metronome =
        Metronome::new_with_accent(bpm, Some(beats_per_measure), AccentConfig::strong())?;
    metronome.start()
}

/// Creates and starts a metronome with eighth note subdivisions.
///
/// This provides 2 subdivisions per beat, useful for practicing eighth note patterns.
///
/// # Arguments
///
/// * `bpm` - Beats per minute (e.g., 120.0)
/// * `beats_per_measure` - Optional number of beats per measure for accents
///
/// # Examples
///
/// ```no_run
/// use metronome_rs::{start_metronome_with_eighth_notes, stop_global_metronome};
/// use std::{thread, time::Duration};
///
/// // Start a metronome with eighth note subdivisions
/// start_metronome_with_eighth_notes(100.0, Some(4))?;
///
/// // Practice for 30 seconds
/// thread::sleep(Duration::from_secs(30));
///
/// stop_global_metronome();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
/// # Errors
///
/// Returns an error if the audio device or configuration cannot be obtained, or if there's an issue starting the metronome.
pub fn start_metronome_with_eighth_notes(
    bpm: f64,
    beats_per_measure: Option<u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let metronome =
        Metronome::new_with_accent(bpm, beats_per_measure, AccentConfig::with_eighth_notes())?;
    metronome.start()
}

/// Creates and starts a metronome with sixteenth note subdivisions.
///
/// This provides 4 subdivisions per beat, useful for practicing sixteenth note patterns.
///
/// # Arguments
///
/// * `bpm` - Beats per minute (e.g., 120.0)
/// * `beats_per_measure` - Optional number of beats per measure for accents
///
/// # Examples
///
/// ```no_run
/// use metronome_rs::{start_metronome_with_sixteenth_notes, stop_global_metronome};
/// use std::{thread, time::Duration};
///
/// // Start a metronome with sixteenth note subdivisions
/// start_metronome_with_sixteenth_notes(80.0, Some(4))?;
///
/// // Practice for 45 seconds
/// thread::sleep(Duration::from_secs(45));
///
/// stop_global_metronome();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
/// # Errors
///
/// Returns an error if the audio device or configuration cannot be obtained, or if there's an issue starting the metronome.
pub fn start_metronome_with_sixteenth_notes(
    bpm: f64,
    beats_per_measure: Option<u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let metronome =
        Metronome::new_with_accent(bpm, beats_per_measure, AccentConfig::with_sixteenth_notes())?;
    metronome.start()
}

/// Creates and starts a metronome with triplet subdivisions.
///
/// This provides 3 subdivisions per beat, useful for practicing triplet patterns.
///
/// # Arguments
///
/// * `bpm` - Beats per minute (e.g., 120.0)
/// * `beats_per_measure` - Optional number of beats per measure for accents
///
/// # Examples
///
/// ```no_run
/// use metronome_rs::{start_metronome_with_triplets, stop_global_metronome};
/// use std::{thread, time::Duration};
///
/// // Start a metronome with triplet subdivisions
/// start_metronome_with_triplets(90.0, Some(4))?;
///
/// // Practice triplets for 60 seconds
/// thread::sleep(Duration::from_secs(60));
///
/// stop_global_metronome();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
/// # Errors
///
/// Returns an error if the audio device or configuration cannot be obtained, or if there's an issue starting the metronome.
pub fn start_metronome_with_triplets(
    bpm: f64,
    beats_per_measure: Option<u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let metronome =
        Metronome::new_with_accent(bpm, beats_per_measure, AccentConfig::with_triplets())?;
    metronome.start()
}

/// Creates and starts a metronome with custom subdivisions.
///
/// This allows you to specify any number of subdivisions per beat.
///
/// # Arguments
///
/// * `bpm` - Beats per minute (e.g., 120.0)
/// * `beats_per_measure` - Optional number of beats per measure for accents
/// * `subdivisions` - Number of subdivisions per beat
/// * `subdivision_volume` - Volume for subdivisions (0.0 to 1.0)
///
/// # Examples
///
/// ```no_run
/// use metronome_rs::{start_metronome_with_subdivisions, stop_global_metronome};
/// use std::{thread, time::Duration};
///
/// // Start a metronome with 5 subdivisions per beat (quintuplets)
/// start_metronome_with_subdivisions(100.0, Some(4), 5, 0.3)?;
///
/// // Practice for 30 seconds
/// thread::sleep(Duration::from_secs(30));
///
/// stop_global_metronome();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
/// # Errors
///
/// Returns an error if the audio device or configuration cannot be obtained, or if there's an issue starting the metronome.
pub fn start_metronome_with_subdivisions(
    bpm: f64,
    beats_per_measure: Option<u32>,
    subdivisions: u32,
    subdivision_volume: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = AccentConfig::with_custom_subdivisions(subdivisions, 330.0, subdivision_volume);
    let metronome = Metronome::new_with_accent(bpm, beats_per_measure, config)?;
    metronome.start()
}
