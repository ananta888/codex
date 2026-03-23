use codex_core::LMSTUDIO_OSS_PROVIDER_ID;
use codex_exec::Cli as ExecCli;
use codex_tui::Cli as TuiCli;

const CODEX_LOCAL_FORK_ENV_VAR: &str = "CODEX_LOCAL_FORK";

pub(crate) fn is_local_fork_invocation() -> bool {
    let exe_stem = std::env::current_exe()
        .ok()
        .and_then(|path| path.file_stem().map(|stem| stem.to_string_lossy().to_string()));
    is_local_fork_invocation_from(
        exe_stem.as_deref(),
        std::env::var(CODEX_LOCAL_FORK_ENV_VAR).ok().as_deref(),
    )
}

fn is_local_fork_invocation_from(exe_stem: Option<&str>, env_override: Option<&str>) -> bool {
    if matches!(env_override, Some("1")) {
        return true;
    }

    exe_stem.is_some_and(|stem| stem.starts_with("codex-local"))
}

pub(crate) fn apply_local_fork_defaults_to_tui(cli: &mut TuiCli) {
    if !is_local_fork_invocation() || cli.oss || cli.oss_provider.is_some() {
        return;
    }

    cli.oss = true;
    cli.oss_provider = Some(LMSTUDIO_OSS_PROVIDER_ID.to_string());
}

pub(crate) fn apply_local_fork_defaults_to_exec(cli: &mut ExecCli) {
    if !is_local_fork_invocation() || cli.oss || cli.oss_provider.is_some() {
        return;
    }

    cli.oss = true;
    cli.oss_provider = Some(LMSTUDIO_OSS_PROVIDER_ID.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn local_fork_detection_accepts_env_override() {
        assert!(is_local_fork_invocation_from(None, Some("1")));
    }

    #[test]
    fn apply_local_fork_defaults_sets_lmstudio_for_tui() {
        let mut cli = TuiCli::parse_from(["codex-local"]);
        apply_local_fork_defaults_to_tui(&mut cli);
        assert!(cli.oss);
        assert_eq!(cli.oss_provider.as_deref(), Some(LMSTUDIO_OSS_PROVIDER_ID));
    }

    #[test]
    fn apply_local_fork_defaults_keeps_explicit_exec_provider() {
        let mut cli = ExecCli::parse_from(["codex-local", "--local-provider", "ollama"]);
        apply_local_fork_defaults_to_exec(&mut cli);
        assert!(!cli.oss);
        assert_eq!(cli.oss_provider.as_deref(), Some("ollama"));
    }
}
