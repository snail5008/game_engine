# (unnamed) Game Engine

A toy project that I am currently working on. I have some amount of experience with OpenGL and graphics programming, however I haven't built a real project before, just in general. Thus, I am attempting to use my knowledge of OpenGL and C to write a half-decent game engine that I could then perhaps use for game jams or similar. I am also doing this to help internalise my git knowledge because that is an area that I haven't really worked on at all.

# Features?

Some features that ideally I would like to implement (tick off as completed, note that this list is very incomplete and I will continue to work on it):
* [ ] window module
    * [X] ~~*use GLFW to initialise and destroy windows*~~ [2023-04-14]
    * [X] ~~*allow the user to select when the frame begins and ends in their script (i.e. don't call ready, update, terminate, etc. functions, have a default 'main.c' file that has a 'game_main()' function that 'entry.c' calls. This allows the user more freedom)*~~ [2023-04-14]
    * [X] ~~*handle mouse movement, and give the mouse position to the user as a pixel value (i.e. for a window that is 800 by 600 give a vector that is (0..800, 0.600))*~~ [2023-04-15]
    * [X] ~~*also implement a function that gives the results between 0 and 1 (aka vec_div(mouse_pos, (vec2){800, 600}) or whatever the syntax will be)*~~ [2023-04-15]
    * [X] ~~*allow the user to request the acceleration of the mouse between two frames. This will naturally be framrate dependant though, so while testing, *make sure that applications function correctly on lower & higher framerates*!*~~ [2023-04-15]
    * [X] ~~*implement mouse button handling, return a uint32_t (32 buttons may be overkill, but why not? I guess they will also just be more efficient on most platforms than using a uint8_t and have the added benefit of being scalable past 8 buttons) that can be or'd with some #defined constants (or real constants I guess) such as MOUSE_LEFT, MOUSE_RIGHT, MOUSE_MIDDLE, etc.*~~ [2023-04-15]
    * [X] ~~*implement mouse scroll wheel input*~~ [2023-04-16]
    * [ ] implement keyboard inputs
    * [X] ~~*functions to change window width, height, and title*~~ [2023-04-15]
    * [X] ~~*functions to get window width, height, and title*~~ [2023-04-15]

* [ ] logging library
    * [ ] allow user to select stream(s) to output messages to. For example, stderr and a file.
    * [ ] implement the ability to select different warning levels
    * [ ] ability to turn off (debug) logging
    * [ ] have a distiction between debug and release logging
    * [ ] in the future: when the ui of the engine is implemented, have a logging window that is a default stream for logging

* [ ] vector library
    * [ ] vec2, 3, and 4 unions with x,y, x,y,z, and x,y,z,w structs inside of them as well as a data[2,3,4] field and others that makes sense (such as u,v, r,g,b, r,g,b,a, s,t, etc.)
    * [ ] mathematical functions (add, sub, div, mul)
    * [ ] function for getting the magnitude of a vector
    * [ ] normalising vectors (this just means making the magnitude of a vector 1 while keeping its direction the same). This involves sqrts, so there may be more efficient algorithms to look into? It doesn't really matter though, premature optimisation = bad

* [ ] mesh module
    * [ ] 

* [ ] UI library (in the future, use this to create an interface for the engine itself)
    * [ ] text rendering
    * [ ] buttons
    * [ ] text boxes
    * [ ] input boxes
    * [ ] structs that contain styling information
