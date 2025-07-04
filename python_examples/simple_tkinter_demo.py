#!/usr/bin/env python3
"""
Minimal Tkinter metronome demo - just start/stop functionality
"""

import tkinter as tk
from tkinter import ttk, messagebox
import metronome_rs

class SimpleMetronomeApp:
    def __init__(self, root):
        self.root = root
        self.root.title("Simple Metronome")
        self.root.geometry("300x200")
        self.is_playing = False
        
        self.setup_ui()
    
    def setup_ui(self):
        # Main frame with padding
        frame = ttk.Frame(self.root, padding="20")
        frame.pack(fill=tk.BOTH, expand=True)
        
        # Title
        ttk.Label(frame, text="Simple Metronome", 
                 font=("Arial", 14, "bold")).pack(pady=(0, 20))
        
        # BPM input
        bpm_frame = ttk.Frame(frame)
        bpm_frame.pack(pady=10)
        ttk.Label(bpm_frame, text="BPM:").pack(side=tk.LEFT)
        self.bpm_var = tk.StringVar(value="120")
        ttk.Entry(bpm_frame, textvariable=self.bpm_var, width=8).pack(side=tk.LEFT, padx=(5, 0))
        
        # Start/Stop button
        self.btn = ttk.Button(frame, text="Start Metronome", 
                             command=self.toggle_metronome)
        self.btn.pack(pady=20)
        
        # Status
        self.status = tk.StringVar(value="Stopped")
        ttk.Label(frame, textvariable=self.status).pack(pady=10)
    
    def toggle_metronome(self):
        if not self.is_playing:
            try:
                bpm = float(self.bpm_var.get())
                metronome_rs.py_start_simple_metronome(bpm)
                self.is_playing = True
                self.btn.config(text="Stop Metronome")
                self.status.set(f"Playing at {bpm} BPM")
            except ValueError:
                messagebox.showerror("Error", "Please enter a valid BPM number")
            except Exception as e:
                messagebox.showerror("Error", f"Failed to start: {e}")
        else:
            metronome_rs.py_stop_global_metronome()
            self.is_playing = False
            self.btn.config(text="Start Metronome")
            self.status.set("Stopped")
    
    def on_closing(self):
        if self.is_playing:
            metronome_rs.py_stop_global_metronome()
        self.root.destroy()

if __name__ == "__main__":
    root = tk.Tk()
    app = SimpleMetronomeApp(root)
    root.protocol("WM_DELETE_WINDOW", app.on_closing)
    root.mainloop()
