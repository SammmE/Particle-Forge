# Particle Forge - An accurate and efficient particle simulator

### The plan:
Requirements & Architecture Planning ✅

    Define core functionalities: physics simulation (engine) and user interaction (particle spawning, display, and UI).
    Decide on a clean separation of concerns (e.g., modules or plugins) for the engine and UI.

Project Setup & Dependency Management ✅

    Initialize a new Cargo project and set up version control.
    Add dependencies:
        Bevy for game engine and rendering.
        bevy_egui (or similar) for integrating a UI panel (egui) into the game.

Define Data Structures & Core Types ✅

    Create base data structures for particles (using a common Particle trait or enum to represent Proton, Neutron, and Electron).
    Define properties such as mass, charge, position, velocity, etc., ensuring each particle type can later be extended with more characteristics.

Design the Simulation Engine (Physics Module)

    Plan the engine's core loop for updating particle positions and handling interactions.
    Choose integration methods (e.g., Euler, Verlet) that balance accuracy and performance.
    Structure the module to enable optimization (e.g., leveraging Bevy’s ECS for parallel updates and data-oriented design).

Design the User Interaction Module

    Plan systems for handling user input (keyboard/mouse events) to spawn particles and manipulate the simulation.
    Define how these interactions trigger changes in the simulation state.
    Ensure that this module communicates cleanly with the engine module via events or messages.

Rendering & Visualization Strategy

    Outline how particles will be visually represented using Bevy’s rendering system.
    Decide on graphical representations for different particle types (e.g., color, size, sprite or 3D mesh).
    Plan for efficient rendering by batching and minimizing draw calls where possible.

Integrate UI with Simulation (Egui Integration)

    Design UI panels using egui to display information about each particle (e.g., properties, status).
    Plan the integration so that UI updates in real time as the simulation changes.
    Ensure UI interactions (such as selecting a particle) can communicate with the engine module.

Integration & System Synchronization

    Combine the simulation engine, rendering, and user interaction systems.
    Establish clear data flow and event channels between modules to keep them decoupled yet synchronized.
    Set up a unified update loop that manages simulation steps and rendering cycles.

Performance Optimization & Testing

    Profile the simulation to identify bottlenecks in both the physics calculations and rendering.
    Optimize code by leveraging Rust’s performance features, such as zero-cost abstractions and parallel systems where applicable.
    Write unit and integration tests for the engine’s calculations and the user interaction flows.

Future Extensions & Modular Design

    Refactor modules to be as decoupled as possible (consider using traits, generics, or plugin architectures) to facilitate adding new particle types or simulation features.
    Document the codebase and design decisions to make future development easier and to maintain clarity in how systems interact.