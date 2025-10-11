# Save System

The following document outlines how the save system works.

It is divided into three sections:

- Current system
- How multiple saves could be added to the current system
- The custom system that should be used in the future

## Current save system

Sandy Fact'ry currently uses [Serde](https://crates.io/crates/serde/) to serialize all relevant structures to binary and save them to a key-value database using [bevy_pkv](https://crates.io/crates/bevy_pkv/).
This approach has the advantage of being easy to implement and storing the data in quite a space efficient format, but it has multiple problems:

- Basically every change to the internal Rust data structure invalidates the save file. To warn the user about this, the version number of the time the save file was created is stored.
- I have little control over how the data is actually saved and can only read whole sections at once, which leads to the above problem.
- A sep

The game is saved when the player quits the game (as long as it isn't killed by the OS), goes back to the main menu, clicks "Save" in the pause menu and automatically every minute.
The save logic is in one central function in [saving/save_game.rs](../../src/saving/save_game.rs) and can be called from everywhere in the project.
It needs the following arguments, which can all be acquired from the ECS:

- `pkv`: The connection to the database
- `seed`, `machine_tiles`, `camera_translations`, `has_completed_game`: The properties that need to be saved

All the properties, that should be saved are put into a struct called `GameSave` that is then serialized by Serde and written to the database.
After that is done, the current game version number (gotten from Cargo.toml's version key) is written to the database under a different key so it can be read independetly from the game save.

## Adding multiple saves

These steps provide a high level overview of what needs to be done in order to add multiple saves to the current systems:

1. Change the save keys so theres a game save and version key appended with the save id
2. Add a new menu to show current saves and a button to create a new one
3. Save the new save under the key prefix and id
   The latest id should be saved under its own key and be incremented every time a new save is created. This is done so no two saves will ever collide.

## New save system

Modifying the old system would only make sense if the feature was needed immediately. If there is more time available, a completely custom system should be implemented.
This will be outlined in the following section.

Every save should be put into its own file in the save directory. The file format is described below.

### File Format
