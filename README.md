# adventofcode2023-rust

Getting my rust skill warmed back up.

# Lessons learned

## VSCode, Rust debugging, and symlinks

TL:DR; open up VSCode with the full path: /usr/local/home/$USER/workspace/pico-bolero/adventofcode2023-rust/Aoc2023
It turns out that that symlinks are not handled nicely by VSCode, Rust, and LLDB. My workspace
folder is symlinked into my home directory. The compiled objects have the full hard paths and
not the symlink paths. This results in the breakpoints being skipped and a warning in the DEBUG CONSOLE
about not finding a file at the location specified, but there is hardlinked path.
The workaround was to open VSCode with the hardlinked path and then debugging works as normal.
I feel like I've solved this before, but I cannot recall how.
