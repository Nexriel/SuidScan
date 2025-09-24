mod error;

pub use self::error::{Error, Result};
use crate::checker_config;
use lib_binary::binary::SUIDBinary;
use std::collections::HashSet;
use std::process::Command;
use tracing::info;

pub struct SuidChecker {
    fx_gtfobins: HashSet<String>,
}

impl SuidChecker {
    pub fn new() -> Self {
        let vuln_bins: HashSet<String> = checker_config().gtfobins.clone();
        Self {
            fx_gtfobins: vuln_bins,
        }
    }

    pub fn is_linux(&self) -> Result<()> {
        match std::env::consts::OS {
            "linux" => Ok(()),
            _ => Err(Error::OsRequirementMismatch {
                system: std::env::consts::OS.to_string(),
            }),
        }
    }

    pub fn check_suid_binaries(&self) -> Result<()> {
        let output = Command::new("find")
            .arg("/")
            .arg("-perm")
            .arg("-u=s")
            .arg("-type")
            .arg("f")
            .output()?;

        let suid_binaries = String::from_utf8_lossy(&output.stdout);
        let mut vulnerable_bins = Vec::new();

        for line in suid_binaries.lines() {
            let binary = SUIDBinary::new(line);
            if self.is_vulnerable(&binary.path) {
                vulnerable_bins.push(binary.path.to_string());
            }
        }

        match vulnerable_bins.is_empty() {
            true => info!("No SUID binaries found."),
            false => {
                for bin in vulnerable_bins {
                    info!("{} is vulnerable to Priv Escalation", bin);
                }
            }
        }
        Ok(())
    }

    fn is_vulnerable(&self, path: &str) -> bool {
        self.fx_gtfobins.contains(path)
    }
}
