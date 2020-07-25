# Block Interaction

This lists the different components of gblite and how they should interact.

### Components

- Board
    - "Main" function for program.
    - Manage all necessary clients, and initialize them.
- CPU
    - Process instructions, reading from memory and CPU registers.
    - Respond to interrupts generated from PPU, or input.
        - This is done by reading the `IF` register
        - `IME` and corresponding `IE` must be set.
- PPU
    - This clock is separate from system clock, and runs in parallel to CPU.
- Memory
- Registers
- IO


## CPU

Clients: Board?
Input: Current register/memory state.
Output: Modify current register/memory state via the current instruction.

# Design Concerns

- Consider `EI`, the spec says that the enable is "delayed by one machine cycle". How can I get
  this working?
- Need to consider a fully cycle-accurate model.
    - Machine delay needs to happen at the right time. If a load takes four machine cycles, then
      the registers should not be changed until four cycles worth of time has passed!
- Communication between CPU/PPU needs to happen across separate threads.
