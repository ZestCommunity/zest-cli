use clap::Args;
use object::{Object, ObjectSegment};
use std::process::Stdio;
use tokio::process::Command;

use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    PackageId,
};

use crate::errors::CliError;

use miette::{miette, Result};

/// Common Cargo options to forward.
#[derive(Args, Debug)]
pub struct CargoOpts {
    /// Arguments forwarded to cargo.
    #[arg(
        trailing_var_arg = true,
        allow_hyphen_values = true,
        value_name = "CARGO-OPTIONS"
    )]
    args: Vec<String>,
}

pub struct BuildOutput {
    pub elf_artifact: Utf8PathBuf,
    pub bin_artifact: Utf8PathBuf,
    pub package_id: PackageId,
}

/// Runs the `make` command in the given `path` directory with the arguments provided in `opts`.
/// The `for_simulator` flag is available for future simulator-specific adjustments.
pub async fn build(
    path: &Utf8Path,
    opts: CargoOpts,
    for_simulator: bool,
) -> Result<Option<BuildOutput>> {
    // Create the make command with the specified working directory.
    let mut make_cmd = Command::new("make");
    make_cmd.current_dir(path);
    make_cmd.args(&opts.args);
    make_cmd.stdout(Stdio::piped());
    make_cmd.stderr(Stdio::piped());

    // Optionally adjust the command if for_simulator is true.
    if for_simulator {
        // For example, you might want to add a simulator-specific flag:
        // make_cmd.arg("SIMULATOR=1");
        // (This branch is a placeholder for any simulator-specific logic.)
    }

    // Run the command and capture its output.
    let output = make_cmd
        .output()
        .await
        .map_err(|e| miette!("failed to run make: {}", e))?;

    // Print the output from the make command.
    println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));

    // Exit the process if the command did not succeed.
    if !output.status.success() {
        std::process::exit(output.status.code().unwrap_or(1));
    }

    // Construct the BuildOutput structure with the captured output.
    let build_output = BuildOutput {
        elf_artifact: "".into(),
        bin_artifact: "".into(),
        package_id: PackageId {
            repr: String::from(""),
        },
    };

    Ok(Some(build_output))
}

pub fn objcopy(elf: &[u8]) -> Result<Vec<u8>, CliError> {
    // Parse the ELF file.
    let elf_data = object::File::parse(elf)?;

    // Get the loadable segments (program data) and sort them by virtual address.
    let mut program_segments: Vec<_> = elf_data.segments().collect();
    program_segments.sort_by_key(|seg| seg.address());

    // used to fill gaps between segments with zeros
    let mut last_addr = program_segments.first().unwrap().address();
    // final binary
    let mut bytes = Vec::new();

    // Concatenate all the segments into a single binary.
    for segment in program_segments {
        // Fill gaps between segments with zeros.
        let gap = segment.address() - last_addr;
        if gap > 0 {
            bytes.extend(vec![0; gap as usize]);
        }

        // Push the segment data to the binary.
        let data = segment.data()?;
        bytes.extend_from_slice(data);

        // data.len() can be different from segment.size() so we use the actual data length
        last_addr = segment.address() + data.len() as u64;
    }

    Ok(bytes)
}
