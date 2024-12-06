# Declutter

A GitHub Action that analyzes a repository to find stale branches and issues.

## Features

- Detects **stale branches**: Branches with no activity for a configurable period (default: 6 months).
- Detects **stale issues**: Issues with no updates for a configurable period (default: 6 months).
- Generates a **markdown report** (`declutter-report.md`) with all findings.
- Runs as a **GitHub Action** on a schedule or manually.
