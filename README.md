# Rust Rover
A rover written in Rust driven by an STM32.

## Code Plan
The control logic is in its own library, while the hooks that attach that logic
to the hardware will be in the firmware section. Eventually there will also be
another crate added to control the rover from another computer.

## Roadmap
- [ ] Control Logic
- [ ] Firmware Prototype
- [ ] Circuits Tested
- [ ] Final Hardware Assembled
- [ ] Desktop Control Software

## Parts Used
* STM32F103C8 Microcontroller, a.k.a. the "BluePill"
* L293D Dual H-Bridge Motor Controller
* Unihobby Hobbiest DC Motors & Wheels
* 3D Printed Chasis (Probably, Not Yet Designed)
