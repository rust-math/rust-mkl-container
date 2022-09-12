//! Create container of MKL library, found by intel-mkl-tool

use anyhow::{ensure, Context, Result};
use colored::Colorize;
use intel_mkl_tool::{Config, Library, LinkType, Threading};
use oci_spec::image::Platform;
use ocipkg::{image::Builder, ImageName};
use std::{
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

fn main() -> Result<()> {
    let run_id: u64 = std::env::var("GITHUB_RUN_ID")
        .unwrap_or_else(|_| "0".to_string()) // fallback value for local testing
        .parse()?;
    for cfg in Config::possibles() {
        if cfg.link == LinkType::Dynamic {
            // ocipkg is designed for static linking only.
            // Getting dynamically-link library as a dependency is bad idea.
            continue;
        }

        let lib = Library::new(cfg)?;
        let (year, _, update) = lib.version()?;
        let name = ImageName::parse(&format!(
            "ghcr.io/rust-math/rust-mkl/{}/{}:{}.{}-{}",
            std::env::consts::OS,
            cfg,
            year,
            update,
            run_id
        ))?;
        let output = format!("{}.tar", cfg);

        eprintln!("{:>12} {}", "Packaging".green().bold(), name);
        let timer = Instant::now();
        pack(lib, &name, &output)?;
        eprintln!(
            "{:>12} {} ({:.2}s)",
            "Created".green().bold(),
            output,
            timer.elapsed().as_secs_f32()
        );
    }
    Ok(())
}

/// Create oci-archive
pub fn pack(lib: Library, name: &ImageName, output: impl AsRef<Path>) -> Result<()> {
    ensure!(lib.config.link == LinkType::Static);

    let cfg = lib.config;
    let mut libs: Vec<PathBuf> = intel_mkl_tool::mkl_libs(cfg)
        .into_iter()
        .map(|lib_name| {
            let file_name = intel_mkl_tool::mkl_file_name(cfg.link, &lib_name);
            lib.library_dir.join(file_name)
        })
        .collect();

    if lib.config.parallel == Threading::OpenMP {
        let iomp5 = lib
            .iomp5_static_dir
            .context("Static OpenMP runtime not found")?
            .join(intel_mkl_tool::openmp_runtime_file_name(LinkType::Static));
        libs.push(iomp5);
    }

    let mut f = fs::File::create(output)?;
    let mut builder = Builder::new(&mut f);
    builder.append_files(&libs)?;
    builder.set_platform(&Platform::default());
    builder.set_name(name);
    Ok(())
}
