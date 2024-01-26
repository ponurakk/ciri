#[derive(Debug)]
pub struct Manager {
    pub agent: &'static str,
    pub build: Option<&'static str>,
    pub doc: Option<&'static str>,
    pub clean: Option<&'static str>,
    pub run: &'static str,
    pub default_exec: Option<&'static str>,
    pub remove: &'static str,
    pub remove_global: &'static str,
    // pub frozen: Option<&'static str>,
    // pub global: &'static str,
    pub add: &'static str,
    pub add_global: &'static str,
    pub test: &'static str,
    pub search: &'static str,
    pub upgrade: &'static str,
    pub execute: Option<&'static str>,
    pub new: &'static str,
}

pub const CARGO_MANAGER: Manager = Manager {
    agent: "cargo",
    build: Some("cargo build"),
    doc: Some("cargo doc"),
    clean: Some("cargo clean"),
    run: "cargo run",
    default_exec: None,
    remove: "cargo rm",
    remove_global: "cargo uninstall",
    add: "cargo add",
    add_global: "cargo install",
    test: "cargo test",
    search: "cargo search",
    upgrade: "cargo update",
    execute: None,
    new: "cargo new",
};

pub const NPM_MANAGER: Manager = Manager {
    agent: "npm",
    build: None, // TODO: Add build from scripts
    doc: None,
    clean: None, // TODO: Remove node_modules
    run: "npm run",
    default_exec: Some("node"),
    remove: "npm uninstall",
    remove_global: "npm uninstall --global",
    add: "npm install",
    add_global: "npm install --global",
    test: "npm test",
    search: "npm search",
    upgrade: "npm update",
    execute: Some("npx"),
    new: "npm init", // TODO: Create new directory and init there
};

pub const PNPM_MANAGER: Manager = Manager {
    agent: "pnpm",
    build: None,
    doc: None,
    clean: None,
    run: "pnpm run",
    default_exec: Some("node"),
    remove: "pnpm remove",
    remove_global: "pnpm remove --global",
    add: "pnpm add",
    add_global: "pnpm add --global",
    test: "pnpm test",
    search: "pnpm search",
    upgrade: "pnpm update",
    execute: Some("pnpx"),
    new: "pnpm init",
};
