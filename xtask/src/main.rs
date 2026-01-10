use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand, Args};
use serde_json::{Map, Value};
use std::fs;
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
    /// Slug, e.g. data‑orchestration
    #[arg(long)]
    slug: String,
    /// Overwrite existing files if present
    #[arg(long, default_value_t = false)]
    force: bool,
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
            CbdCommand::NewTask(args) => cbd_new_task(&args.id, &args.slug, args.force),
            CbdCommand::ValidateReady(args) => cbd_validate_ready(&args.id),
        },
    }
}

/// Determine the repository root. The xtask crate lives either at the root or
/// inside `.cbd/xtask`. We ascend one parent from `CARGO_MANIFEST_DIR`.
fn repo_root() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir
        .parent()
        .ok_or_else(|| anyhow!("Could not determine repo root from CARGO_MANIFEST_DIR"))?;
    Ok(root.to_path_buf())
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

fn cbd_new_task(id: &str, slug: &str, force: bool) -> Result<i32> {
    let cbd = cbd_root()?;

    // Output paths
    let task_path = cbd.join("tasks").join(format!("{id}-{slug}.md"));
    let contract_path = cbd.join("contracts").join(format!("{id}.contract.json"));
    let bundle_path = cbd.join("bundles").join(format!("{id}.bundle.json"));
    let evidence_path = cbd.join("reports").join(format!("{id}.evidence.md"));

    // Template paths
    let task_template_path = cbd.join("tasks").join("TEMPLATE.md");
    let contract_template_path = cbd.join("contracts").join("TEMPLATE.contract.json");
    let bundle_template_path = cbd.join("bundles").join("TEMPLATE.bundle.json");
    let evidence_template_path = cbd.join("reports").join("TEMPLATE.evidence.md");

    // Load templates
    let task_template = read_to_string(&task_template_path).with_context(|| {
        format!(
            "Missing task template. Expected at {}",
            task_template_path.display()
        )
    })?;
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

    // Fill a few obvious fields (mirror existing Python behavior)
    let task_text = task_template.replace("<ID>", id);

    set_string_field(&mut contract_t, "id", id);
    set_string_field(&mut bundle_t, "id", id);

    // bundle.artifact_paths.{task,contract,evidence}
    {
        let bundle_obj = ensure_object_mut(&mut bundle_t);
        let artifact_paths = bundle_obj
            .entry("artifact_paths".to_string())
            .or_insert_with(|| Value::Object(Map::new()));
        let ap = ensure_object_mut(artifact_paths);

        ap.insert(
            "task".to_string(),
            Value::String(format!(".cbd/tasks/{id}-{slug}.md")),
        );
        ap.insert(
            "contract".to_string(),
            Value::String(format!(".cbd/contracts/{id}.contract.json")),
        );
        ap.insert(
            "evidence".to_string(),
            Value::String(format!(".cbd/reports/{id}.evidence.md")),
        );
    }

    let evidence_text = evidence_template.replace("<ID>", id);

    // Write outputs
    write_text(&task_path, &task_text, force)?;
    write_json(&contract_path, &contract_t, force)?;
    write_json(&bundle_path, &bundle_t, force)?;
    write_text(&evidence_path, &evidence_text, force)?;

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