use std::path::PathBuf;

pub const MERCURIO_WORKSPACE_ROOT_ENV: &str = "MERCURIO_WORKSPACE_ROOT";
pub const MERCURIO_PILOT_ROOT_ENV: &str = "MERCURIO_PILOT_ROOT";
pub const MERCURIO_EXAMPLES_ROOT_ENV: &str = "MERCURIO_EXAMPLES_ROOT";

const PILOT_REPO_NAME: &str = "SysML-v2-Pilot-Implementation";
const EXAMPLES_REPO_NAME: &str = "mercurio-examples";

pub fn default_pilot_root() -> PathBuf {
    if let Some(path) = env_path(MERCURIO_PILOT_ROOT_ENV) {
        return path;
    }

    if let Some(workspace_root) = env_path(MERCURIO_WORKSPACE_ROOT_ENV) {
        let sibling = workspace_root.join(PILOT_REPO_NAME);
        if sibling.exists() {
            return sibling;
        }
        return workspace_root.join("external").join(PILOT_REPO_NAME);
    }

    let external = PathBuf::from("../external").join(PILOT_REPO_NAME);
    if external.exists() {
        external
    } else {
        PathBuf::from("..").join(PILOT_REPO_NAME)
    }
}

pub fn default_kerml_examples_root(fallback_in_core: impl Into<PathBuf>) -> PathBuf {
    let fallback_in_core = fallback_in_core.into();

    if let Some(path) = env_path(MERCURIO_EXAMPLES_ROOT_ENV) {
        let kerml_examples = path.join("kerml").join("examples");
        if kerml_examples.exists() {
            return kerml_examples;
        }
        return path;
    }

    if let Some(workspace_root) = env_path(MERCURIO_WORKSPACE_ROOT_ENV) {
        let examples_root = workspace_root
            .join(EXAMPLES_REPO_NAME)
            .join("kerml")
            .join("examples");
        if examples_root.exists() {
            return examples_root;
        }
    }

    fallback_in_core
}

fn env_path(name: &str) -> Option<PathBuf> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}
