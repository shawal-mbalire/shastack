# shastack Development Workflow

## The Rust Pivot
The shastack core is now built with **Rust**. All TypeScript files have been removed to ensure a high-performance, type-safe, and cross-platform experience.

## Installation (One-Liner)
To install the latest version of the `sha` CLI:
```bash
curl -fsSL https://raw.githubusercontent.com/shawal-mbalire/shastack/main/install.sh | bash
```

## Issue-Driven Development (IDD) with `gh`

1.  **Create Issue:** Every task starts with a GitHub issue.
    ```bash
    gh issue create --title "Feature description" --body "Description of the task"
    ```
2.  **Create Branch:** Use the issue ID in the branch name.
    ```bash
    git checkout -b issue-[ID]-[slug]
    ```
3.  **Development:** Follow standard execution cycles. Every commit must follow **Conventional Commits** and end with `#[ID]`.
    - `feat: ... #[ID]` -> Minor version bump
    - `fix: ... #[ID]` -> Patch version bump
    - `feat!: ... #[ID]` -> Major version bump (Breaking Change)
4.  **Create PR:** Once work is validated.
    ```bash
    gh pr create --title "feat: description #[ID]" --body "Details of changes"
    ```
5.  **Merge PR:** After approval/checks.
    ```bash
    gh pr merge --merge --delete-branch
    ```

## CLI Usage
Use `just sha` for all workspace management tasks.
