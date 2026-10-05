contains various layers of abstraction for OpenGL, from no abstractions on the raw commands, up to a much higher level renderer that uses GLFW to manage the window and user-interaction events

TO DO
- [x] add "Custom" shader program option, in addition to pre-written ones
- [ ] write documentation
- [x] object structs
- [x] fix blinn-phong lighting
    - [x] figure out multi-light scenes for N lights
    - [ ] implement the normal matrix for blinn-phong lighting (and just in general)
- [ ] ppm viewer (as textures?)
    - [x] use PPM struct for ppm image textures
- [ ] render to textures
- [x] save renders as image
    - [ ] from textures
- [ ] gui stuff
- [ ] convolution kernel postprocessing
- [x] fix camera viewing position at correct point, without seeing behind camera
    - [x] harmonize all camera and matrix code together
    - [x] neaten camera viewing code
    - [x] add camera enums for ease of use
- [ ] text rendering
- [ ] add Module system to allow for pre-packaged settings such as Shaders, Camera pre-sets, and others
    - [x] move glfw window keybindings into individual functions (modules)
- [x] test out whether multiple shader programs can be used sequentially without overriding each other
    - [ ] can i use multiple shader calls, at multiple (concurrent or different) positions, and see all of the results simultaneously, without conflicts
- [x] migrate things from rendering_stuff_rs to atmospheric