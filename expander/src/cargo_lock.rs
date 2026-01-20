use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};

pub fn resolve_git_library_path(entry_file: &Path, crate_name: &str) -> Result<PathBuf> {
    let cargo_lock = find_cargo_lock(entry_file)?;
    let source = find_git_source(&cargo_lock, crate_name)?;
    let (repo_name, rev) = parse_git_source(&source)?;
    let checkout = find_git_checkout(&repo_name, rev.as_deref())?;
    find_crate_dir_in_checkout(&checkout, crate_name)
}

fn find_cargo_lock(entry_file: &Path) -> Result<PathBuf> {
    let mut dir = if entry_file.is_dir() {
        entry_file.to_path_buf()
    } else {
        entry_file
            .parent()
            .context("entry_file has no parent directory")?
            .to_path_buf()
    };

    loop {
        let candidate = dir.join("Cargo.lock");
        if candidate.exists() {
            return Ok(candidate);
        }
        if !dir.pop() {
            break;
        }
    }

    bail!("Cargo.lock not found from {:?}", entry_file);
}

fn find_git_source(cargo_lock: &Path, crate_name: &str) -> Result<String> {
    let contents = fs::read_to_string(cargo_lock)
        .with_context(|| format!("failed to read {:?}", cargo_lock))?;

    let mut matches: Vec<(Option<String>, Option<String>)> = Vec::new();

    let mut in_package = false;
    let mut name: Option<String> = None;
    let mut version: Option<String> = None;
    let mut source: Option<String> = None;

    for line in contents.lines() {
        let line = line.trim();
        if line == "[[package]]" {
            if let Some(n) = name.take() {
                if n == crate_name {
                    matches.push((version.take(), source.take()));
                }
            }
            in_package = true;
            name = None;
            version = None;
            source = None;
            continue;
        }

        if !in_package {
            continue;
        }

        if let Some(value) = parse_string_value(line, "name") {
            name = Some(value);
        } else if let Some(value) = parse_string_value(line, "version") {
            version = Some(value);
        } else if let Some(value) = parse_string_value(line, "source") {
            source = Some(value);
        }
    }

    if let Some(n) = name {
        if n == crate_name {
            matches.push((version, source));
        }
    }

    if matches.is_empty() {
        bail!("crate not found in Cargo.lock: {crate_name}");
    }

    let git_matches: Vec<(Option<String>, String)> = matches
        .into_iter()
        .filter_map(|(version, source)| {
            source.and_then(|s| {
                if s.starts_with("git+") {
                    Some((version, s))
                } else {
                    None
                }
            })
        })
        .collect();

    if git_matches.is_empty() {
        bail!("crate is not a git dependency: {crate_name}");
    }

    if git_matches.len() > 1 {
        let versions = git_matches
            .iter()
            .map(|(version, _)| version.clone().unwrap_or_else(|| "unknown".to_string()))
            .collect::<Vec<_>>()
            .join(", ");
        bail!("multiple git dependencies found for {crate_name} in Cargo.lock: {versions}");
    }

    Ok(git_matches[0].1.clone())
}

fn parse_string_value(line: &str, key: &str) -> Option<String> {
    let prefix = format!("{key} =");
    if !line.starts_with(&prefix) {
        return None;
    }
    let value = line.split_once('=')?.1.trim();
    if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
        Some(value[1..value.len() - 1].to_string())
    } else {
        None
    }
}

fn parse_git_source(source: &str) -> Result<(String, Option<String>)> {
    if !source.starts_with("git+") {
        bail!("source is not git: {source}");
    }
    let source = &source[4..];
    let (url, rev) = match source.split_once('#') {
        Some((url, rev)) => (url.to_string(), Some(rev.to_string())),
        None => (source.to_string(), None),
    };

    let repo_name = extract_repo_name(&url)?;
    Ok((repo_name, rev))
}

fn extract_repo_name(url: &str) -> Result<String> {
    let url = url.split('?').next().unwrap_or(url);
    let url = url.trim_end_matches('/');
    let last = url.rsplit('/').next().unwrap_or(url);
    let repo = last.strip_suffix(".git").unwrap_or(last);

    if repo.is_empty() {
        bail!("failed to extract repo name from {url}");
    }

    Ok(repo.to_string())
}

fn find_git_checkout(repo_name: &str, rev: Option<&str>) -> Result<PathBuf> {
    let cargo_home = cargo_home_dir()?;
    let checkouts_dir = cargo_home.join("git").join("checkouts");
    if !checkouts_dir.exists() {
        bail!(
            "cargo git checkouts directory not found: {:?}",
            checkouts_dir
        );
    }

    let prefix = format!("{repo_name}-");
    let mut candidates = Vec::new();
    for entry in fs::read_dir(&checkouts_dir)
        .with_context(|| format!("failed to read {:?}", checkouts_dir))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with(&prefix) {
            candidates.push(path);
        }
    }

    if candidates.is_empty() {
        bail!(
            "no cargo git checkouts found for repo: {repo_name} in {:?}",
            checkouts_dir
        );
    }

    let mut best: Option<(usize, PathBuf)> = None;
    let mut fallback: Vec<PathBuf> = Vec::new();

    for candidate in candidates {
        for entry in
            fs::read_dir(&candidate).with_context(|| format!("failed to read {:?}", candidate))?
        {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let name = entry.file_name();
            let name = name.to_string_lossy();

            match rev {
                Some(rev) => {
                    if name == rev {
                        best = Some((rev.len(), path));
                        continue;
                    }
                    if name.starts_with(rev) || rev.starts_with(name.as_ref()) {
                        let score = name.len().min(rev.len());
                        if best
                            .as_ref()
                            .is_none_or(|(best_score, _)| score > *best_score)
                        {
                            best = Some((score, path));
                        }
                    }
                }
                None => fallback.push(path),
            }
        }
    }

    if let Some((_, path)) = best {
        return Ok(path);
    }

    if rev.is_some() {
        bail!("no matching git checkout found for repo {repo_name} at {rev:?}");
    }

    if fallback.len() == 1 {
        return Ok(fallback.remove(0));
    }

    bail!("multiple git checkouts found for repo {repo_name}; specify a rev in Cargo.lock");
}

fn cargo_home_dir() -> Result<PathBuf> {
    if let Ok(path) = env::var("CARGO_HOME") {
        return Ok(PathBuf::from(path));
    }

    let home = env::var("HOME").context("HOME is not set; set CARGO_HOME")?;
    Ok(PathBuf::from(home).join(".cargo"))
}

fn find_crate_dir_in_checkout(checkout: &Path, crate_name: &str) -> Result<PathBuf> {
    if is_crate_root(checkout, crate_name)? {
        return Ok(checkout.to_path_buf());
    }

    let mut dirs = vec![checkout.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(&dir).with_context(|| format!("failed to read {:?}", dir))? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            if is_crate_root(&path, crate_name)? {
                return Ok(path);
            }
            dirs.push(path);
        }
    }

    bail!(
        "crate directory not found in git checkout: {crate_name} under {:?}",
        checkout
    );
}

fn is_crate_root(dir: &Path, crate_name: &str) -> Result<bool> {
    let manifest = dir.join("Cargo.toml");
    if !manifest.exists() {
        return Ok(false);
    }
    let contents =
        fs::read_to_string(&manifest).with_context(|| format!("failed to read {:?}", manifest))?;

    let mut in_package = false;
    for line in contents.lines() {
        let line = line.trim();
        if line == "[package]" {
            in_package = true;
            continue;
        }
        if line.starts_with('[') && line != "[package]" && in_package {
            break;
        }
        if in_package {
            if let Some(value) = parse_string_value(line, "name") {
                return Ok(value == crate_name);
            }
        }
    }

    Ok(false)
}
