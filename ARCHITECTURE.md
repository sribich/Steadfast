# steadfast-engine-core

    1. Booting the game
    2. OS Communication
    3. Game Loop (input - state machine?, update, render)

# steadfast-engine-physics

    1. Collision Detection 
        a. Axis-aligned bounding box collision (No overlap, one-axis overlap, two-axes overlap)
        b. Space partitioning (och-tree)

# steadfast-engine-render

    1. Object Loading
    2. Rendering
        a. Input Asm (Vertex/Index buffers)
        b. Vertex Shader
        c. Raster
        d. Depth Test
        e. Pixel Shading
        f. Depth Test
        g. Render Target Output
    3. Shader Graph (Pixel Shading)
    4. Framerate
    5. VFX (Particles, Game objects (no ai, simple physics))

IO, VFS
Entry Point (In engine or app?)
Application Layer
Window Layer
    - Input
    - Events
Renderer
Render API Abstraction
Debugging - Tracing - Profiling

Animation??? Inverse Kinematics, Cloth, Hair
AI Framework???
    - State machine?
    - Behavior Tree
Audio (fmod, wwise, custom)
    - SFX (Attack, Decay, Sustain, Release) ADSR
    - VO (Voice Over)
    - Music
Tools
Build Pipeline (Local vs Shipped) - Feature Cutdown
Scripting
Networking
    - Replication
        - State (changed hp)
        - Event (something happened, explosion?)
    - Prediction
        - Commonly on movement (Move)
    - Verification
        - One computer checks what another reported to see. Trust it?

OOP vs Data Oriented Design
    - ECS

Engine Heavy vs Engine Light

Linear algebra Library
S

SCENE GRAPH
GameObject, Contains GameComponents

+Some way to insert game into engine
+Better scratchpad support
-Better textures (reading, blitting, basic image processing)
+Some way to separate game and engine data
+Add higher level rendering constructs (Shaders, Materials, Meshes, Transforms)
+Add some form of automatic shader selection
-Add some form of primitives (Rectangles, Spheres)
+More flexible transform class (default values for projection and camera)
=Non-perspective views, (orthogonal views)
-Texture manipulation (Translation, rotation, scale)
-More Control over textures (Filtering, Formatting)
+More friendly constructors
=Window should have more properties (Center position, fullscreen, maybe mouse locking)
=Better vector math (normalization, +=, Commonly used Vector constants, Swizzling support)
-Options class/system, some place to read values that the player chooses
+Centralized "level" or "scene" class that holds the data the game is using
+Game object class
+Make all positioning names "position"
+Make naming more consistent
+Vector swizzling
+Make all time based on seconds rather than nanoseconds
+Change the way delta is passed
+Give vectors a copy constructor
+Give vectors an interpolation method
-Transparency Support
-Sprite Support
+Good way to compare vectors
+Vector2f cross product
-Some way to display text

-Eventually, make easier way to generate mesh
-Eventually, make easy way of generating texture coordinates