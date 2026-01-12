use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

struct TestArtifacts {
    files: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
}

impl TestArtifacts {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    fn add_file<P: Into<PathBuf>>(&mut self, path: P) {
        self.files.push(path.into());
    }

    fn add_dir<P: Into<PathBuf>>(&mut self, path: P) {
        self.dirs.push(path.into());
    }
}

impl Drop for TestArtifacts {
    fn drop(&mut self) {
        for path in self.files.drain(..) {
            let _ = fs::remove_file(path);
        }
        for path in self.dirs.drain(..) {
            let _ = fs::remove_dir_all(path);
        }
    }
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root")
        .to_path_buf()
}

fn read_to_string(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("failed to read {}: {err}", path.display());
    })
}

fn apply_replacements(text: &str, replacements: &[(&str, &str)]) -> String {
    let mut out = text.to_string();
    for (key, value) in replacements {
        out = out.replace(key, value);
    }
    out
}

fn headings(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        let hashes = trimmed.chars().take_while(|c| *c == '#').count();
        if hashes == 0 {
            continue;
        }
        let rest = trimmed[hashes..].trim_start();
        if rest.is_empty() {
            continue;
        }
        out.push((hashes, rest.to_string()));
    }
    out
}

fn filtered_headings(text: &str) -> Vec<(usize, String)> {
    headings(text)
        .into_iter()
        .filter(|(level, title)| !(*level >= 3 && title.starts_with("Scenario ")))
        .collect()
}

fn assert_headings_match(expected: &str, actual: &str) {
    assert_eq!(filtered_headings(expected), filtered_headings(actual));
}

fn line(value: &str) -> String {
    format!("{value}\n")
}

fn block(lines: &[&str]) -> String {
    format!("{}\n\n", lines.join("\n"))
}

fn run_xtask(args: &[&str], input: &str) {
    let root = repo_root();
    let mut child = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .current_dir(&root)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn xtask");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("write stdin");

    let output = child.wait_with_output().expect("wait on xtask");
    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("xtask failed\nstdout:\n{stdout}\nstderr:\n{stderr}");
    }
}

#[test]
fn interactive_task_renders_template_and_replaces_sections() {
    let root = repo_root();
    let id = "9002";
    let slug = "interactive-task";
    let title = "Interactive Task Title";

    let task_path = root
        .join(".cbd")
        .join("tasks")
        .join(format!("{id}-{slug}.md"));
    let contract_path = root
        .join(".cbd")
        .join("contracts")
        .join(format!("{id}.contract.json"));
    let bundle_path = root
        .join(".cbd")
        .join("bundles")
        .join(format!("{id}.bundle.json"));
    let evidence_path = root
        .join(".cbd")
        .join("reports")
        .join(format!("{id}.evidence.md"));
    let evidence_json_path = root
        .join(".cbd")
        .join("reports")
        .join(format!("{id}.evidence.json"));

    let mut cleanup = TestArtifacts::new();
    cleanup.add_file(task_path.clone());
    cleanup.add_file(contract_path);
    cleanup.add_file(bundle_path);
    cleanup.add_file(evidence_path);
    cleanup.add_file(evidence_json_path);

    let input = [
        line(title),
        block(&["Goal line"]),
        line("As a developer, I want templates so that output is consistent."),
        block(&["Scope A", "Scope B"]),
        block(&["Out A"]),
        block(&["### Scenario 1: Example", "Given A", "When B", "Then C"]),
        block(&["Context line"]),
        block(&["Constraint 1", "Constraint 2"]),
        block(&["Dependency 1"]),
        block(&["Unknown 1"]),
    ]
    .concat();

    run_xtask(
        &[
            "cbd",
            "new-task",
            "--id",
            id,
            "--slug",
            slug,
            "--interactive",
            "--force",
        ],
        &input,
    );

    let output_text = read_to_string(&task_path);
    let template = read_to_string(&root.join(".cbd").join("tasks").join("TEMPLATE.md"));
    let expected_template =
        apply_replacements(&template, &[("<ID>", id), ("<short title>", title)]);

    assert_headings_match(&expected_template, &output_text);
    assert!(output_text.contains(&format!("# Task {id} — {title}")));
    assert!(!output_text.contains("<ID>"));
    assert!(!output_text.contains("<short title>"));

    assert!(output_text.contains("Goal line"));
    assert!(!output_text.contains("Describe what should be true when done."));

    assert!(output_text.contains("As a developer, I want templates so that output is consistent."));
    assert!(!output_text.contains("As a <user>, I want <capability> so that <benefit>."));

    assert!(output_text.contains("- Scope A"));
    assert!(output_text.contains("- Scope B"));
    assert!(output_text.contains("- Out A"));

    assert!(output_text.contains("### Scenario 1: Example"));
    assert!(!output_text.contains("### Scenario 1: <name>"));

    assert!(output_text.contains("- Context line"));
    assert!(!output_text.contains("- Relevant background"));

    assert!(output_text.contains("- Constraint 1"));
    assert!(output_text.contains("- Constraint 2"));
    assert!(!output_text.contains("- Runtime/platform constraints"));

    assert!(output_text.contains("- Dependency 1"));
    assert!(!output_text.contains("- External systems / APIs"));

    assert!(output_text.contains("- Unknown 1"));

    assert!(output_text.contains("## Observability (optional)"));
    assert!(output_text.contains("- Logs (redaction expectations if any)"));
    assert!(output_text.contains("- Metrics"));
    assert!(output_text.contains("- Traces"));
}

#[test]
fn interactive_epic_renders_template_and_replaces_sections() {
    let root = repo_root();
    let id = "EP-9002";
    let slug = "interactive-epic";
    let title = "Interactive Epic Title";

    let epic_path = root
        .join(".cbd")
        .join("requirements")
        .join(format!("{id}-{slug}.md"));
    let tasklist_path = root
        .join(".cbd")
        .join("requirements")
        .join(format!("{id}-{slug}.tasklist.json"));

    let mut cleanup = TestArtifacts::new();
    cleanup.add_file(epic_path.clone());
    cleanup.add_file(tasklist_path);

    let input = [
        line(title),
        block(&["Problem line"]),
        block(&["Primary user: Devs"]),
        block(&["Goal A", "Goal B"]),
        block(&["Metric A"]),
        block(&["Scope A"]),
        block(&["Scope B"]),
        block(&["Security/privacy: None", "Compliance/legal: None"]),
        block(&["External systems: None"]),
        block(&["1) As a user, I want X so that Y."]),
        block(&["### Scenario 1: Epic", "Given A", "When B", "Then C"]),
        block(&["Q-001: TBD"]),
    ]
    .concat();

    run_xtask(
        &[
            "cbd",
            "new-epic",
            "--id",
            id,
            "--slug",
            slug,
            "--interactive",
            "--force",
        ],
        &input,
    );

    let output_text = read_to_string(&epic_path);
    let template = read_to_string(
        &root
            .join(".cbd")
            .join("requirements")
            .join("TEMPLATE.epic.md"),
    );
    let expected_template = apply_replacements(
        &template,
        &[
            ("<EPIC_ID>", id),
            ("<TITLE>", title),
            ("<slug>", slug),
            ("<epic_id>", id),
        ],
    );

    assert_headings_match(&expected_template, &output_text);
    assert!(output_text.contains(&format!("# Epic {id} — {title}")));
    assert!(!output_text.contains("<EPIC_ID>"));
    assert!(!output_text.contains("<TITLE>"));
    assert!(!output_text.contains("<slug>"));
    assert!(!output_text.contains("<epic_id>"));

    assert!(output_text.contains("- Problem line"));
    assert!(!output_text.contains("What problem are we solving?"));

    assert!(output_text.contains("- Primary user: Devs"));
    assert!(!output_text.contains("- Secondary users:"));

    assert!(output_text.contains("- Goal A"));
    assert!(output_text.contains("- Goal B"));

    assert!(output_text.contains("- Metric A"));
    assert!(!output_text.contains("- What measurement window/timeframe?"));

    assert!(output_text.contains("- Scope A"));
    assert!(output_text.contains("- Scope B"));

    assert!(output_text.contains("- Security/privacy: None"));
    assert!(!output_text.contains("- Performance/latency:"));

    assert!(output_text.contains("- External systems: None"));
    assert!(!output_text.contains("- Auth model:"));

    assert!(output_text.contains("1) As a user, I want X so that Y."));
    assert!(!output_text.contains("1) …"));

    assert!(output_text.contains("### Scenario 1: Epic"));

    assert!(output_text.contains("Q-001: TBD"));
    assert!(!output_text.contains("- Q-001:"));

    assert!(output_text.contains("## Architectural forks (ADRs)"));
    assert!(output_text.contains("ADR-0001: <title> — docs/decisions/0001-interactive-epic.md"));
    assert!(output_text.contains("## C4 notes (optional)"));
    assert!(output_text.contains("docs/c4/EP-9002-interactive-epic/context"));
    assert!(output_text.contains("## Task backlog"));
}

#[test]
fn interactive_review_renders_template_and_replaces_sections() {
    let root = repo_root();
    let id = "R-9002";
    let slug = "interactive-review";
    let title = "Interactive Review Title";

    let review_dir = root
        .join(".cbd")
        .join("reviews")
        .join(format!("{id}-{slug}"));
    let seed_path = review_dir.join("review.seed.md");

    let mut cleanup = TestArtifacts::new();
    cleanup.add_dir(review_dir.clone());

    let input = [
        line("Implementation"),
        line("2025-01-01"),
        block(&[
            "- PRD link / doc path: docs/prd.md",
            "- Repo / commit / PR link (if implementation): https://example.com/pr/1",
            "- Owner / stakeholders: Core Team",
        ]),
        block(&["Scope item A"]),
        block(&["Scope item B"]),
        block(&["Assumption 1"]),
        block(&["Credentials"]),
        block(&["GitHub"]),
        block(&["PII is restricted"]),
        block(&["Risk 1"]),
    ]
    .concat();

    run_xtask(
        &[
            "cbd",
            "new-review",
            "--id",
            id,
            "--slug",
            slug,
            "--title",
            title,
            "--interactive",
            "--force",
        ],
        &input,
    );

    let output_text = read_to_string(&seed_path);
    let template = read_to_string(
        &root
            .join(".cbd")
            .join("reviews")
            .join("TEMPLATE")
            .join("review.seed.md"),
    );
    let expected_template = apply_replacements(
        &template,
        &[("<REVIEW_ID>", id), ("<TITLE>", title), ("<slug>", slug)],
    );

    assert_headings_match(&expected_template, &output_text);
    assert!(output_text.contains(&format!("# Review {id} — {title}")));
    assert!(!output_text.contains("<REVIEW_ID>"));
    assert!(!output_text.contains("<TITLE>"));

    assert!(output_text.contains("Kind: Implementation"));
    assert!(output_text.contains("Date: 2025-01-01"));
    assert!(!output_text.contains("Kind: PRD | Implementation | Both"));
    assert!(!output_text.contains("Date: YYYY-MM-DD"));

    assert!(output_text.contains("- PRD link / doc path: docs/prd.md"));
    assert!(output_text
        .contains("- Repo / commit / PR link (if implementation): https://example.com/pr/1"));
    assert!(output_text.contains("- Owner / stakeholders: Core Team"));

    assert!(output_text.contains("- Scope item A"));
    assert!(output_text.contains("- Scope item B"));

    assert!(output_text.contains("- Assumption 1"));
    assert!(output_text.contains("- Credentials"));
    assert!(output_text.contains("- GitHub"));
    assert!(output_text.contains("- PII is restricted"));
    assert!(output_text.contains("- Risk 1"));

    assert!(output_text.contains("STRIDE: applicable (default)"));
    assert!(output_text.contains("Component list (id + slug + short purpose):"));
    assert!(output_text.contains("- contract-map.md"));
}
