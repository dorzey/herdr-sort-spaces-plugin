//! Sort Herdr workspaces lexicographically by label.
use serde_json::{json, Value};
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::time::Duration;

fn socket_path() -> PathBuf {
    if let Ok(p) = env::var("HERDR_SOCKET_PATH") {
        return PathBuf::from(p);
    }
    let home = env::var("HOME").expect("HOME not set");
    PathBuf::from(home).join(".config/herdr/herdr.sock")
}

fn call(path: &PathBuf, method: &str, params: Value) -> Value {
    let stream = UnixStream::connect(path)
        .unwrap_or_else(|e| panic!("connect to {path:?} failed: {e}"));
    stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    stream.set_write_timeout(Some(Duration::from_secs(5))).unwrap();

    let request = json!({"id": "1", "method": method, "params": params});
    let mut writer = stream.try_clone().expect("clone stream");
    writer
        .write_all(format!("{request}\n").as_bytes())
        .unwrap_or_else(|e| panic!("write to socket failed: {e}"));

    let mut line = String::new();
    BufReader::new(stream)
        .read_line(&mut line)
        .unwrap_or_else(|e| panic!("read from socket failed: {e}"));
    writer.shutdown(Shutdown::Both).ok();

    let resp: Value = serde_json::from_str(&line)
        .unwrap_or_else(|e| panic!("invalid JSON from herdr: {e}: {line}"));
    match resp.get("result") {
        Some(result) => result.clone(),
        None => {
            eprintln!("herdr {method} failed: {resp}");
            std::process::exit(1);
        }
    }
}

/// Returns workspace_ids sorted by label (case-insensitive), reversed if `reverse`.
fn sorted_ids(workspaces: &[Value], reverse: bool) -> Vec<&str> {
    let mut sorted: Vec<&Value> = workspaces.iter().collect();
    sorted.sort_by_key(|w| w["label"].as_str().unwrap_or("").to_lowercase());
    if reverse {
        sorted.reverse();
    }
    sorted
        .iter()
        .map(|w| w["workspace_id"].as_str().unwrap())
        .collect()
}

fn main() {
    let reverse = env::args().nth(1).as_deref() == Some("desc");
    let path = socket_path();

    let list = call(&path, "workspace.list", json!({}));
    let workspaces = list["workspaces"].as_array().expect("workspaces array");

    let current: Vec<&str> = workspaces
        .iter()
        .map(|w| w["workspace_id"].as_str().unwrap())
        .collect();

    let wanted = sorted_ids(workspaces, reverse);

    if current == wanted {
        println!("already sorted");
    } else {
        call(&path, "workspace.move_block", json!({"workspace_ids": wanted}));
        println!("sorted: {}", wanted.join(" "));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ws(id: &str, label: &str) -> Value {
        json!({"workspace_id": id, "label": label})
    }

    #[test]
    fn sorts_ascending_case_insensitive() {
        let workspaces = vec![ws("1", "banana"), ws("2", "Apple"), ws("3", "cherry")];
        assert_eq!(sorted_ids(&workspaces, false), vec!["2", "1", "3"]);
    }

    #[test]
    fn sorts_descending() {
        let workspaces = vec![ws("1", "banana"), ws("2", "Apple"), ws("3", "cherry")];
        assert_eq!(sorted_ids(&workspaces, true), vec!["3", "1", "2"]);
    }

    #[test]
    fn missing_label_sorts_first() {
        let workspaces = vec![ws("1", "banana"), json!({"workspace_id": "2"})];
        assert_eq!(sorted_ids(&workspaces, false), vec!["2", "1"]);
    }

    #[test]
    fn already_sorted_is_stable() {
        let workspaces = vec![ws("1", "apple"), ws("2", "banana")];
        assert_eq!(sorted_ids(&workspaces, false), vec!["1", "2"]);
    }
}
