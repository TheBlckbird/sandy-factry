# Sandy Fact'ry

Sandy Fact'ry is a factory builder developed for a school project.

## Install

Go to the [latest release](https://github.com/TheBlckbird/sandy-factry/releases/latest) and download the respective file for your system.

### Building from source

There are multiple options to build this project:

-   `cargo run`: This is the simplest option, it compiles a debug build and immediately runs it. Not intended for redistribution.
-   `cargo build --release`: Builds a release build for the current platform and puts it into `target/release/sandy-factry(.exe)`. But this can't be distributed on its own, it needs the `assets` folder.
-   `./build/macos.sh`: Compiles and bundles the project for MacOS. This may take some time, because it needs to build it twice to create a build that runs on both Apple Silicon and the old Intel processors.
-   `./build/windows.sh`: Compiles and bundles the project for Windows. This creates a zip archive with the executable and the assets folder.

The output for the two build scripts can be found in the `/out` directory.

Note that building this for the first time may take up to ten minutes, although it's usually around five. This is due to the whole game engine needing to be compiled. All the compiles after that will be faster, because it only needs to recompile what's changed.

Also, the `/target` directory may become quite big (multiple GBs), because of the incremental compiles. You can safely delete it after you're done, but you will need to recompile everything again the next time.

## Documention

I had to write a documentation for the school project. It goes more in depth into the simulation algorithm, but is currently only available in German:

[Dokumentation.pdf](https://github.com/user-attachments/files/20745087/Dokumentation.pdf)

But be aware that it is already outdated.

## License

This project is dual licensed under [MIT](/LICENSE-MIT) and [Apache-2.0](/LICENSE-APACHE).
