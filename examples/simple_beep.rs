use metronome_rs::beep;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Playing a simple beep...");
    beep()?;
    println!("Beep complete!");
    Ok(())
}
