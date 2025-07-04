use metronome_rs::{
    beep_frequency, get_default_host, get_default_output_config, get_default_output_device,
    play_beep_with_config_and_params,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Various Beeps Demo");
    println!("This demonstrates different types of beeps and tones.");

    println!("\n1. Standard beep (440Hz)...");
    metronome_rs::beep()?;

    println!("\n2. High frequency beep (880Hz)...");
    beep_frequency(880.0)?;

    println!("\n3. Low frequency beep (220Hz)...");
    beep_frequency(220.0)?;

    println!("\n4. Musical scale (C major)...");
    let frequencies = [
        261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25,
    ]; // C4 to C5
    let note_names = ["C4", "D4", "E4", "F4", "G4", "A4", "B4", "C5"];

    let host = get_default_host();
    let device = get_default_output_device(&host)?;
    let config = get_default_output_config(&device)?;
    let stream_config = config.into();

    for (i, &freq) in frequencies.iter().enumerate() {
        println!("Playing {}... ({:.2} Hz)", note_names[i], freq);
        play_beep_with_config_and_params(&device, &stream_config, freq, 500)?;
        std::thread::sleep(std::time::Duration::from_millis(100)); // Small gap between notes
    }

    println!("\nAll beeps complete!");

    Ok(())
}
