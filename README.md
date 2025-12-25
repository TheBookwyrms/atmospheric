contains various layers of abstraction for OpenGL, from no abstractions on the raw commands, up to a much higher level renderer that uses GLFW to manage the window and user-interaction events

TO DO
- add "Custom" shader program option, in addition to pre-written ones
- write documentation
- write examples for all levels of abstraction
- object structs
- gui stuff
- ppm viewer (as textures?)
- render to textures
- save renders as image (from textures?)
- convolution kernel postprocessing
- fix camera viewing position at correct point, without seeing behind camera
- text rendering

project ideas
- recreate all old opengl testing projects in rust
- gravity in shaders
- wave simulator
- matrix structure analysis (statics/frames + trusses/loadings)
    - internal forces and stresses
    - shear force
    - bending moment
    - visualise it in 3D