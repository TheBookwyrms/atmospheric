contains various layers of abstraction for OpenGL, from no abstractions on the raw commands, up to a much higher level renderer that uses GLFW to manage the window and user-interaction events

TO DO
- [x] add "Custom" shader program option, in addition to pre-written ones
- [ ] write documentation
- [ ] write examples for all levels of abstraction
- [ ] object structs
- [ ] fix blinn-phong lighting
- [ ] ppm viewer (as textures?)
    - [x] use PPM struct for ppm image textures
- [ ] render to textures
- [ ] save renders as image (from textures?)
- [ ] gui stuff
- [ ] convolution kernel postprocessing
- [x] fix camera viewing position at correct point, without seeing behind camera
    - [ ] harmonize all camera and matrix code together
    - [ ] neaten camera viewing code
    - [ ] add camera enums for ease of use
- [ ] text rendering


project ideas
- [ ] recreate all old opengl testing projects in rust
- [ ] gravity in shaders
- [ ] wave simulator
- [ ] matrix structure analysis (statics/frames + trusses/loadings)
    - internal forces and stresses
    - shear force
    - bending moment
    - visualise it in 3D
- [ ] doppler effect simulator and other cool stuff (trains?)
- [ ] cool stuff with parametric curves