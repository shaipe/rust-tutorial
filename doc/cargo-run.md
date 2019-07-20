cargo-run man page
===

cargo-run — Run the current package

### Synopsis
cargo run [Options] [-- ARGS]

### Description
Run a binary or example of the local package.

All the arguments following the two dashes (--) are passed to the binary to run. If you’re passing arguments to both Cargo and the binary, the ones after -- go to the binary, the ones before go to Cargo.

### Options
Package Selection
By default, the package in the current working directory is selected. The -p flag can be used to choose a different package in a workspace.

`-p` SPEC, --package SPEC

The package to run. See cargo-pkgid(1) for the SPEC format.

### Target Selection
When no target selection options are given, cargo run will run the binary target. If there are multiple binary targets, you must pass a target flag to choose one.

`--bin` NAME
Run the specified binary.

`--example` NAME
Run the specified example.

### Feature Selection
When no feature options are given, the default feature is activated for every selected package.

--features FEATURES
Space or comma separated list of features to activate. These features only apply to the current directory’s package. Features of direct dependencies may be enabled with <dep-name>/<feature-name> syntax.

--all-features
Activate all available features of all selected packages.

--no-default-features
Do not activate the default feature of the current directory’s package.

Compilation Options
--target TRIPLE
Run for the given architecture. The default is the host architecture. The general format of the triple is <arch><sub>-<vendor>-<sys>-<abi>. Run rustc --print target-list for a list of supported targets.

This may also be specified with the build.target config value.

--release
Run optimized artifacts with the release profile. See the Profiles section for details on how this affects profile selection.

### Output Options
--target-dir DIRECTORY
Directory for all generated artifacts and intermediate files. May also be specified with the CARGO_TARGET_DIR environment variable, or the build.target-dir config value. Defaults to target in the root of the workspace.

### Display Options
-v, --verbose
Use verbose output. May be specified twice for "very verbose" output which includes extra output such as dependency warnings and build script output. May also be specified with the term.verbose config value.

-q, --quiet
No output printed to stdout.

--color WHEN
Control when colored output is used. Valid values:

auto (default): Automatically detect if color support is available on the terminal.
always: Always display colors.
never: Never display colors.
May also be specified with the term.color config value.

--message-format FMT
The output format for diagnostic messages. Valid values:

human (default): Display in a human-readable text format.
json: Emit JSON messages to stdout.
short: Emit shorter, human-readable text messages.
Manifest Options
--manifest-path PATH
Path to the Cargo.toml file. By default, Cargo searches in the current directory or any parent directory for the Cargo.toml file.

--frozen, --locked
Either of these flags requires that the Cargo.lock file is up-to-date. If the lock file is missing, or it needs to be updated, Cargo will exit with an error. The --frozen flag also prevents Cargo from attempting to access the network to determine if it is out-of-date.

These may be used in environments where you want to assert that the Cargo.lock file is up-to-date (such as a CI build) or want to avoid network access.

--offline
Prevents Cargo from accessing the network for any reason. Without this flag, Cargo will stop with an error if it needs to access the network and the network is not available. With this flag, Cargo will attempt to proceed without the network if possible.

Beware that this may result in different dependency resolution than online mode. Cargo will restrict itself to crates that are downloaded locally, even if there might be a newer version as indicated in the local copy of the index. See the cargo-fetch(1) command to download dependencies before going offline.

May also be specified with the net.offline config value.

### Common Options
-h, --help
Prints help information.

-Z FLAG...
Unstable (nightly-only) flags to Cargo. Run cargo -Z help for details.

### Miscellaneous Options
-j N, --jobs N
Number of parallel jobs to run. May also be specified with the build.jobs config value. Defaults to the number of CPUs.

### Profiles
Profiles may be used to configure compiler options such as optimization levels and debug settings. See the reference for more details.

Profile selection depends on the target and crate being built. By default the dev or test profiles are used. If the --release flag is given, then the release or bench profiles are used.

### Target	Default Profile	--release Profile
lib, bin, example	dev	release
test, bench, or any target
in "test" or "bench" mode	test	bench
Dependencies use the dev/release profiles.

### Environment
See the reference for details on environment variables that Cargo reads.

### Exit Status
0
Cargo succeeded.

101
Cargo failed to complete.

### Examples
Build the local package and run its main target (assuming only one binary):

cargo run
Run an example with extra arguments:

cargo run --example exname -- --exoption exarg1 exarg2