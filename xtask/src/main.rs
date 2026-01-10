use anyhow::{anyhow, Context, Result};
use clap::{Args, Parser, Subcommand};
use serde_json::{Map, Value};
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Repo automation helpers (Contract‑First / DbC workflow).
///
/// This command provides contract‑first scaffolding and validation for the `.cbd/`
/// directory. It supports nested subcommands under `cbd`.
#[derive(Parser, Debug)]
#[command(name = "xtask", about = "Repo automation helpers (Contract‑First / DbC workflow)")]
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
    /// Fail fast if a contract is not ready
    ValidateReady(ValidateReadyArgs),
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

/// Arguments for the `cbd validate-ready` subcommand.
#[derive(Args, Debug)]
struct ValidateReadyArgs {
    /// Task id, e.g. 0001
    #[arg(long)]
    id: String,
}

fn main() {
    // Mirror the Python scripts' exit codes for easy CI use:
    // 0 = ok/ready
    // 2 = file missing
    // 3 = invalid JSON
    // 4 = not ready / open questions present
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
            CbdCommand::ValidateReady(args) => cbd_validate_ready(&args.id),
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
fn ensure_object_mut<'a>(v: &'a mut Value) -> &'a mut Map<String, Value> {
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

        let line = line
            .trim_end_matches(|c| c == '\n' || c == '\r')
            .to_string();

        if line.trim().is_empty() {
            break;
        }

        lines.push(line);
    }

    Ok(lines.join("\n"))
}

fn title_from_slug(slug: &str) -> String {
    let words = slug
        .split(|c: char| c == '-' || c == '_' || c == ' ')
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

fn render_task_markdown(
    id: &str,
    title: &str,
    goal: &str,
    user_story: &str,
    context: &str,
    constraints: &str,
    out_of_scope: &str,
    unknowns: &str,
) -> String {
    let title = body_or(title, "Untitled");
    let goal = body_or(goal, "TBD");
    let user_story = body_or(user_story, "As a <user>, I want <capability> so that <benefit>.");
    let context = body_or(context, "- TBD");
    let constraints = body_or(constraints, "- TBD");
    let out_of_scope = body_or(out_of_scope, "- TBD");
    let unknowns = if unknowns.trim().is_empty() {
        "- (none yet; the agent will ask in CONTRACT mode)".to_string()
    } else {
        normalize_block(unknowns)
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

## Context
{context}

## Constraints
{constraints}

## Out of scope
{out_of_scope}

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

fn cbd_new_task(args: &NewTaskArgs) -> Result<i32> {
    let cbd = cbd_root()?;

    // Output paths
    let task_path = cbd.join("tasks").join(format!("{}-{}.md", args.id, args.slug));
    let contract_path = cbd
        .join("contracts")
        .join(format!("{}.contract.json", args.id));
    let bundle_path = cbd.join("bundles").join(format!("{}.bundle.json", args.id));
    let evidence_path = cbd
        .join("reports")
        .join(format!("{}.evidence.md", args.id));

    // Template paths
    let task_template_path = cbd.join("tasks").join("TEMPLATE.md");
    let contract_template_path = cbd.join("contracts").join("TEMPLATE.contract.json");
    let bundle_template_path = cbd.join("bundles").join("TEMPLATE.bundle.json");
    let evidence_template_path = cbd.join("reports").join("TEMPLATE.evidence.md");

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
        let context = prompt_block("Context (background, links, current behavior)")?;
        let constraints = prompt_block("Constraints (runtime, perf, security, compliance)")?;
        let out_of_scope = prompt_block("Out of scope (explicitly not doing)")?;
        let unknowns = prompt_block("Unknowns (open questions; optional)")?;

        render_task_markdown(
            &args.id,
            &title,
            &goal,
            &user_story,
            &context,
            &constraints,
            &out_of_scope,
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
    }

    let evidence_text = evidence_template.replace("<ID>", &args.id);

    // Write outputs
    write_text(&task_path, &task_text, args.force)?;
    write_json(&contract_path, &contract_t, args.force)?;
    write_json(&bundle_path, &bundle_t, args.force)?;
    write_text(&evidence_path, &evidence_text, args.force)?;

    let root = repo_root()?;
    println!("Created:");
    println!(
        "  {}",
        task_path.strip_prefix(&root).unwrap_or(&task_path).display()
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
        bundle_path.strip_prefix(&root).unwrap_or(&bundle_path).display()
    );
    println!(
        "  {}",
        evidence_path
            .strip_prefix(&root)
            .unwrap_or(&evidence_path)
            .display()
    );

    Ok(0)
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

    let status = data.get("status").and_then(|v| v.as_str()).unwrap_or("");
    let open_q_len = match data.get("open_questions") {
        Some(Value::Array(arr)) => arr.len(),
        Some(Value::Null) | None => 0,
        Some(_) => 1, // present but wrong type => treat as "not ready"
    };

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