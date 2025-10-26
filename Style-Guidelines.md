# Style Guide v0.1.0 
This document describes how to enable and **require** the respective formatters and hooks for various programming languages in your development environment based on the provided configuration and follow styling.

> [!NOTE]
> All Flakes should include these based on what template you use. This is for cases after usage.
---

### General Hooks (Required)

The following formatters and checks are always **required** to be enabled for all languages and workflows:

Nix Formating:
- **nixfmt-rfc-style.enable**: Required Nix file formatting according to RFC style.
- **flake-checker.enable**: Required flake configuration checking.
- **statix.enable**: Required static analysis checks.

Git:
- **commitizen.enable**: Required commit message format enforcement.
- **check-merge-conflicts.enable**: Required check for merge conflicts.
- **no-commit-to-branch.enable**: Required prevention of commits to protected branches.

Workflows:
- **check-yaml.enable**: Required YAML syntax checking.
- **yamlfmt.enable**: Required YAML file formatting.
- **yamllint.enable**: Required YAML linting.

---

### Rust Hooks (Required)

When Rust language is detected, the following hooks must be **enabled and enforced**:

- `rustfmt.enable`: Required Rust code formatter.
- `clippy.enable`: Required Clippy linting.
- `clippy.settings.extraArgs`: Set to treat all warnings as errors (`-D warnings`).
- `cargo-check.enable`: Required to perform workspace cargo checks.
- `cargo-check.settings.workspace`: Checks are enabled for the entire workspace.

Please also continue to the [Toml Section](#toml-hooks-required)

---

### Node.js Hooks (Required)

- `biome.enable`: Required formatter and linter for Node.js.

---

### Python Hooks (Required)

- `ruff.enable`: Required Python linter.
- `ruff-format.enable`: Required Python code formatter.
- `uv.enable`: Required Python utility/tool integration.

---

### Markdown Hooks (Required)

- `cspell.enable`: Required spell checker for Markdown.
- `markdownlint.enable`: Required Markdown linter.
- `mdformat.enable`: Required Markdown formatter.

---

### TOML Hooks (Required)

- `"check-toml".enable`: Required for the validation of toml
- `taplo.enable`: Required toolkit for toml formating

---

### Gradle Hooks (Required)

> [!WARNING]
> This currently doesn't work please skip to toml section.

`google-java-format.enable`: Required for java formatting
`ktlint`: Required for Kotlin Formatting


Please also continue to [Toml Section](#toml-hooks-required) if your using lib.versions.toml 

---

# Summary

The formatters and linters specified in this configuration are **mandatory** and must be enabled for each supported language. This ensures code style consistency, quality, and automated formatting enforcement as part of the development workflow.
If not followed your pr will be blocked with linkage to this guide.
