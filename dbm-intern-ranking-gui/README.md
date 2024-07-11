# DBM Ranking GUI

Rust application using Egui framework for registering Data of the DBM intern weekly Smash Tourney. 

## Functionality

The Application provides multiple Dropdown menus for selecting the Discord username of the Player, up to 5 diffrent Charackter that the Player picked. It also provides 2 Textfields for the amount of Wins and Loses

After filling out and klicking the send button the Application sends a Post request to an API Server.

## Usage

Fill out all fields with the corresponding Player data and klick the send button

## Installation

To install the Software download the appropriate File for your Operating System from the [Download Page](https://github.com/KyokoSpl/dbm-intern-ranking/releases/tag/1.0.0) and Run it.
To run the Programm you need to have rust installed.

## Building from Source

### Deps:
- rust
- cargo

### how to do
Download the source Code from the [Download Page](https://github.com/KyokoSpl/dbm-intern-ranking/releases/tag/1.0.0) unpack it than do the following:
```bash
cd dbm-intern-ranking/
cargo build 

```
If you want to execute it immediately do `cargo run` instead of `cargo build`
