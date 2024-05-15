# Lovers Splitter

An auto splitter for Lovers in a Dangerous Spacetime.

## Explanation

### Start

The pointers for the save files are saved on every update.
When the currently selected (on that update) save file goes from null (0) to some value, we know that that save file has just been created.
When this happens the timer starts.

### Split
When the player completes a level in a campaign, a temporary "ResumeSaveData" structure is created.
This structure contains information about the current campaign and level, however it only gets created when a level is actually completed, thus the following sequence happens:

| game state   | ResumeSaveData level |
|--------------|----------------------|
| level select | None                 |
| level 1      | None                 |
| level 2      | 2                    |
| level 3      | 3                    |
| level 4      | 4                    |
| boss         | 5                    |
| level select | None                 |

Thus, no distinction can be made between beating the boss and quiting to level select, this is currently an inherent flaw of this splitter.

### Final Split

The game Control class contains a flag to determine if the player currently has input, this flag goes from 0 to 2 when the final boss is defeated, since both the boss defeat cutscene and mission complete screen pause input (this does not occur on other bosses).
Since we can read the ResumeSaveData to retrieve the current campaign and level number, we can check if we are currently on the final boss, and then split on the 0->2 transition.

### Reset

Unity has an internal value for the currently loaded level, see the table below:

| level | state                     |
|-------|---------------------------|
| 0     | init                      |
| 1     | logo                      |
| 2     | gamepad recommended       |
| 3     | loading                   |
| 4     | main menu                 |
| 5     | character select          |
| 6     | level/chapter select      |
| 7     | in level                  |
| 8     | intro cutscene            |
| 9     | ending cutscene           |
| 10    | mission/campaign complete |
| 14    | credits                   |

We can detect the main menu state to reset the splitter.

## Compilation

This auto splitter is written in Rust. In order to compile it, you need to
install the Rust compiler: [Install Rust](https://www.rust-lang.org/tools/install).

Afterwards install the WebAssembly target:
```sh
rustup target add wasm32-unknown-unknown --toolchain nightly
```

The auto splitter can now be compiled:
```sh
cargo b --release
```

The auto splitter is then available at:
```
target/wasm32-unknown-unknown/release/lovers_splitter.wasm
```

Make sure too look into the [API documentation](https://livesplit.org/asr/asr/) for the `asr` crate.

## Development

You can use the [debugger](https://github.com/LiveSplit/asr-debugger) while
developing the auto splitter to more easily see the log messages, statistics,
dump memory, step through the code and more.

The repository comes with preconfigured Visual Studio Code tasks. During
development it is recommended to use the `Debug Auto Splitter` launch action to
run the `asr-debugger`. You need to install the `CodeLLDB` extension to run it.

You can then use the `Build Auto Splitter (Debug)` task to manually build the
auto splitter. This will automatically hot reload the auto splitter in the
`asr-debugger`.

Alternatively you can install the [`cargo
watch`](https://github.com/watchexec/cargo-watch?tab=readme-ov-file#install)
subcommand and run the `Watch Auto Splitter` task for it to automatically build
when you save your changes.

The debugger is able to step through the code. You can set breakpoints in VSCode
and it should stop there when the breakpoint is hit. Inspecting variables may
not work all the time.
