# wivir: a simple tool for Wi-Fi connections in the terminal

The goal was to avoid getting out of my terminal workflow to switch Wi-Fi
networks on-the-fly, and doing so easily and without using the mouse ;).

Vim-like exploration on my workflow is something I try to achieve across all my
tools, so this is my attempt at managing wireless networks said way.

## Status
- Still an idea: nothing yet :/

## Implementation

- Starting for now with the crate `nmrs` to access NetworkManager's API with
  Rust bindings.

- Terminal interface with `ratatui` as the front-end, and Vim-like keybinds.

## Credits:

- [nmrs](https://github.com/cachebag/nmrs) and its developers for the foundation
  to this project
