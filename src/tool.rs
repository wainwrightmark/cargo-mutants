// Copyright 2023 Martin Pool.

//! A generic interface for running a tool such as Cargo that can determine the tree
//! shape, build it, and run tests.
//!
//! At present only Cargo is supported, but this interface aims to leave a place to
//! support for example Bazel in future.

use std::fmt::Debug;
use std::marker::{Send, Sync};
use std::sync::Arc;
use std::time::{Duration, Instant};

use camino::{Utf8Path, Utf8PathBuf};
use tracing::{debug, debug_span, trace};

use crate::build_dir::BuildDir;
use crate::console::Console;
use crate::interrupt::check_interrupted;
use crate::log_file::LogFile;
use crate::options::Options;
use crate::outcome::{Phase, PhaseResult};
use crate::process::Process;
use crate::scenario::Scenario;
use crate::Result;
use crate::SourceFile;

pub trait Tool: Debug + Send + Sync {
    /// Find the root of the package enclosing a given path.
    ///
    /// The root is the enclosing directory that needs to be copied to make a self-contained
    /// scratch directory, and from where source discovery begins.
    fn find_root(&self, path: &Utf8Path) -> Result<Utf8PathBuf>;

    /// Find all the root files from whence source discovery should begin.
    ///
    /// For Cargo, this is files like `src/bin/*.rs`, `src/lib.rs` identified by targets
    /// in the manifest.
    fn root_files(&self, path: &Utf8Path) -> Result<Vec<Arc<SourceFile>>>;

    /// Run the tool for one phase.
    fn run(
        &self,
        build_dir: &mut BuildDir,
        scenario: &Scenario,
        phase: Phase,
        log_file: &mut LogFile,
        timeout: Duration,
        console: &Console,
        options: &Options,
    ) -> Result<PhaseResult> {
        let _span = debug_span!("run", ?phase).entered();
        let start = Instant::now();
        let (argv, envp) = self.compose_argv_envp(build_dir, scenario, phase, options)?;
        let process_status =
            Process::run(&argv, &envp, build_dir.path(), timeout, log_file, console)?;
        check_interrupted()?;
        debug!(?process_status, elapsed = ?start.elapsed());
        Ok(PhaseResult {
            phase,
            duration: start.elapsed(),
            process_status,
            argv,
        })
    }

    /// Compose argv and envp to run one phase in this tool.
    fn compose_argv_envp(
        &self,
        build_dir: &mut BuildDir,
        scenario: &Scenario,
        phase: Phase,
        options: &Options,
    ) -> Result<(Vec<String>, Vec<(String, String)>)>;
}
