# Overview

The Rust Seamless Texture Generator

   - Sorry, I never came up with a clever name for it.


This project generates a seamlessly tileable png formatted texture image generated through OpenSimplex2 noise.


There are controls built in settings for adjusting various things, including:

   - OpenSimplex2 variant (Fast or Smooth)

   - Seed

   - Frequency

   - Domain warp, with adjustable settings

   - Fractaling, with adjustable settings

   - Tiling factor for previewing


There is a lso a live updated preview of your texture with a button for saving as a png file.


[Software Demo Video](https://drive.google.com/file/d/1buHhlaFhnXLRz1ZazmseEVRomMBM00jZ/view?usp=sharing)

# Development Environment

Written using VSCode

Written in Rust with the following packages:
eframe v0.37.0
egui v0.37.0
image v0.25.10
opensimplex2 v1.1.0
rfd v0.18.0

# Useful Websites

- [opensimplex2 Crate documentation](https://docs.rs/opensimplex2/latest/opensimplex2/)
- [eframe Crate documentation](https://docs.rs/eframe/latest/eframe/)
- [egui Crate documentation](https://docs.rs/egui/latest/egui/)

# Future Work

- Toggle light mode for UI
- More options for adjusting noise
- Maybe options to apply a color mask, but this is probably handled elsewhere with exported texture.