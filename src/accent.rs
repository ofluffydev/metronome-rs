/// Wave types available for metronome sounds.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum WaveType {
    /// Sine wave - smooth, pure tone
    #[default]
    Sine,
    /// Square wave - harsh, digital sound
    Square,
    /// Sawtooth wave - bright, buzzy tone
    Sawtooth,
    /// Triangle wave - softer than square, warmer than sine
    Triangle,
}

/// Configuration for accent beats in the metronome.
#[derive(Clone, Debug)]
pub struct AccentConfig {
    /// Frequency for accented beats (Hz)
    pub accent_frequency: f32,
    /// Frequency for regular beats (Hz)
    pub regular_frequency: f32,
    /// Duration for accented beats (ms)
    pub accent_duration: u64,
    /// Duration for regular beats (ms)
    pub regular_duration: u64,
    /// Wave type for accented beats
    pub accent_wave_type: WaveType,
    /// Wave type for regular beats
    pub regular_wave_type: WaveType,
    /// Number of subdivisions per beat (e.g., 2 for eighth notes, 4 for sixteenth notes)
    pub subdivisions: u32,
    /// Frequency for subdivision clicks (Hz)
    pub subdivision_frequency: f32,
    /// Duration for subdivision clicks (ms)
    pub subdivision_duration: u64,
    /// Wave type for subdivision clicks
    pub subdivision_wave_type: WaveType,
    /// Volume multiplier for subdivisions (0.0 to 1.0, where 1.0 is same volume as regular beats)
    pub subdivision_volume: f32,
}

impl Default for AccentConfig {
    fn default() -> Self {
        Self {
            accent_frequency: 880.0,  // A5 note (one octave higher than A4)
            regular_frequency: 440.0, // A4 note
            accent_duration: 150,     // Longer accent
            regular_duration: 100,    // Shorter regular beat
            accent_wave_type: WaveType::Sine,
            regular_wave_type: WaveType::Sine,
            subdivisions: 1,               // No subdivisions by default
            subdivision_frequency: 523.25, // C5 note (more audible than E4)
            subdivision_duration: 80,      // Longer subdivision clicks for better audibility
            subdivision_wave_type: WaveType::Sine,
            subdivision_volume: 0.7, // Higher volume for subdivisions
        }
    }
}

impl AccentConfig {
    /// Creates a new accent configuration with basic values (no subdivisions).
    pub fn new(
        accent_frequency: f32,
        regular_frequency: f32,
        accent_duration: u64,
        regular_duration: u64,
        accent_wave_type: WaveType,
        regular_wave_type: WaveType,
    ) -> Self {
        Self {
            accent_frequency,
            regular_frequency,
            accent_duration,
            regular_duration,
            accent_wave_type,
            regular_wave_type,
            subdivisions: 1,
            subdivision_frequency: 523.25,
            subdivision_duration: 80,
            subdivision_wave_type: WaveType::Sine,
            subdivision_volume: 0.7,
        }
    }

    /// Creates a new accent configuration with subdivision support.
    pub fn with_subdivisions(
        accent_frequency: f32,
        regular_frequency: f32,
        subdivisions: u32,
        subdivision_frequency: f32,
    ) -> Self {
        Self {
            accent_frequency,
            regular_frequency,
            accent_duration: 150,
            regular_duration: 100,
            accent_wave_type: WaveType::Sine,
            regular_wave_type: WaveType::Sine,
            subdivisions,
            subdivision_frequency,
            subdivision_duration: 70, // Longer duration for better audibility
            subdivision_wave_type: WaveType::Sine,
            subdivision_volume: 0.65, // Higher volume
        }
    }

    /// Creates an accent configuration with a more subtle difference.
    pub fn subtle() -> Self {
        Self {
            accent_frequency: 660.0,  // E5 note (less dramatic than octave)
            regular_frequency: 440.0, // A4 note
            accent_duration: 120,     // Slightly longer
            regular_duration: 100,    // Regular duration
            accent_wave_type: WaveType::Sine,
            regular_wave_type: WaveType::Sine,
            subdivisions: 1, // No subdivisions by default
            subdivision_frequency: 523.25,
            subdivision_duration: 80,
            subdivision_wave_type: WaveType::Sine,
            subdivision_volume: 0.7,
        }
    }

    /// Creates an accent configuration with a very pronounced difference.
    pub fn strong() -> Self {
        Self {
            accent_frequency: 1760.0, // A6 note (two octaves higher)
            regular_frequency: 440.0, // A4 note
            accent_duration: 200,     // Much longer
            regular_duration: 80,     // Shorter regular beats
            accent_wave_type: WaveType::Sine,
            regular_wave_type: WaveType::Sine,
            subdivisions: 1, // No subdivisions by default
            subdivision_frequency: 523.25,
            subdivision_duration: 80,
            subdivision_wave_type: WaveType::Sine,
            subdivision_volume: 0.7,
        }
    }

    /// Creates an accent configuration with different wave types for accent and regular beats.
    pub fn with_wave_types(accent_wave: WaveType, regular_wave: WaveType) -> Self {
        Self {
            accent_frequency: 880.0,
            regular_frequency: 440.0,
            accent_duration: 150,
            regular_duration: 100,
            accent_wave_type: accent_wave,
            regular_wave_type: regular_wave,
            subdivisions: 1, // No subdivisions by default
            subdivision_frequency: 523.25,
            subdivision_duration: 80,
            subdivision_wave_type: WaveType::Sine,
            subdivision_volume: 0.7,
        }
    }

    /// Creates an accent configuration where both accent and regular beats use the same wave type.
    pub fn with_wave_type(wave_type: WaveType) -> Self {
        Self {
            accent_frequency: 880.0,
            regular_frequency: 440.0,
            accent_duration: 150,
            regular_duration: 100,
            accent_wave_type: wave_type.clone(),
            regular_wave_type: wave_type,
            subdivisions: 1, // No subdivisions by default
            subdivision_frequency: 523.25,
            subdivision_duration: 80,
            subdivision_wave_type: WaveType::Sine,
            subdivision_volume: 0.7,
        }
    }

    /// Creates a strong accent configuration with square waves for a more pronounced effect.
    pub fn strong_square() -> Self {
        Self {
            accent_frequency: 1760.0,
            regular_frequency: 440.0,
            accent_duration: 200,
            regular_duration: 80,
            accent_wave_type: WaveType::Square,
            regular_wave_type: WaveType::Sine,
            subdivisions: 1, // No subdivisions by default
            subdivision_frequency: 523.25,
            subdivision_duration: 80,
            subdivision_wave_type: WaveType::Sine,
            subdivision_volume: 0.7,
        }
    }

    /// Creates a subtle accent configuration with triangle waves for a softer sound.
    pub fn subtle_triangle() -> Self {
        Self {
            accent_frequency: 660.0,
            regular_frequency: 440.0,
            accent_duration: 120,
            regular_duration: 100,
            accent_wave_type: WaveType::Triangle,
            regular_wave_type: WaveType::Triangle,
            subdivisions: 1, // No subdivisions by default
            subdivision_frequency: 523.25,
            subdivision_duration: 80,
            subdivision_wave_type: WaveType::Sine,
            subdivision_volume: 0.7,
        }
    }

    // Subdivision-specific presets

    /// Creates a configuration with eighth note subdivisions (2 subdivisions per beat).
    pub fn with_eighth_notes() -> Self {
        Self {
            accent_frequency: 880.0,
            regular_frequency: 440.0,
            accent_duration: 150,
            regular_duration: 100,
            accent_wave_type: WaveType::Sine,
            regular_wave_type: WaveType::Sine,
            subdivisions: 2,               // Eighth notes
            subdivision_frequency: 587.33, // D5 note (clearly audible)
            subdivision_duration: 70,
            subdivision_wave_type: WaveType::Sine,
            subdivision_volume: 0.65, // Good volume for eighth notes
        }
    }

    /// Creates a configuration with sixteenth note subdivisions (4 subdivisions per beat).
    pub fn with_sixteenth_notes() -> Self {
        Self {
            accent_frequency: 880.0,
            regular_frequency: 440.0,
            accent_duration: 150,
            regular_duration: 100,
            accent_wave_type: WaveType::Sine,
            regular_wave_type: WaveType::Sine,
            subdivisions: 4,               // Sixteenth notes
            subdivision_frequency: 659.25, // E5 note (bright and audible)
            subdivision_duration: 80,
            subdivision_wave_type: WaveType::Square, // Square wave is more cutting for fast subdivisions
            subdivision_volume: 0.55,                // Higher volume for sixteenth notes
        }
    }

    /// Creates a configuration with triplet subdivisions (3 subdivisions per beat).
    pub fn with_triplets() -> Self {
        Self {
            accent_frequency: 880.0,
            regular_frequency: 440.0,
            accent_duration: 150,
            regular_duration: 100,
            accent_wave_type: WaveType::Sine,
            regular_wave_type: WaveType::Sine,
            subdivisions: 3,               // Triplets
            subdivision_frequency: 523.25, // C5 note
            subdivision_duration: 65,
            subdivision_wave_type: WaveType::Triangle, // Different wave type for triplets
            subdivision_volume: 0.6,
        }
    }

    /// Creates a custom subdivision configuration.
    pub fn with_custom_subdivisions(
        subdivisions: u32,
        subdivision_frequency: f32,
        subdivision_volume: f32,
    ) -> Self {
        Self {
            accent_frequency: 880.0,
            regular_frequency: 440.0,
            accent_duration: 150,
            regular_duration: 100,
            accent_wave_type: WaveType::Sine,
            regular_wave_type: WaveType::Sine,
            subdivisions,
            subdivision_frequency,
            subdivision_duration: 70, // Better default duration
            subdivision_wave_type: WaveType::Sine,
            subdivision_volume,
        }
    }

    // Builder-style methods for modifying existing configurations

    /// Returns a copy of this configuration with the specified number of subdivisions.
    pub fn set_subdivisions(mut self, subdivisions: u32) -> Self {
        self.subdivisions = subdivisions;
        self
    }

    /// Returns a copy of this configuration with the specified subdivision frequency.
    pub fn set_subdivision_frequency(mut self, frequency: f32) -> Self {
        self.subdivision_frequency = frequency;
        self
    }

    /// Returns a copy of this configuration with the specified subdivision volume.
    pub fn set_subdivision_volume(mut self, volume: f32) -> Self {
        self.subdivision_volume = volume;
        self
    }

    /// Returns a copy of this configuration with the specified subdivision wave type.
    pub fn set_subdivision_wave_type(mut self, wave_type: WaveType) -> Self {
        self.subdivision_wave_type = wave_type;
        self
    }

    // Extra subdivision presets for specific use cases

    /// Creates a configuration optimized for practicing slow pieces with clear subdivisions.
    pub fn practice_subdivisions() -> Self {
        Self {
            accent_frequency: 880.0,
            regular_frequency: 440.0,
            accent_duration: 150,
            regular_duration: 100,
            accent_wave_type: WaveType::Sine,
            regular_wave_type: WaveType::Sine,
            subdivisions: 2,               // Eighth notes for practice
            subdivision_frequency: 587.33, // D5 - clearly audible
            subdivision_duration: 80,
            subdivision_wave_type: WaveType::Sine,
            subdivision_volume: 0.75, // High volume for practice
        }
    }

    /// Creates a configuration optimized for fast technical passages.
    pub fn technical_subdivisions() -> Self {
        Self {
            accent_frequency: 1046.5,  // C6 - very bright accent
            regular_frequency: 523.25, // C5 - bright regular beats
            accent_duration: 120,
            regular_duration: 80,
            accent_wave_type: WaveType::Square,
            regular_wave_type: WaveType::Sine,
            subdivisions: 4,               // Sixteenth notes
            subdivision_frequency: 698.46, // F5 - cutting through
            subdivision_duration: 70,
            subdivision_wave_type: WaveType::Square, // Very clear for fast passages
            subdivision_volume: 0.6,                 // Audible but not overwhelming
        }
    }
}
