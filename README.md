# Declutter

A GitHub Action that analyses a repository to identify stale branches and issues.

## Features

- Detects **stale branches**: Branches with no activity for a configurable period (default: 6 months).
- Detects **stale issues**: Issues with no updates for a configurable period (default: 6 months).
- Generates a report (`declutter-report.md`) containing all findings.
- Runs as a GitHub Action on a schedule or manually.

## Requirements

- Rust (if running locally).
- A GitHub repository with a valid `GITHUB_TOKEN` set in the environment.

## Usage

### 1. Add this repository
Fork or clone the repository into your GitHub account.

### 2. Set up the GitHub Action

Declutter is configured to run as a GitHub Action by default. The configuration is located in `.github/workflows/declutter.yml`. It runs every Sunday but can also be triggered manually.

### 3. Run Locally (Optional)

1. Install [Rust](https://rustup.rs/).
2. Clone the repository and navigate to its directory.
3. Run:

```bash
cargo build --release
GITHUB_TOKEN=<your-access-token> GITHUB_REPOSITORY=<owner/repo> ./target/release/declutter
```

## Output

It generates `declutter-report.md`, which includes:

- **Stale Branches**: Branches that have been inactive beyond the threshold.
- **Stale Issues**: Issues that haven’t been updated within the threshold.

## Configuration

You can change the thresholds for stale branches and issues by modifying these constants in `src/main.rs`:

```
const STALE_BRANCH_THRESHOLD_DAYS: i64 = 180;
const STALE_ISSUE_THRESHOLD_DAYS: i64 = 180;
```
