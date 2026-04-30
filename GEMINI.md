# shastack Development Workflow

## Issue-Driven Development (IDD) with `gh`

1.  **Create Issue:** Every task starts with a GitHub issue.
    ```bash
    gh issue create --title "Feature description" --body "Description of the task"
    ```
2.  **Create Branch:** Use the issue ID in the branch name.
    ```bash
    git checkout -b issue-[ID]-[slug]
    ```
3.  **Development:** Follow standard execution cycles. Every commit must end with `#[ID]`.
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
