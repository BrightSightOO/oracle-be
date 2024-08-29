use std::path::PathBuf;
use std::process::{Command, Output};
use std::{env, fmt};

fn main() {
    let output =
        Command::new("pnpm").arg("generate:idls").output().expect("failed to generate idls");

    if !output.status.success() {
        struct Error(Output);

        impl fmt::Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "failed to generate idls, exit status: {}", self.0.status)?;

                if !self.0.stdout.is_empty() {
                    let stdout = String::from_utf8_lossy(&self.0.stdout);
                    write!(f, "\n\n=== stdout ===\n{stdout}")?;
                }

                if !self.0.stderr.is_empty() {
                    let stderr = String::from_utf8_lossy(&self.0.stderr);
                    write!(f, "\n\n=== stderr ===\n{stderr}")?;
                }

                Ok(())
            }
        }

        panic!("{}", Error(output));
    }

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .expect("missing 'CARGO_MANIFEST_DIR' environment variable");

    let out_dir =
        env::var_os("OUT_DIR").map(PathBuf::from).expect("missing 'OUT_DIR' environment variable");

    let workspace_root =
        manifest_dir.parent().and_then(|p| p.parent()).expect("cannot find workspace root");

    let idl_src = workspace_root.join("idls/optimistic_oracle.json");
    let idl_dst = out_dir.join("solana.idl.zip");

    include_idl::compress_idl(&idl_src, &idl_dst).expect("failed to compress idl");
}
