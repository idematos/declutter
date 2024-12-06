use octocrab::{models::repos::RepoCommit, Octocrab};
use chrono::{Duration, Utc};
use std::{env, fs};

const STALE_BRANCH_THRESHOLD_DAYS: i64 = 180;
const STALE_ISSUE_THRESHOLD_DAYS: i64 = 180;

async fn get_stale_branches(repo_owner: &str, repo_name: &str, octocrab: &Octocrab) -> Vec<String> {
    let mut stale_branches = Vec::new();

    if let Ok(branches) = octocrab.repos(repo_owner, repo_name).list_branches().send().await {
        for branch in branches {
            if let Ok(commit) = octocrab
                .repos(repo_owner, repo_name)
                .get_commit(&branch.commit.sha)
                .await
            {
                if let Some(last_commit_date) = commit.commit.author.and_then(|a| a.date) {
                    let days_inactive = (Utc::now() - last_commit_date).num_days();
                    if days_inactive > STALE_BRANCH_THRESHOLD_DAYS {
                        stale_branches.push(format!(
                            "{} (inactive for {} days)",
                            branch.name, days_inactive
                        ));
                    }
                }
            }
        }
    }

    stale_branches
}

async fn get_stale_issues(repo_owner: &str, repo_name: &str, octocrab: &Octocrab) -> Vec<String> {
    let mut stale_issues = Vec::new();

    if let Ok(issues) = octocrab
        .issues(repo_owner, repo_name)
        .list()
        .state("open")
        .send()
        .await
    {
        for issue in issues {
            if let Some(last_updated_date) = issue.updated_at {
                let days_inactive = (Utc::now() - last_updated_date).num_days();
                if days_inactive > STALE_ISSUE_THRESHOLD_DAYS {
                    stale_issues.push(format!(
                        "#{}: {} (inactive for {} days)",
                        issue.number, issue.title, days_inactive
                    ));
                }
            }
        }
    }

    stale_issues
}

fn generate_report(stale_branches: Vec<String>, stale_issues: Vec<String>, output_file: &str) {
    let mut report = String::new();
    report.push_str("# Declutter Report\n");

    report.push_str("## Stale Branches\n");
    if stale_branches.is_empty() {
        report.push_str("No stale branches found.\n");
    } else {
        for branch in stale_branches {
            report.push_str(&format!("- {}\n", branch));
        }
    }

    report.push_str("\n## Stale Issues\n");
    if stale_issues.is_empty() {
        report.push_str("No stale issues found.\n");
    } else {
        for issue in stale_issues {
            report.push_str(&format!("- {}\n", issue));
        }
    }

    fs::write(output_file, report).expect("Failed to write report");
}

#[tokio::main]
async fn main() {
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let github_repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY must be set");
    let (repo_owner, repo_name) = github_repo
        .split_once('/')
        .expect("GITHUB_REPOSITORY must be in 'owner/repo' format");

    let octocrab = Octocrab::builder().personal_token(github_token).build().unwrap();

    println!("Fetching stale branches...");
    let stale_branches = get_stale_branches(repo_owner, repo_name, &octocrab).await;

    println!("Fetching stale issues...");
    let stale_issues = get_stale_issues(repo_owner, repo_name, &octocrab).await;

    println!("Generating report...");
    generate_report(stale_branches, stale_issues, "declutter-report.md");
    println!("Report generated: declutter-report.md");
}