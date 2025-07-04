#!/usr/bin/env python3
"""
Simple Tkinter GUI demo for metronome-rs

This demonstrates basic start/stop functionality with a simple GUI.
"""

import tkinter as tk
from tkinter import ttk, messagebox
import metronome_rs
import threading
import time


class MetronomeApp:
    def __init__(self, root):
        self.root = root
        self.root.title("Metronome-RS Demo")
        self.root.geometry("400x300")
        self.root.resizable(False, False)

        # State variables
        self.is_playing = False

        self.setup_ui()

    def setup_ui(self):
        """Setup the user interface"""
        # Main frame
        main_frame = ttk.Frame(self.root, padding="20")
        main_frame.grid(row=0, column=0, sticky=(tk.W, tk.E, tk.N, tk.S))

        # Title
        title_label = ttk.Label(
            main_frame, text="Metronome-RS Demo", font=("Arial", 16, "bold")
        )
        title_label.grid(row=0, column=0, columnspan=2, pady=(0, 20))

        # BPM input
        ttk.Label(main_frame, text="BPM:").grid(row=1, column=0, sticky=tk.W, pady=5)
        self.bpm_var = tk.StringVar(value="120")
        self.bpm_entry = ttk.Entry(main_frame, textvariable=self.bpm_var, width=10)
        self.bpm_entry.grid(row=1, column=1, sticky=tk.W, pady=5)

        # Time signature input
        ttk.Label(main_frame, text="Time Signature:").grid(
            row=2, column=0, sticky=tk.W, pady=5
        )
        self.time_sig_var = tk.StringVar(value="4")
        self.time_sig_entry = ttk.Entry(
            main_frame, textvariable=self.time_sig_var, width=10
        )
        self.time_sig_entry.grid(row=2, column=1, sticky=tk.W, pady=5)

        # Metronome type selection
        ttk.Label(main_frame, text="Type:").grid(row=3, column=0, sticky=tk.W, pady=5)
        self.metronome_type = tk.StringVar(value="Simple")
        type_combo = ttk.Combobox(
            main_frame,
            textvariable=self.metronome_type,
            values=["Simple", "With Accents", "Practice", "Performance"],
            state="readonly",
            width=15,
        )
        type_combo.grid(row=3, column=1, sticky=tk.W, pady=5)

        # Control buttons frame
        button_frame = ttk.Frame(main_frame)
        button_frame.grid(row=4, column=0, columnspan=2, pady=20)

        # Start/Stop button
        self.start_stop_btn = ttk.Button(
            button_frame,
            text="Start",
            command=self.toggle_metronome,
            style="Accent.TButton",
        )
        self.start_stop_btn.pack(side=tk.LEFT, padx=5)

        # Test beep button
        test_btn = ttk.Button(button_frame, text="Test Beep", command=self.test_beep)
        test_btn.pack(side=tk.LEFT, padx=5)

        # Status label
        self.status_var = tk.StringVar(value="Ready")
        self.status_label = ttk.Label(
            main_frame, textvariable=self.status_var, font=("Arial", 10)
        )
        self.status_label.grid(row=5, column=0, columnspan=2, pady=10)

        # Instructions
        instructions = tk.Text(main_frame, height=6, width=45, wrap=tk.WORD)
        instructions.grid(row=6, column=0, columnspan=2, pady=10)
        instructions.insert(
            "1.0",
            "Instructions:\n"
            "• Set your desired BPM (beats per minute)\n"
            "• Choose time signature (beats per measure)\n"
            "• Select metronome type\n"
            "• Click 'Start' to begin, 'Stop' to end\n"
            "• Use 'Test Beep' to check audio output",
        )
        instructions.config(state=tk.DISABLED)

    def toggle_metronome(self):
        """Start or stop the metronome"""
        if not self.is_playing:
            self.start_metronome()
        else:
            self.stop_metronome()

    def start_metronome(self):
        """Start the metronome"""
        try:
            # Get and validate BPM
            bpm = float(self.bpm_var.get())
            if bpm <= 0 or bpm > 300:
                messagebox.showerror("Error", "BPM must be between 1 and 300")
                return

            # Get time signature (optional for some types)
            time_sig = None
            if self.metronome_type.get() != "Simple":
                try:
                    time_sig = int(self.time_sig_var.get())
                    if time_sig <= 0 or time_sig > 16:
                        messagebox.showerror(
                            "Error", "Time signature must be between 1 and 16"
                        )
                        return
                except ValueError:
                    messagebox.showerror("Error", "Time signature must be a number")
                    return

            # Start the appropriate metronome type
            metronome_type = self.metronome_type.get()

            if metronome_type == "Simple":
                metronome_rs.py_start_simple_metronome(bpm)
            elif metronome_type == "With Accents":
                metronome_rs.py_start_metronome_with_time_signature(bpm, time_sig)
            elif metronome_type == "Practice":
                metronome_rs.py_start_practice_metronome(bpm, time_sig)
            elif metronome_type == "Performance":
                metronome_rs.py_start_performance_metronome(bpm, time_sig)

            # Update UI state
            self.is_playing = True
            self.start_stop_btn.config(text="Stop")
            self.status_var.set(f"Playing: {bpm} BPM")

            # Disable input fields while playing
            self.bpm_entry.config(state="disabled")
            self.time_sig_entry.config(state="disabled")

        except ValueError:
            messagebox.showerror("Error", "BPM must be a valid number")
        except Exception as e:
            messagebox.showerror("Error", f"Failed to start metronome: {str(e)}")

    def stop_metronome(self):
        """Stop the metronome"""
        try:
            metronome_rs.py_stop_global_metronome()

            # Update UI state
            self.is_playing = False
            self.start_stop_btn.config(text="Start")
            self.status_var.set("Stopped")

            # Re-enable input fields
            self.bpm_entry.config(state="normal")
            self.time_sig_entry.config(state="normal")

        except Exception as e:
            messagebox.showerror("Error", f"Failed to stop metronome: {str(e)}")

    def test_beep(self):
        """Play a test beep to verify audio output"""

        def beep_in_thread():
            try:
                metronome_rs.py_beep()
            except Exception as e:
                # Schedule the error message to run in the main thread
                self.root.after(
                    0,
                    lambda: messagebox.showerror(
                        "Error", f"Failed to play beep: {str(e)}"
                    ),
                )

        # Run beep in separate thread to avoid blocking UI
        threading.Thread(target=beep_in_thread, daemon=True).start()

    def on_closing(self):
        """Handle window closing"""
        if self.is_playing:
            metronome_rs.py_stop_global_metronome()
        self.root.destroy()


def main():
    """Main application entry point"""
    try:
        # Test if metronome_rs is available
        import metronome_rs
    except ImportError:
        print("Error: metronome_rs module not found!")
        print("Please install it first with: maturin develop --features python")
        return

    # Create and run the application
    root = tk.Tk()
    app = MetronomeApp(root)

    # Handle window closing
    root.protocol("WM_DELETE_WINDOW", app.on_closing)

    # Center the window
    root.update_idletasks()
    x = (root.winfo_screenwidth() // 2) - (root.winfo_width() // 2)
    y = (root.winfo_screenheight() // 2) - (root.winfo_height() // 2)
    root.geometry(f"+{x}+{y}")

    # Start the GUI
    root.mainloop()


if __name__ == "__main__":
    main()
