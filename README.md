# Declutter

A GitHub Action that analyzes a repository to find stale branches and issues.

## Features

- Detects **stale branches**: Branches with no activity for a configurable period (default: 6 months).
- Detects **stale issues**: Issues with no updates for a configurable period (default: 6 months).
- Generates a **markdown report** (`declutter-report.md`) with all findings.
- Runs as a **GitHub Action** on a schedule or manually.

## Requirements

- Rust (if running locally).
- A GitHub repository with a valid `GITHUB_TOKEN` set in the environment.

## Installation

### 1. Add this repository
Fork or clone the repository into your GitHub account.

### 2. Set up the GitHub Action

Declutter is configured to run as a GitHub Action by default. The configuration is located in `.github/workflows/declutter.yml`. It runs every Sunday but can also be triggered manually.
