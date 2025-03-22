A simple program to filter an M3U list.
Read the config_example.toml file for examples

To install rust:\
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh\
This will install everything you need to use Rust

To install m3u\_filter, clone this to your projects directory:\
mkdir -p projects\
cd projects\
git clone https://github.com/bmillham/m3u_filter

The project is now in projects/m3u\_filter

cd m3u\_filter\
Everything from here on is done in the m3u\_filter directory.

Copy config_example.toml to m3u\_filter\_config.toml and edit to your liking

Once you've edited m3u\_filter\_config.toml you are ready to run:

cargo run

The first time you run you will notice a lot of packages being downloaded and compiled.
This is normal.
You may also see a few Rust warnings: They are because of a new feature I have
not yet started working on, so you can ignore them.

After the program runs you will get one or more m3u files that have been filtered down
as you configured. You can then import that m3u into TVHeadend, VLC or other player.

# Building
If you want to run this from a cron job, etc you need to build the project. To do this just run

cargo build --release

And you will find m3u_filter in target/release

You can copy that file anywhere you'd like. By default it will look for the config file and put the parsed files in the current directory.
This obviously isn't good for running from a cron job, so I added an options to specify the config file and where to put the parsed files so you can now do:

m3u_filter -c /home/user/projects/m3u\_filter/m3u\_filter\_config.toml -o /home/user/projects/m3u\_filter

# Options
+ -c, --config\_file: Specify the location of the config file. Defaults to m3u\_filter\_config.toml in the current directory
+ -o, --output\_dir: Specify where to write the files. Defaults to the current directory
+ -i, --input\_file: A local m3u file (or URL). Overrides the [urls] section in the config file. Useful for testing.
+ -t, --template: Overrides the template setting in the config file. Useful for testing.

Enjoy! And feedback is welcome!
