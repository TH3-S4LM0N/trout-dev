# trout-dev
A gh repo for trout. It will get a new dedi repo on 1st release.

# trout
CLI Music Player

### Deps
Requires: (non-rust deps) <br>
- <a href="https://github.com/spotDL/spotify-downloader">spotdl</a> <br>

## TODO
- Backend
    - load
        - figure out how to make rust realize that `cfg_path` will never be an `Option<PathBuf>`
    - commands
        - gen
            - check if xdg and trout exists so rust doesnt panic
        - new
            - rework to allow easy addition of new downloaders/sites
                - spotify.rs is messy
        - play
            - use regex