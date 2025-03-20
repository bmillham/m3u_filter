A simple program to filter an M3U list.
Read the config_example.toml file for examples

To install rust:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
This will install everything you need to use Rust

To install m3u_filter, clone this to your projects directory.

Copy config_example.toml to m3u_filter_config.toml and edit to your liking

Once you've edited m3u_filter_config.toml you are ready to run:

cargo run

The first time you run you will notice a lot of packages being downloaded and compiled.
This is normal.
You may also see a few Rust warnings: They are because of a new feature I have
not yet started working on, so you can ignore them.

After the program runs you will get one or more m3u files that have been filtered down
as you configured. You can then import that m3u into TVHeadend, VLC or other player.

Enjoy! And feedback is welcome!
