# Life-Plus

"Enhanced" (not yet) version of Conway's Game of Life for the Web (not yet).

## Running

To run the game, you need to have Rust installed and to open a terminal and set the current directory to life-plus and execute `cargo run`.

The file ./settings/settings.toml allows for set up of different parameters of the simulation, such as colors and speed.

## Features (Under Development)

Below is the list of features and the implementation plan.

- [ ] Default Conway's Game of Life (v0.1.0)
- [ ] Custom Set Up (v0.2.0):
  - [ ] Rules' Values
  - [ ] Out of Bounds Cell Behavior
    - [ ] Boundaries Loop
  - [ ] Cell Colors:
    - [ ] Constant
    - [ ] Palette
    - [ ] Custom List
    - [ ] Full Random
  - [ ] Grid Colors:
    - [ ] Constant
    - [ ] Palette
    - [ ] Custom List
    - [ ] Full Random
  - [ ] Background Image
- [ ] User Interactive Cell Spawning (v0.3.0)
- [ ] Alternative Cell Shapes (v1.0.0):
  - [ ] Hexagonal
  - [ ] Triangular
  - [ ] Pentagonal
- [ ] Grid 3D Projections (v1.1.0):
  - [ ] Spherical
  - [ ] Toroidal/Donut
  - [ ] Conic
  - [ ] Cylindrical
  - [ ] Polyhedral (Check Feasibility)
- [ ] World Regions (v1.2.0)
- [ ] Audio Output Capture (v1.3.0):
  - [ ] Grid Coloring
  - [ ] Background Coloring
  - [ ] Cell Coloring
  - [ ] Cell Spawning
