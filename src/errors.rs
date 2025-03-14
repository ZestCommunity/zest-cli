use humansize::{format_size, BINARY};
use image::ImageError;
use miette::Diagnostic;
use thiserror::Error;
use vex_v5_serial::packets::cdc2::Cdc2Ack;

#[non_exhaustive]
#[derive(Error, Diagnostic, Debug)]
pub enum CliError {
    #[error(transparent)]
    #[diagnostic(code(zest::io_error))]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(code(zest::serial_error))]
    SerialError(#[from] vex_v5_serial::connection::serial::SerialError),

    #[error(transparent)]
    #[diagnostic(code(zest::cdc2_nack))]
    Nack(#[from] Cdc2Ack),

    #[cfg(feature = "fetch-template")]
    #[error(transparent)]
    #[diagnostic(code(zest::bad_response))]
    ReqwestError(#[from] reqwest::Error),

    #[cfg(feature = "fetch-template")]
    #[error("Recieved a malformed HTTP response")]
    #[diagnostic(code(zest::malformed_response))]
    MalformedResponse,

    #[error(transparent)]
    #[diagnostic(code(zest::image_error))]
    ImageError(#[from] ImageError),

    // TODO: Add source spans.
    #[error("Incorrect type for field `{field}` (expected {expected}, found {found}).")]
    #[diagnostic(
        code(zest::bad_field_type),
        help("The `{field}` field should be of type {expected}.")
    )]
    BadFieldType {
        /// Field name
        field: String,

        /// Expected type
        expected: String,

        /// Actual type
        found: String,
    },

    // TODO: Add optional source spans.
    #[error("The provided slot should be in the range [1, 8] inclusive.")]
    #[diagnostic(
        code(zest::slot_out_of_range),
        help("The V5 brain only has eight program slots. Adjust the `slot` field or argument to be a number from 1-8."),
    )]
    SlotOutOfRange,

    // TODO: Add source spans.
    #[error("{0} is not a valid icon.")]
    #[diagnostic(
        code(zest::invalid_icon),
        help("See `zest upload --help` for a list of valid icon identifiers.")
    )]
    InvalidIcon(String),

    #[error("{0} is not a valid upload strategy.")]
    #[diagnostic(
        code(zest::invalid_upload_strategy),
        help("See `zest upload --help` for a list of valid upload strategies.")
    )]
    InvalidUploadStrategy(String),

    #[error("No slot number was provided.")]
    #[diagnostic(
        code(zest::no_slot),
        help("A slot number is required to upload programs. Try passing in a slot using the `--slot` argument, or setting the `package.v5.metadata.slot` field in your Cargo.toml.")
    )]
    NoSlot,

    #[error("ELF build artifact not found. Is this a binary crate?")]
    #[diagnostic(
        code(zest::no_artifact),
        help("`zest build` should generate an ELF file in your project's `target` folder unless this is a library crate. You can explicitly supply a file to upload with the `--file` (`-f`) argument.")
    )]
    NoArtifact,

    #[error("No V5 devices found.")]
    #[diagnostic(
        code(zest::no_device),
        help("Ensure that a V5 brain or controller is plugged in and powered on with a stable USB connection, then try again.")
    )]
    NoDevice,

    #[error("Output ELF file could not be parsed.")]
    #[diagnostic(code(zest::elf_parse_error))]
    ElfParseError(#[from] object::Error),

    #[error("Controller never switched radio channels.")]
    #[diagnostic(
        code(zest::radio_channel_disconnect_timeout),
        help("Try running `zest upload` again. If the problem persists, power cycle your controller and Brain.")
    )]
    RadioChannelDisconnectTimeout,

    #[error("Controller never reconnected after switching radio channels.")]
    #[diagnostic(
        code(zest::radio_channel_reconnect_timeout),
        help("Try running `zest upload` again. If the problem persists, power cycle your controller and Brain.")
    )]
    RadioChannelReconnectTimeout,

    #[cfg(feature = "field-control")]
    #[error("No V5 controllers found.")]
    #[diagnostic(
        code(zest::no_controller),
        help("`zest fc` can only be ran over a controller connection. Make sure you have a controller plugged into USB, then try again.")
    )]
    NoController,

    #[cfg(feature = "field-control")]
    #[error("Attempted to change the match mode over a direct Brain connection.")]
    #[diagnostic(
        code(zest::brain_connection_set_match_mode),
        help("This state should not be reachable and is a bug if encountered. Please report it to https://github.com/vexide/cargo-v5")
    )]
    BrainConnectionSetMatchMode,

    #[error("Attempted to create a new project at {0}, but the directory is not empty.")]
    #[diagnostic(
        code(zest::project_dir_full),
        help("Try creating the project in a different directory or with a different name.")
    )]
    ProjectDirFull(String),

    #[error("Program exceeded the maximum differential upload size of 2MiB (program was {}).", format_size(*.0, BINARY))]
    #[diagnostic(
        code(cargo_v5::program_too_large),
        help("This size limitation may change in the future. To upload larger binaries, switch to a monolith upload by specifying `--upload-strategy=monolith`.")
    )]
    ProgramTooLarge(usize),

    #[error("Patch exceeded the maximum size of 2MiB (patch was {}).", format_size(*.0, BINARY))]
    #[diagnostic(
        code(cargo_v5::patch_too_large),
        help("Try running a cold upload using `cargo v5 upload --cold`.")
    )]
    PatchTooLarge(usize),
}
