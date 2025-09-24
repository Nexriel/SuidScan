use std::collections::HashSet;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Config {
    pub gtfobins: HashSet<String>,
}

pub fn checker_config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| Config::new())
}

impl Config {
    pub fn new() -> Self {
        let gtfobins: HashSet<String> = vec![
            "aa-exec",
            "ar",
            "ash",
            "bconsole",
            "bpftrace",
            "capsh",
            "chmod",
            "chown",
            "chroot",
            "clamscan",
            "cp",
            "cpan",
            "cpio",
            "debugfs",
            "dialog",
            "diff",
            "dmidecode",
            "docker",
            "ed",
            "efax",
            "emacs",
            "env",
            "ex",
            "file",
            "find",
            "finger",
            "fish",
            "flock",
            "gdb",
            "gimp",
            "git",
            "grep",
            "gtester",
            "gzip",
            "hd",
            "head",
            "hexdump",
            "highlight",
            "hping3",
            "install",
            "ionice",
            "ip",
            "julia",
            "ksh",
            "ksu",
            "less",
            "links",
            "loginctl",
            "logsave",
            "make",
            "man",
            "more",
            "mosquitto",
            "mysql",
            "nano",
            "nasm",
            "nawk",
            "passwd",
            "perl",
            "pkexec",
            "puppet",
            "python",
            "redis",
            "rsync",
            "sash",
            "scanmem",
            "sed",
            "service",
            "setfacl",
            "setlock",
            "sftp",
            "ssh-agent",
            "sudo",
            "systemctl",
            "tar",
            "tftp",
            "unshare",
            "vagrant",
            "vi",
            "vipw",
            "whois",
            "zsh",
        ]
        .into_iter()
        .map(|bin| bin.to_string())
        .collect();

        Config { gtfobins }
    }
}
