use anyhow::{anyhow, Context, Result};
use clap::{Args, Parser, Subcommand};
use serde_json::{Map, Value};
use std::collections::{BTreeSet, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Repo automation helpers (Contract‑First / DbC workflow).
///
/// This command provides contract‑first scaffolding and validation for the `.cbd/`
/// directory. It supports nested subcommands under `cbd`.
#[derive(Parser, Debug)]
#[command(
    name = "xtask",
    about = "Repo automation helpers (Contract‑First / DbC workflow)"
)]
enum Cli {
    /// Contract‑First (DbC) workflow helpers under .cbd/
    #[command(subcommand)]
    Cbd(CbdCommand),
}

/// Subcommands for the `cbd` namespace.
#[derive(Subcommand, Debug)]
enum CbdCommand {
    /// Scaffold a new Contract‑First task bundle by copying templates in .cbd/
    NewTask(NewTaskArgs),

    /// Scaffold a new Epic requirements seed (PRD-level) under .cbd/requirements/
    NewEpic(NewEpicArgs),

    /// Fail fast if a contract is not ready
    ValidateReady(ValidateReadyArgs),
    /// Hard gate: ready + evidence coverage + checks
    Verify(VerifyArgs),
}

/// Arguments for the `cbd new-task` subcommand.
#[derive(Args, Debug)]
struct NewTaskArgs {
    /// Task id, e.g. 0001
    #[arg(long)]
    id: String,

    /// Slug, e.g. data-orchestration
    #[arg(long)]
    slug: String,

    /// Overwrite existing files if present
    #[arg(long, default_value_t = false)]
    force: bool,

    /// Ask you a few prompts and write a pre-filled task markdown file.
    ///
    /// This is meant for humans. Agents typically run non-interactive and let CONTRACT mode
    /// resolve ambiguity via question rounds.
    #[arg(long, default_value_t = false)]
    interactive: bool,
}

/// Arguments for the `cbd new-epic` subcommand.
#[derive(Args, Debug)]
struct NewEpicArgs {
    /// Epic id, e.g. EP-0001
    #[arg(long)]
    id: String,

    /// Slug, e.g. trading-bot
    #[arg(long)]
    slug: String,

    /// Overwrite existing files if present
    #[arg(long, default_value_t = false)]
    force: bool,

    /// Ask you a stock set of discovery questions and write a pre-filled epic requirements file.
    ///
    /// This is meant for humans. Agents in REQUIREMENTS mode will iterate via question rounds.
    #[arg(long, default_value_t = false)]
    interactive: bool,
}

/// Arguments for the `cbd validate-ready` subcommand.
#[derive(Args, Debug)]
struct ValidateReadyArgs {
    /// Task id, e.g. 0001
    #[arg(long)]
    id: String,
}

/// Arguments for the `cbd verify` subcommand.
#[derive(Args, Debug)]
struct VerifyArgs {
    /// Task id, e.g. 0001
    #[arg(long)]
    id: String,
}

fn main() {
    // Mirror the Python scripts' exit codes for easy CI use:
    // 0 = ok/ready
    // 2 = file missing
    // 3 = invalid JSON
    // 4 = gate failed (not ready, open questions, or missing proofs)
    let code = match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{e:#}");
            1
        }
    };
    std::process::exit(code);
}

fn run() -> Result<i32> {
    let cli = Cli::parse();

    match cli {
        Cli::Cbd(cmd) => match cmd {
            CbdCommand::NewTask(args) => cbd_new_task(&args),
            CbdCommand::NewEpic(args) => cbd_new_epic(&args),
            CbdCommand::ValidateReady(args) => cbd_validate_ready(&args.id),
            CbdCommand::Verify(args) => cbd_verify(&args.id),
        },
    }
}

/// Determine the repository root.
///
/// Supports both:
/// - `<repo>/xtask`
/// - `<repo>/.cbd/xtask`
fn repo_root() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let parent = manifest_dir
        .parent()
        .ok_or_else(|| anyhow!("Could not determine repo root from CARGO_MANIFEST_DIR"))?;

    if parent.file_name() == Some(OsStr::new(".cbd")) {
        let root = parent
            .parent()
            .ok_or_else(|| anyhow!("Could not determine repo root (xtask is inside .cbd/)"))?;
        return Ok(root.to_path_buf());
    }

    Ok(parent.to_path_buf())
}

/// Return the path to the `.cbd` directory at the repo root.
fn cbd_root() -> Result<PathBuf> {
    Ok(repo_root()?.join(".cbd"))
}

fn read_to_string(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))
}

fn read_json(path: &Path) -> Result<Value> {
    let raw = read_to_string(path)?;
    serde_json::from_str(&raw).with_context(|| format!("Invalid JSON in {}", path.display()))
}

fn write_text(path: &Path, text: &str, force: bool) -> Result<()> {
    if path.exists() && !force {
        return Err(anyhow!(
            "Refusing to overwrite existing file (use --force): {}",
            path.display()
        ));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }
    fs::write(path, text).with_context(|| format!("Failed to write {}", path.display()))
}

fn write_json(path: &Path, value: &Value, force: bool) -> Result<()> {
    let mut s = serde_json::to_string_pretty(value)?;
    s.push('\n');
    write_text(path, &s, force)
}

/// Ensure the given `Value` is an object and return a mutable reference to the underlying map.
fn ensure_object_mut(v: &mut Value) -> &mut Map<String, Value> {
    if !v.is_object() {
        *v = Value::Object(Map::new());
    }
    v.as_object_mut().expect("just ensured object")
}

fn set_string_field(obj: &mut Value, key: &str, val: &str) {
    let map = ensure_object_mut(obj);
    map.insert(key.to_string(), Value::String(val.to_string()));
}

/// Prompt for a single line. If the user enters nothing and a default is provided, returns the default.
fn prompt_line(label: &str, default: Option<&str>) -> Result<String> {
    match default {
        Some(d) => print!("{label} [{d}]: "),
        None => print!("{label}: "),
    }
    io::stdout().flush().context("Failed to flush stdout")?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("Failed to read from stdin")?;

    let s = input.trim().to_string();
    if s.is_empty() {
        Ok(default.unwrap_or("").to_string())
    } else {
        Ok(s)
    }
}

/// Prompt for a multi-line block. User finishes by submitting an empty line.
fn prompt_block(label: &str) -> Result<String> {
    println!("{label} (enter one or more lines; finish with an empty line):");
    let mut lines: Vec<String> = Vec::new();

    loop {
        print!("> ");
        io::stdout().flush().context("Failed to flush stdout")?;

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .context("Failed to read from stdin")?;

        let line = line.trim_end_matches(&['\n', '\r'][..]).to_string();

        if line.trim().is_empty() {
            break;
        }

        lines.push(line);
    }

    Ok(lines.join("\n"))
}

fn title_from_slug(slug: &str) -> String {
    let words = slug
        .split(&['-', '_', ' '][..])
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>();

    if words.is_empty() {
        "Untitled".to_string()
    } else {
        words.join(" ")
    }
}

fn normalize_block(text: &str) -> String {
    text.trim_end().to_string()
}

fn body_or(text: &str, fallback: &str) -> String {
    if text.trim().is_empty() {
        fallback.to_string()
    } else {
        normalize_block(text)
    }
}

#[allow(clippy::too_many_arguments)]
fn render_task_markdown(
    id: &str,
    title: &str,
    goal: &str,
    user_story: &str,
    in_scope: &str,
    out_of_scope: &str,
    scenarios: &str,
    context: &str,
    constraints: &str,
    dependencies: &str,
    unknowns: &str,
) -> String {
    let title = body_or(title, "Untitled");
    let goal = body_or(goal, "TBD");
    let user_story = body_or(
        user_story,
        "As a <user>, I want <capability> so that <benefit>.",
    );
    let in_scope = bullet_lines(in_scope, "- TBD");
    let out_of_scope = bullet_lines(out_of_scope, "- TBD");
    let scenarios = body_or(
        scenarios,
        "### Scenario 1: <name>\nGiven …\nWhen …\nThen …\n\n### Scenario 2: <name>\nGiven …\nWhen …\nThen …",
    );
    let context = body_or(context, "- TBD");
    let constraints = bullet_lines(constraints, "- TBD");
    let dependencies = bullet_lines(dependencies, "- (none yet)");
    let unknowns = if unknowns.trim().is_empty() {
        "- (none yet; the agent will ask in CONTRACT mode)".to_string()
    } else {
        bullet_lines(unknowns, "- (none yet; the agent will ask in CONTRACT mode)")
    };

    // Keep this aligned with your contract-first workflow.
    // The point is to capture just enough intent so CONTRACT mode can draft the DbC artifact.
    format!(
        "\
# Task {id} — {title}

## Goal
{goal}

## User story
{user_story}

## Scope

### In scope
{in_scope}

### Out of scope
{out_of_scope}

## Acceptance scenarios (examples-first)
Use Given/When/Then style.

{scenarios}

## Context
{context}

## Constraints
{constraints}

## Dependencies
{dependencies}

## Observability (optional)
- Logs (redaction expectations if any)
- Metrics
- Traces

## Unknowns
{unknowns}

## Clarifications (optional)
If any section above is blank or vague, answer these prompts (the agent may ask these in CONTRACT mode):
- Goal: What observable outcome proves the task is complete?
- User story: Who benefits from this change, what do they need, and why?
- Context: What existing code/docs should be read first?
- Constraints: Any perf, security, compliance, or operational limits?
- Out of scope: What is explicitly excluded?
"
    )
}

fn bullet_lines(block: &str, fallback: &str) -> String {
    let trimmed = block.trim();
    if trimmed.is_empty() {
        return fallback.to_string();
    }

    trimmed
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            if l.starts_with('-') || l.starts_with('*') {
                l.to_string()
            } else {
                format!("- {l}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[allow(clippy::too_many_arguments)]
fn render_epic_markdown(
    epic_id: &str,
    slug: &str,
    title: &str,
    problem: &str,
    primary_user: &str,
    value: &str,
    success_metrics: &str,
    success_timeframe: &str,
    in_scope: &str,
    out_of_scope: &str,
    must_not_happen: &str,
    security: &str,
    compliance: &str,
    performance: &str,
    reliability: &str,
    cost: &str,
    ops: &str,
    external_systems: &str,
    auth_model: &str,
    data_sources: &str,
    data_sinks: &str,
    runtime_env: &str,
    scenarios: &str,
    walking_skeleton: &str,
) -> String {
    let title = body_or(title, "Untitled epic");
    let problem = body_or(problem, "- TBD");
    let primary_user = body_or(primary_user, "TBD");
    let value = body_or(value, "- TBD");
    let success_metrics = body_or(success_metrics, "- TBD");
    let success_timeframe = body_or(success_timeframe, "- TBD");

    let in_scope = bullet_lines(in_scope, "- TBD");
    let out_of_scope = bullet_lines(out_of_scope, "- TBD");
    let must_not_happen = bullet_lines(must_not_happen, "- (none specified yet)");

    let security = body_or(security, "TBD");
    let compliance = body_or(compliance, "TBD");
    let performance = body_or(performance, "TBD");
    let reliability = body_or(reliability, "TBD");
    let cost = body_or(cost, "TBD");
    let ops = body_or(ops, "TBD");

    let external_systems = bullet_lines(external_systems, "- TBD");
    let auth_model = body_or(auth_model, "TBD");
    let data_sources = bullet_lines(data_sources, "- TBD");
    let data_sinks = bullet_lines(data_sinks, "- TBD");
    let runtime_env = bullet_lines(runtime_env, "- TBD");

    let scenarios = body_or(
        scenarios,
        "### Scenario 1: <name>\nGiven …\nWhen …\nThen …\n\n### Scenario 2: <name>\nGiven …\nWhen …\nThen …",
    );
    let walking_skeleton = if walking_skeleton.trim().is_empty() {
        "- (not specified yet)".to_string()
    } else {
        bullet_lines(walking_skeleton, "- (not specified yet)")
    };

    format!(
        "\
# Epic {epic_id} — {title}
Status: draft

## Problem
{problem}

## Primary user
- {primary_user}

## Value / benefit
{value}

## Success metrics
{success_metrics}

Timeframe / measurement window:
{success_timeframe}

## Scope
### In scope
{in_scope}

### Out of scope / Non-goals
{out_of_scope}

## Must-not-happen (safety / risk constraints)
{must_not_happen}

## Constraints
- Security/privacy: {security}
- Compliance/legal: {compliance}
- Performance/latency: {performance}
- Reliability/availability: {reliability}
- Cost: {cost}
- Operational constraints: {ops}

## Integrations / dependencies
- External systems:
{external_systems}
- Auth model: {auth_model}
- Data sources:
{data_sources}
- Data sinks:
{data_sinks}

## Runtime / environment
{runtime_env}

## Acceptance scenarios (examples-first)
Use Given/When/Then style.

{scenarios}

## Walking skeleton MVP
{walking_skeleton}

## Open questions
- Q-001:
- Q-002:

## Architectural forks (ADRs)
Only if needed. Link MADR files under `docs/decisions/`.

- ADR-0001: <title> — docs/decisions/0001-<slug>.md (status: proposed/accepted/…)

## C4 notes (optional)
- Context diagram: docs/c4/{epic_id}-{slug}/context.(md|puml|mermaid)
- Container diagram: docs/c4/{epic_id}-{slug}/container.(md|puml|mermaid)

## Task backlog
(High-level list. The machine-readable version lives in `{epic_id}-{slug}.tasklist.json`.)

- T-????:
"
    )
}

fn cbd_new_epic(args: &NewEpicArgs) -> Result<i32> {
    let cbd = cbd_root()?;

    let req_dir = cbd.join("requirements");
    let epic_md_path = req_dir.join(format!("{}-{}.md", args.id, args.slug));
    let tasklist_path = req_dir.join(format!("{}-{}.tasklist.json", args.id, args.slug));

    let epic_template_path = req_dir.join("TEMPLATE.epic.md");
    let tasklist_template_path = req_dir.join("TEMPLATE.tasklist.json");

    let default_title = title_from_slug(&args.slug);

    let (epic_title, epic_text) = if args.interactive {
        println!("Creating epic {}-{} interactively.", args.id, args.slug);
        println!("Press Enter to accept defaults or skip optional fields.\n");

        let title = prompt_line("Short title", Some(&default_title))?;

        // Stock discovery questions (baseline for any epic)
        let problem = prompt_block("Problem (1–3 sentences; what problem are we solving?)")?;
        let primary_user = prompt_line("Primary user/persona", None)?;
        let value = prompt_block("Value/benefit (who benefits, what changes, why)")?;

        let success_metrics = prompt_block("Success metrics (what does success look like?)")?;
        let success_timeframe = prompt_line(
            "Success timeframe / measurement window (e.g., '2 weeks after launch')",
            None,
        )?;

        let in_scope = prompt_block("In scope for the first release (one per line)")?;
        let out_of_scope = prompt_block("Out of scope / non-goals (one per line)")?;

        let must_not_happen = prompt_block(
            "Must-not-happen safety/risk constraints (one per line; e.g., 'never leak secrets')",
        )?;

        let security = prompt_line("Security/privacy constraints [optional]", None)?;
        let compliance = prompt_line("Compliance/legal constraints [optional]", None)?;
        let performance = prompt_line("Performance/latency expectations [optional]", None)?;
        let reliability = prompt_line("Reliability/availability expectations [optional]", None)?;
        let cost = prompt_line("Cost constraints [optional]", None)?;
        let ops = prompt_line(
            "Operational constraints (deploy env, observability expectations) [optional]",
            None,
        )?;

        let external_systems = prompt_block("External systems/integrations (one per line)")?;
        let auth_model = prompt_line("Auth model (how do we authenticate?) [optional]", None)?;
        let data_sources = prompt_block("Data sources (one per line) [optional]")?;
        let data_sinks = prompt_block("Data sinks (one per line) [optional]")?;
        let runtime_env = prompt_block(
            "Runtime/environment (where will it run? local/cloud/CI; constraints) (one per line)",
        )?;

        let scenarios = prompt_block(
            "Example scenarios (2–3; Given/When/Then). You may include headings like '### Scenario 1: ...'",
        )?;

        let walking_skeleton = prompt_block(
            "Walking skeleton MVP (smallest end-to-end slice) [optional; one per line]",
        )?;

        let text = render_epic_markdown(
            &args.id,
            &args.slug,
            &title,
            &problem,
            &primary_user,
            &value,
            &success_metrics,
            &success_timeframe,
            &in_scope,
            &out_of_scope,
            &must_not_happen,
            &security,
            &compliance,
            &performance,
            &reliability,
            &cost,
            &ops,
            &external_systems,
            &auth_model,
            &data_sources,
            &data_sinks,
            &runtime_env,
            &scenarios,
            &walking_skeleton,
        );

        (title, text)
    } else {
        // Non-interactive: copy the template and replace obvious placeholders.
        let t = read_to_string(&epic_template_path).with_context(|| {
            format!(
                "Missing epic template. Expected at {}",
                epic_template_path.display()
            )
        })?;

        let text = t
            .replace("<EPIC_ID>", &args.id)
            .replace("<TITLE>", &default_title)
            .replace("<slug>", &args.slug)
            .replace("<epic_id>", &args.id);

        (default_title.clone(), text)
    };

    let mut tasklist_t = read_json(&tasklist_template_path).with_context(|| {
        format!(
            "Missing tasklist template. Expected at {}",
            tasklist_template_path.display()
        )
    })?;

    // Fill required fields
    set_string_field(&mut tasklist_t, "epic_id", &args.id);
    set_string_field(&mut tasklist_t, "slug", &args.slug);
    set_string_field(&mut tasklist_t, "title", &epic_title);
    set_string_field(&mut tasklist_t, "status", "draft");
    // Start with an empty task list; REQUIREMENTS mode will populate.
    {
        let obj = ensure_object_mut(&mut tasklist_t);
        obj.insert("tasks".to_string(), Value::Array(vec![]));
    }

    write_text(&epic_md_path, &epic_text, args.force)?;
    write_json(&tasklist_path, &tasklist_t, args.force)?;

    let root = repo_root()?;
    println!("Created:");
    println!(
        "  {}",
        epic_md_path
            .strip_prefix(&root)
            .unwrap_or(&epic_md_path)
            .display()
    );
    println!(
        "  {}",
        tasklist_path
            .strip_prefix(&root)
            .unwrap_or(&tasklist_path)
            .display()
    );

    Ok(0)
}

fn cbd_new_task(args: &NewTaskArgs) -> Result<i32> {
    let cbd = cbd_root()?;

    // Output paths
    let task_path = cbd
        .join("tasks")
        .join(format!("{}-{}.md", args.id, args.slug));
    let contract_path = cbd
        .join("contracts")
        .join(format!("{}.contract.json", args.id));
    let bundle_path = cbd.join("bundles").join(format!("{}.bundle.json", args.id));
    let evidence_path = cbd.join("reports").join(format!("{}.evidence.md", args.id));
    let evidence_json_path = cbd
        .join("reports")
        .join(format!("{}.evidence.json", args.id));

    // Template paths
    let task_template_path = cbd.join("tasks").join("TEMPLATE.md");
    let contract_template_path = cbd.join("contracts").join("TEMPLATE.contract.json");
    let bundle_template_path = cbd.join("bundles").join("TEMPLATE.bundle.json");
    let evidence_template_path = cbd.join("reports").join("TEMPLATE.evidence.md");
    let evidence_json_template_path = cbd.join("reports").join("TEMPLATE.evidence.json");

    // Load templates (contract/bundle/evidence are required)
    let mut contract_t = read_json(&contract_template_path).with_context(|| {
        format!(
            "Missing contract template. Expected at {}",
            contract_template_path.display()
        )
    })?;
    let mut bundle_t = read_json(&bundle_template_path).with_context(|| {
        format!(
            "Missing bundle template. Expected at {}",
            bundle_template_path.display()
        )
    })?;
    let evidence_template = read_to_string(&evidence_template_path).with_context(|| {
        format!(
            "Missing evidence template. Expected at {}",
            evidence_template_path.display()
        )
    })?;
    let mut evidence_json_t = read_json(&evidence_json_template_path).with_context(|| {
        format!(
            "Missing evidence.json template. Expected at {}",
            evidence_json_template_path.display()
        )
    })?;

    // Title: default from slug; interactive can override
    let default_title = title_from_slug(&args.slug);
    let mut title = default_title.clone();

    // Task markdown:
    // - Non-interactive: use TEMPLATE.md (replace <ID> and <SLUG> if present)
    // - Interactive: prompt for fields and render a filled file (independent of TEMPLATE.md)
    let task_text = if args.interactive {
        println!("Creating task {}-{} interactively.", args.id, args.slug);
        println!("Press Enter to accept defaults or skip optional fields.\n");

        title = prompt_line("Short title", Some(&default_title))?;

        let goal = prompt_block("Goal (what should be true when done)")?;
        let user_story = prompt_line(
            "User story (As a ..., I want ..., so that ...) [optional]",
            None,
        )?;

        let in_scope = prompt_block("Scope: in scope (one per line)")?;
        let out_of_scope = prompt_block("Scope: out of scope (one per line)")?;
        let scenarios = prompt_block(
            "Acceptance scenarios (Given/When/Then) [optional; you may include headings like '### Scenario 1: ...']",
        )?;

        let context = prompt_block("Context (background, links, current behavior)")?;
        let constraints = prompt_block("Constraints (runtime, perf, security, compliance) (one per line)")?;
        let dependencies = prompt_block(
            "Dependencies (external systems/APIs, ADRs) (one per line) [optional]",
        )?;
        let unknowns = prompt_block("Unknowns (open questions; optional)")?;

        render_task_markdown(
            &args.id,
            &title,
            &goal,
            &user_story,
            &in_scope,
            &out_of_scope,
            &scenarios,
            &context,
            &constraints,
            &dependencies,
            &unknowns,
        )
    } else {
        let task_template = read_to_string(&task_template_path).with_context(|| {
            format!(
                "Missing task template. Expected at {}",
                task_template_path.display()
            )
        })?;

        task_template
            .replace("<ID>", &args.id)
            .replace("<SLUG>", &args.slug)
    };

    // Fill a few obvious fields in contract/bundle
    set_string_field(&mut contract_t, "id", &args.id);
    set_string_field(&mut contract_t, "title", &title);

    set_string_field(&mut bundle_t, "id", &args.id);
    set_string_field(&mut bundle_t, "title", &title);
    set_string_field(&mut bundle_t, "slug", &args.slug);

    // bundle.artifact_paths.{task,contract,evidence}
    {
        let bundle_obj = ensure_object_mut(&mut bundle_t);
        let artifact_paths = bundle_obj
            .entry("artifact_paths".to_string())
            .or_insert_with(|| Value::Object(Map::new()));
        let ap = ensure_object_mut(artifact_paths);

        ap.insert(
            "task".to_string(),
            Value::String(format!(".cbd/tasks/{}-{}.md", args.id, args.slug)),
        );
        ap.insert(
            "contract".to_string(),
            Value::String(format!(".cbd/contracts/{}.contract.json", args.id)),
        );
        ap.insert(
            "evidence".to_string(),
            Value::String(format!(".cbd/reports/{}.evidence.md", args.id)),
        );
        ap.insert(
            "evidence_json".to_string(),
            Value::String(format!(".cbd/reports/{}.evidence.json", args.id)),
        );
    }

    let evidence_text = evidence_template.replace("<ID>", &args.id);
    set_string_field(&mut evidence_json_t, "contract_id", &args.id);

    // Write outputs
    write_text(&task_path, &task_text, args.force)?;
    write_json(&contract_path, &contract_t, args.force)?;
    write_json(&bundle_path, &bundle_t, args.force)?;
    write_text(&evidence_path, &evidence_text, args.force)?;
    write_json(&evidence_json_path, &evidence_json_t, args.force)?;

    let root = repo_root()?;
    println!("Created:");
    println!(
        "  {}",
        task_path
            .strip_prefix(&root)
            .unwrap_or(&task_path)
            .display()
    );
    println!(
        "  {}",
        contract_path
            .strip_prefix(&root)
            .unwrap_or(&contract_path)
            .display()
    );
    println!(
        "  {}",
        bundle_path
            .strip_prefix(&root)
            .unwrap_or(&bundle_path)
            .display()
    );
    println!(
        "  {}",
        evidence_path
            .strip_prefix(&root)
            .unwrap_or(&evidence_path)
            .display()
    );
    println!(
        "  {}",
        evidence_json_path
            .strip_prefix(&root)
            .unwrap_or(&evidence_json_path)
            .display()
    );

    Ok(0)
}

fn contract_ready_details(data: &Value) -> (String, usize) {
    let status = data.get("status").and_then(|v| v.as_str()).unwrap_or("");
    let open_q_len = match data.get("open_questions") {
        Some(Value::Array(arr)) => arr.len(),
        Some(Value::Null) | None => 0,
        Some(_) => 1, // present but wrong type => treat as "not ready"
    };

    (status.to_string(), open_q_len)
}

fn cbd_validate_ready(id: &str) -> Result<i32> {
    let cbd = cbd_root()?;
    let contract_path = cbd.join("contracts").join(format!("{id}.contract.json"));

    if !contract_path.exists() {
        println!("Missing contract: {}", contract_path.display());
        return Ok(2);
    }

    let data = match read_json(&contract_path) {
        Ok(v) => v,
        Err(e) => {
            println!("Invalid JSON in {}: {e}", contract_path.display());
            return Ok(3);
        }
    };

    let (status, open_q_len) = contract_ready_details(&data);

    if status != "ready" || open_q_len > 0 {
        println!("Contract {id} not ready:");
        println!("  status={status:?}");
        println!(
            "  open_questions={}",
            data.get("open_questions").unwrap_or(&Value::Null)
        );
        return Ok(4);
    }

    println!("Contract {id} is ready.");
    Ok(0)
}

fn cbd_verify(id: &str) -> Result<i32> {
    let cbd = cbd_root()?;

    let contract_path = cbd.join("contracts").join(format!("{id}.contract.json"));
    let bundle_path = cbd.join("bundles").join(format!("{id}.bundle.json"));
    let evidence_path = cbd.join("reports").join(format!("{id}.evidence.json"));

    if !contract_path.exists() {
        println!("Missing contract: {}", contract_path.display());
        return Ok(2);
    }

    let contract = match read_json(&contract_path) {
        Ok(v) => v,
        Err(e) => {
            println!("Invalid JSON in {}: {e}", contract_path.display());
            return Ok(3);
        }
    };

    let (status, open_q_len) = contract_ready_details(&contract);
    if !(status == "ready" || status == "implemented") || open_q_len > 0 {
        println!("Contract {id} not ready/implemented:");
        println!("  status={status:?}");
        println!(
            "  open_questions={}",
            contract.get("open_questions").unwrap_or(&Value::Null)
        );
        return Ok(4);
    }

    let clause_ids = match collect_contract_clause_ids(&contract) {
        Ok(ids) => ids,
        Err(e) => {
            println!("Contract {id} is ready, but clauses are invalid: {e:#}");
            return Ok(3);
        }
    };

    if let Err(e) = validate_acceptance_test_proves(&contract, &clause_ids) {
        println!("Contract {id} acceptance_tests[].proves is invalid: {e:#}");
        return Ok(3);
    }

    if !bundle_path.exists() {
        println!("Missing bundle: {}", bundle_path.display());
        return Ok(2);
    }

    let bundle = match read_json(&bundle_path) {
        Ok(v) => v,
        Err(e) => {
            println!("Invalid JSON in {}: {e}", bundle_path.display());
            return Ok(3);
        }
    };

    let planned = match collect_bundle_planned_clause_ids(&bundle) {
        Ok(v) => v,
        Err(e) => {
            println!("Invalid bundle.json in {}: {e:#}", bundle_path.display());
            return Ok(3);
        }
    };

    // Bundle hygiene: `phases.build[].proves` must reference only known clause ids.
    let unknown_in_bundle: Vec<String> = planned
        .iter()
        .filter(|cid| !clause_ids.contains(*cid))
        .cloned()
        .collect();
    if !unknown_in_bundle.is_empty() {
        println!("Bundle references unknown clause ids in phases.build[].proves:");
        for cid in unknown_in_bundle {
            println!("  - {cid}");
        }
        return Ok(3);
    }

    // Planning coverage: every clause must be assigned to at least one build work item.
    // This keeps the handoff mechanical and makes delegation possible.
    let unplanned: Vec<String> = clause_ids
        .iter()
        .filter(|cid| !planned.contains(*cid))
        .cloned()
        .collect();
    if !unplanned.is_empty() {
        println!(
            "Bundle planning coverage missing for contract {id}: {}/{} clauses not assigned",
            unplanned.len(),
            clause_ids.len()
        );
        for cid in unplanned {
            println!("  - {cid}");
        }
        return Ok(4);
    }

    if !evidence_path.exists() {
        println!("Missing evidence: {}", evidence_path.display());
        return Ok(2);
    }

    let evidence = match read_json(&evidence_path) {
        Ok(v) => v,
        Err(e) => {
            println!("Invalid JSON in {}: {e}", evidence_path.display());
            return Ok(3);
        }
    };

    if evidence.get("contract_id").and_then(|v| v.as_str()) != Some(id) {
        println!(
            "Evidence contract_id mismatch in {}: expected {id:?}, got {}",
            evidence_path.display(),
            evidence.get("contract_id").unwrap_or(&Value::Null)
        );
        return Ok(3);
    }

    let proven = match collect_proven_clause_ids(&evidence, &clause_ids) {
        Ok(ids) => ids,
        Err(e) => {
            println!(
                "Invalid evidence.json in {}: {e:#}",
                evidence_path.display()
            );
            return Ok(3);
        }
    };

    let missing: Vec<String> = clause_ids
        .iter()
        .filter(|cid| !proven.contains(*cid))
        .cloned()
        .collect();

    if !missing.is_empty() {
        println!(
            "Missing clause proofs for contract {id}: {}/{}",
            missing.len(),
            clause_ids.len()
        );
        for cid in missing {
            println!("  - {cid}");
        }
        return Ok(4);
    }

    let root = repo_root()?;
    run_rust_checks(&root)?;
    run_typescript_checks(&root)?;

    println!("cbd verify {id}: OK");
    Ok(0)
}

fn collect_contract_clause_ids(contract: &Value) -> Result<BTreeSet<String>> {
    let mut ids = BTreeSet::<String>::new();
    let mut dupes = Vec::<String>::new();

    // interfaces.commands[*].{preconditions,postconditions,errors}
    match contract.get("interfaces") {
        Some(Value::Object(interfaces)) => match interfaces.get("commands") {
            Some(Value::Array(commands)) => {
                for (i, cmd) in commands.iter().enumerate() {
                    let ctx = format!("interfaces.commands[{i}]");
                    let cmd_obj = cmd
                        .as_object()
                        .ok_or_else(|| anyhow!("Expected object at {ctx}"))?;

                    let pre = cmd_obj
                        .get("preconditions")
                        .ok_or_else(|| anyhow!("Missing {ctx}.preconditions"))?
                        .as_array()
                        .ok_or_else(|| anyhow!("Expected array at {ctx}.preconditions"))?;
                    collect_clause_ids_from_array(
                        pre,
                        &format!("{ctx}.preconditions"),
                        &mut ids,
                        &mut dupes,
                    )?;

                    let post = cmd_obj
                        .get("postconditions")
                        .ok_or_else(|| anyhow!("Missing {ctx}.postconditions"))?
                        .as_array()
                        .ok_or_else(|| anyhow!("Expected array at {ctx}.postconditions"))?;
                    collect_clause_ids_from_array(
                        post,
                        &format!("{ctx}.postconditions"),
                        &mut ids,
                        &mut dupes,
                    )?;

                    let errs = cmd_obj
                        .get("errors")
                        .ok_or_else(|| anyhow!("Missing {ctx}.errors"))?
                        .as_array()
                        .ok_or_else(|| anyhow!("Expected array at {ctx}.errors"))?;
                    collect_error_clause_ids_from_array(
                        errs,
                        &format!("{ctx}.errors"),
                        &mut ids,
                        &mut dupes,
                    )?;
                }
            }
            Some(_) => return Err(anyhow!("Expected array at interfaces.commands")),
            None => {}
        },
        Some(_) => return Err(anyhow!("Expected object at interfaces")),
        None => return Err(anyhow!("Missing interfaces")),
    }

    // data_contracts[*].invariants
    if let Some(Value::Array(data_contracts)) = contract.get("data_contracts") {
        for (i, dc) in data_contracts.iter().enumerate() {
            let ctx = format!("data_contracts[{i}]");
            let dc_obj = dc
                .as_object()
                .ok_or_else(|| anyhow!("Expected object at {ctx}"))?;
            let inv = dc_obj
                .get("invariants")
                .ok_or_else(|| anyhow!("Missing {ctx}.invariants"))?
                .as_array()
                .ok_or_else(|| anyhow!("Expected array at {ctx}.invariants"))?;
            collect_clause_ids_from_array(inv, &format!("{ctx}.invariants"), &mut ids, &mut dupes)?;
        }
    } else if let Some(v) = contract.get("data_contracts") {
        return Err(anyhow!("Expected array at data_contracts, got {v:?}"));
    }

    // system_invariants
    if let Some(Value::Array(system_invariants)) = contract.get("system_invariants") {
        collect_clause_ids_from_array(
            system_invariants,
            "system_invariants",
            &mut ids,
            &mut dupes,
        )?;
    } else if let Some(v) = contract.get("system_invariants") {
        return Err(anyhow!("Expected array at system_invariants, got {v:?}"));
    }

    if !dupes.is_empty() {
        dupes.sort();
        dupes.dedup();
        return Err(anyhow!("Duplicate clause ids: {}", dupes.join(", ")));
    }

    Ok(ids)
}

fn validate_acceptance_test_proves(contract: &Value, clause_ids: &BTreeSet<String>) -> Result<()> {
    let Some(Value::Array(tests)) = contract.get("acceptance_tests") else {
        return Err(anyhow!("Missing acceptance_tests array"));
    };

    for (i, t) in tests.iter().enumerate() {
        let ctx = format!("acceptance_tests[{i}]");
        let t_obj = t
            .as_object()
            .ok_or_else(|| anyhow!("Expected object at {ctx}"))?;
        let proves = t_obj
            .get("proves")
            .ok_or_else(|| anyhow!("Missing {ctx}.proves"))?
            .as_array()
            .ok_or_else(|| anyhow!("Expected array at {ctx}.proves"))?;
        for (j, cid) in proves.iter().enumerate() {
            let Some(cid) = cid.as_str() else {
                return Err(anyhow!("Expected string at {ctx}.proves[{j}]"));
            };
            if !clause_ids.contains(cid) {
                return Err(anyhow!("Unknown clause id in {ctx}.proves[{j}]: {cid}"));
            }
        }
    }

    Ok(())
}

fn collect_clause_ids_from_array(
    clauses: &[Value],
    ctx: &str,
    ids: &mut BTreeSet<String>,
    dupes: &mut Vec<String>,
) -> Result<()> {
    for (i, c) in clauses.iter().enumerate() {
        let cctx = format!("{ctx}[{i}]");
        let obj = c
            .as_object()
            .ok_or_else(|| anyhow!("Expected object at {cctx}"))?;

        let id = obj
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing string {cctx}.id"))?;
        let statement = obj
            .get("statement")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing string {cctx}.statement"))?;
        let enforcement = obj
            .get("enforcement")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing string {cctx}.enforcement"))?;
        let obligation = obj
            .get("obligation")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing string {cctx}.obligation"))?;

        if id.trim().is_empty() {
            return Err(anyhow!("Empty string {cctx}.id"));
        }
        if statement.trim().is_empty() {
            return Err(anyhow!("Empty string {cctx}.statement"));
        }
        if enforcement.trim().is_empty() {
            return Err(anyhow!("Empty string {cctx}.enforcement"));
        }
        if obligation.trim().is_empty() {
            return Err(anyhow!("Empty string {cctx}.obligation"));
        }

        if !ids.insert(id.to_string()) {
            dupes.push(id.to_string());
        }
    }

    Ok(())
}

fn collect_error_clause_ids_from_array(
    errors: &[Value],
    ctx: &str,
    ids: &mut BTreeSet<String>,
    dupes: &mut Vec<String>,
) -> Result<()> {
    for (i, e) in errors.iter().enumerate() {
        let ectx = format!("{ctx}[{i}]");
        let obj = e
            .as_object()
            .ok_or_else(|| anyhow!("Expected object at {ectx}"))?;

        let id = obj
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing string {ectx}.id"))?;
        let code = obj
            .get("code")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing string {ectx}.code"))?;
        let when = obj
            .get("when")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing string {ectx}.when"))?;
        let enforcement = obj
            .get("enforcement")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing string {ectx}.enforcement"))?;
        let obligation = obj
            .get("obligation")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing string {ectx}.obligation"))?;

        if id.trim().is_empty() {
            return Err(anyhow!("Empty string {ectx}.id"));
        }
        if code.trim().is_empty() {
            return Err(anyhow!("Empty string {ectx}.code"));
        }
        if when.trim().is_empty() {
            return Err(anyhow!("Empty string {ectx}.when"));
        }
        if enforcement.trim().is_empty() {
            return Err(anyhow!("Empty string {ectx}.enforcement"));
        }
        if obligation.trim().is_empty() {
            return Err(anyhow!("Empty string {ectx}.obligation"));
        }

        if !ids.insert(id.to_string()) {
            dupes.push(id.to_string());
        }
    }

    Ok(())
}

fn collect_proven_clause_ids(
    evidence: &Value,
    allowed_clause_ids: &BTreeSet<String>,
) -> Result<HashSet<String>> {
    let obj = evidence
        .as_object()
        .ok_or_else(|| anyhow!("Evidence must be a JSON object"))?;

    let clause_proofs = obj
        .get("clause_proofs")
        .ok_or_else(|| anyhow!("Missing clause_proofs"))?
        .as_array()
        .ok_or_else(|| anyhow!("Expected array at clause_proofs"))?;

    let mut proven = HashSet::<String>::new();

    for (i, entry) in clause_proofs.iter().enumerate() {
        let ctx = format!("clause_proofs[{i}]");
        let eobj = entry
            .as_object()
            .ok_or_else(|| anyhow!("Expected object at {ctx}"))?;
        let clause_id = eobj
            .get("clause_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing string {ctx}.clause_id"))?;

        if !allowed_clause_ids.contains(clause_id) {
            return Err(anyhow!(
                "Unknown clause id in evidence.json at {ctx}.clause_id: {clause_id}"
            ));
        }

        let proofs = eobj
            .get("proofs")
            .ok_or_else(|| anyhow!("Missing {ctx}.proofs"))?
            .as_array()
            .ok_or_else(|| anyhow!("Expected array at {ctx}.proofs"))?;

        // Count as "proven" only if there is at least one proof entry.
        if !proofs.is_empty() {
            proven.insert(clause_id.to_string());
        }
    }

    Ok(proven)
}

fn collect_bundle_planned_clause_ids(bundle: &Value) -> Result<BTreeSet<String>> {
    let mut planned = BTreeSet::<String>::new();
    let mut work_item_ids = HashSet::<String>::new();

    let bundle_obj = bundle
        .as_object()
        .ok_or_else(|| anyhow!("Bundle must be a JSON object"))?;

    let phases = bundle_obj
        .get("phases")
        .ok_or_else(|| anyhow!("Missing phases"))?
        .as_object()
        .ok_or_else(|| anyhow!("Expected object at phases"))?;

    let build = phases
        .get("build")
        .ok_or_else(|| anyhow!("Missing phases.build"))?
        .as_array()
        .ok_or_else(|| anyhow!("Expected array at phases.build"))?;

    for (i, wi) in build.iter().enumerate() {
        let ctx = format!("phases.build[{i}]");
        let wi_obj = wi
            .as_object()
            .ok_or_else(|| anyhow!("Expected object at {ctx}"))?;

        // Basic hygiene: work item ids must be present and unique.
        let wi_id = wi_obj
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing string {ctx}.id"))?;
        if wi_id.trim().is_empty() {
            return Err(anyhow!("Empty string {ctx}.id"));
        }
        if !work_item_ids.insert(wi_id.to_string()) {
            return Err(anyhow!("Duplicate work item id in phases.build: {wi_id}"));
        }

        let proves = wi_obj
            .get("proves")
            .ok_or_else(|| anyhow!("Missing {ctx}.proves"))?
            .as_array()
            .ok_or_else(|| anyhow!("Expected array at {ctx}.proves"))?;

        for (j, cid) in proves.iter().enumerate() {
            let Some(cid) = cid.as_str() else {
                return Err(anyhow!("Expected string at {ctx}.proves[{j}]"));
            };
            if cid.trim().is_empty() {
                return Err(anyhow!("Empty string at {ctx}.proves[{j}]"));
            }
            planned.insert(cid.to_string());
        }
    }

    Ok(planned)
}

fn run_checked(program: &str, args: &[&str], dir: &Path) -> Result<()> {
    println!("==> (cd {}) {} {}", dir.display(), program, args.join(" "));
    let status = Command::new(program)
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("Failed to start {program}"))?;

    if !status.success() {
        let code = status.code().unwrap_or(1);
        return Err(anyhow!("{program} exited with code {code}"));
    }

    Ok(())
}

fn rust_checks_dir(repo_root: &Path) -> Option<PathBuf> {
    if repo_root.join("Cargo.toml").exists() {
        return Some(repo_root.to_path_buf());
    }

    // Fallback: this repo is tooling-only; at least keep `xtask` green.
    let xtask_dir = repo_root.join("xtask");
    if xtask_dir.join("Cargo.toml").exists() {
        return Some(xtask_dir);
    }

    None
}

fn run_rust_checks(repo_root: &Path) -> Result<()> {
    let Some(dir) = rust_checks_dir(repo_root) else {
        return Err(anyhow!(
            "No Rust workspace found (expected Cargo.toml at repo root)"
        ));
    };

    run_checked("cargo", &["fmt", "--check"], &dir)?;
    run_checked("cargo", &["clippy", "--", "-D", "warnings"], &dir)?;
    run_checked("cargo", &["test"], &dir)?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum PackageManager {
    Pnpm,
    Yarn,
    Npm,
}

fn find_frontend_dir(repo_root: &Path) -> Option<PathBuf> {
    let candidates = [
        repo_root.to_path_buf(),
        repo_root.join("frontend"),
        repo_root.join("web"),
        repo_root.join("ui"),
    ];

    candidates
        .into_iter()
        .find(|dir| dir.join("package.json").exists())
}

fn detect_package_manager(dir: &Path) -> PackageManager {
    if dir.join("pnpm-lock.yaml").exists() {
        return PackageManager::Pnpm;
    }
    if dir.join("yarn.lock").exists() {
        return PackageManager::Yarn;
    }
    PackageManager::Npm
}

fn has_script(package_json: &Value, name: &str) -> bool {
    package_json
        .get("scripts")
        .and_then(|v| v.as_object())
        .and_then(|scripts| scripts.get(name))
        .and_then(|v| v.as_str())
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false)
}

fn run_typescript_checks(repo_root: &Path) -> Result<()> {
    let Some(dir) = find_frontend_dir(repo_root) else {
        println!("(no frontend package.json detected; skipping TypeScript checks)");
        return Ok(());
    };

    let package_json_path = dir.join("package.json");
    let package_json = read_json(&package_json_path)
        .with_context(|| format!("Failed to read {}", package_json_path.display()))?;

    let pm = detect_package_manager(&dir);
    let (label, exe) = match pm {
        PackageManager::Pnpm => ("pnpm", "pnpm"),
        PackageManager::Yarn => ("yarn", "yarn"),
        PackageManager::Npm => ("npm", "npm"),
    };

    println!(
        "==> frontend detected at {} (package manager: {label})",
        dir.display()
    );

    let checks = ["lint", "test", "build"];
    for script in checks {
        if !has_script(&package_json, script) {
            println!("(no {script} script; skipping)");
            continue;
        }

        match pm {
            PackageManager::Pnpm | PackageManager::Yarn => {
                run_checked(exe, &[script], &dir)?;
            }
            PackageManager::Npm => {
                if script == "test" {
                    run_checked(exe, &["test"], &dir)?;
                } else {
                    run_checked(exe, &["run", script], &dir)?;
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn collects_clause_ids_across_contract() {
        let contract = json!({
            "interfaces": {
                "commands": [
                    {
                        "name": "cmd",
                        "preconditions": [
                            { "id": "P1", "statement": "s", "enforcement": "rust_test", "obligation": "caller" }
                        ],
                        "postconditions": [
                            { "id": "Q1", "statement": "t", "enforcement": "rust_test", "obligation": "system" }
                        ],
                        "errors": [
                            { "id": "E1", "code": "INVALID", "when": "w", "enforcement": "rust_test", "obligation": "system" }
                        ]
                    }
                ]
            },
            "data_contracts": [
                {
                    "name": "entity",
                    "schema": "schema.json",
                    "invariants": [
                        { "id": "I1", "statement": "inv", "enforcement": "rust_test", "obligation": "system" }
                    ]
                }
            ],
            "system_invariants": [
                { "id": "SI1", "statement": "sys", "enforcement": "rust_test", "obligation": "system" }
            ],
            "acceptance_tests": [
                { "id": "AT-1", "description": "", "proves": ["P1", "Q1"] }
            ]
        });

        let ids = collect_contract_clause_ids(&contract).unwrap();
        assert!(ids.contains("P1"));
        assert!(ids.contains("Q1"));
        assert!(ids.contains("E1"));
        assert!(ids.contains("I1"));
        assert!(ids.contains("SI1"));

        validate_acceptance_test_proves(&contract, &ids).unwrap();
    }

    #[test]
    fn rejects_duplicate_clause_ids() {
        let contract = json!({
            "interfaces": {
                "commands": [
                    {
                        "name": "cmd",
                        "preconditions": [
                            { "id": "DUP", "statement": "s", "enforcement": "rust_test", "obligation": "caller" }
                        ],
                        "postconditions": [
                            { "id": "DUP", "statement": "t", "enforcement": "rust_test", "obligation": "system" }
                        ],
                        "errors": []
                    }
                ]
            },
            "acceptance_tests": []
        });

        let err = collect_contract_clause_ids(&contract).unwrap_err();
        assert!(err.to_string().contains("Duplicate clause ids"));
    }

    #[test]
    fn proven_clause_ids_require_non_empty_proofs() {
        let evidence = json!({
            "contract_id": "0001",
            "clause_proofs": [
                { "clause_id": "P1", "proofs": [ { "kind": "test", "location": "x" } ] },
                { "clause_id": "Q1", "proofs": [] }
            ]
        });

        let allowed: BTreeSet<String> = ["P1", "Q1"].into_iter().map(String::from).collect();
        let proven = collect_proven_clause_ids(&evidence, &allowed).unwrap();
        assert!(proven.contains("P1"));
        assert!(!proven.contains("Q1"));
    }
}
