I had to implement a bit of a hack here to get cargo to link
the sniper_roi library.

If you create the file ~/.cargo/config

and place in it:

[target.x86_64-unknown-linux-gnu.sniper_roi]
rustc-link-search = ["src/"]
rustc-link-lib = ["sniper_roi"]

..then "make rust" will build the project.
