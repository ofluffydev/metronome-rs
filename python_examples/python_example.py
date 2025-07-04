#!/usr/bin/env python3
"""
Example usage of metronome-rs Python bindings

This example demonstrates how to use the metronome library from Python.
"""

import time
import metronome_rs

def main():
    print("Metronome-RS Python Bindings Demo")
    print("=" * 40)
    
    # Test basic beep functionality
    print("\n1. Playing a simple beep...")
    metronome_rs.py_beep()
    
    print("\n2. Playing a 880Hz beep...")
    metronome_rs.py_beep_frequency(880.0)
    
    # Test simple metronome
    print("\n3. Starting a simple 120 BPM metronome for 3 seconds...")
    metronome_rs.py_start_simple_metronome(120.0)
    time.sleep(3)
    metronome_rs.py_stop_global_metronome()
    
    # Test metronome with time signature
    print("\n4. Starting a 4/4 metronome at 100 BPM for 4 seconds...")
    metronome_rs.py_start_metronome_with_time_signature(100.0, 4)
    time.sleep(4)
    metronome_rs.py_stop_global_metronome()
    
    # Test custom accent configuration
    print("\n5. Creating custom accent configuration...")
    wave_type = metronome_rs.PyWaveType.square()
    config = metronome_rs.PyAccentConfig.with_wave_type(wave_type)
    print(f"Config: {config}")
    
    print("Starting custom metronome with square waves for 3 seconds...")
    metronome_rs.py_start_custom_metronome(90.0, 3, config)
    time.sleep(3)
    metronome_rs.py_stop_global_metronome()
    
    # Test subdivisions
    print("\n6. Testing metronome with eighth note subdivisions for 4 seconds...")
    metronome_rs.py_start_metronome_with_eighth_notes(80.0, 4)
    time.sleep(4)
    metronome_rs.py_stop_global_metronome()
    
    # Test timed metronome (non-blocking example)
    print("\n7. Playing a practice metronome for exactly 2 seconds...")
    metronome_rs.py_play_metronome_for_duration(110.0, 4, 2000)
    print("Done!")
    
    # Test preset configurations
    print("\n8. Testing different accent presets...")
    
    # Subtle config
    subtle_config = metronome_rs.PyAccentConfig.subtle()
    print("Playing subtle accent metronome...")
    metronome_rs.py_play_custom_metronome_for_duration(100.0, 4, subtle_config, 2000)
    
    # Strong config
    strong_config = metronome_rs.PyAccentConfig.strong()
    print("Playing strong accent metronome...")
    metronome_rs.py_play_custom_metronome_for_duration(100.0, 4, strong_config, 2000)
    
    # Test wave types
    print("\n9. Testing different wave types...")
    for wave_name in ["sine", "square", "triangle", "sawtooth"]:
        print(f"Playing {wave_name} wave metronome...")
        wave_type = metronome_rs.PyWaveType(wave_name)
        config = metronome_rs.PyAccentConfig.with_wave_type(wave_type)
        metronome_rs.py_play_custom_metronome_for_duration(120.0, None, config, 1500)
    
    print("\nDemo completed!")

if __name__ == "__main__":
    main()
