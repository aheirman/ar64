# AR64
Aron's RISCV-64 sim

This project contains a multicore RISCV simulator that can show the current processor state via a webpage.
There are 2 servers behind the scenes, one server written in rust that runs the actual simulation and a javascript node.js server that serves the webpage written in svelte. 

![web view of the debugger](https://raw.githubusercontent.com/aheirman/ar64/refs/heads/main/ar64_web/gui.png)

The GUI uses the instruction decoder from https://luplab.gitlab.io/rvcodecjs/ .
This allows the GUI (not the engine) to show what each instruction should do.