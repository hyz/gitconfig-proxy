//! Main entry point for Myex2
#![feature(exit_status_error)]
#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use git_clone_or_pull::application::APP;

/// Boot git_clone_or_pull
#[tokio::main]
async fn main() {
    abscissa_core::boot(&APP);
}
