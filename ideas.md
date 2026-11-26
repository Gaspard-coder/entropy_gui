# Permutation Entropy simulator guidelines

## Goal 
GUI and automatization to compute permutation entropy tools

## Requirements
- Computing functions already made in MATLAB
- GUI with desktop app (eventually web also)

Ideally, the GUI is developed in Rust
Hosting: Rust desktop app with background MATLAB's files

## Tools candidates (written in Rust):
- rustmex: bind Rust with Matlab's MEX C api
- ratatui: terminal user interface
- egui: GUI for app and web
- Leptos: web framework
- dioxus: web/desktop/mobile app in one file
- Tauri + Yew: Cross-platform backend + frontend

## Questions for AI
- Is Rust a good language to do this project? It is realistic to link MATLAB with RUST GUI
- What tool is the best (for me it is between egui and dioxus). You can propose others tools if relevant
- What are the good development practises to dev this type of project?
- What are the mandatory sections of the Rust book (https://rust-book.cs.brown.edu/ch06-02-match.html) to achieve this type of project?


# Directory organization
- src/main.rs # Entry point & OS window initialization
- src/app.rs # Host application
- example.rs # UI component

# Requirements
- egui
- MATLAB
- Linux (for now)

# Notes
 matlab -nodesktop -nosplash -r "HXC_standard_deviation(4,1,'/home/gaspichou/Documents/git/DBT2-permutation-entropy/Database/diabet/raw_datasets/DBT2'); quit" => only give the result, not the display
