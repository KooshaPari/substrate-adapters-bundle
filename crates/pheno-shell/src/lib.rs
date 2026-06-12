//! PhenoShell - CLI Shell Framework

use anyhow::Result;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

/// Shell state
pub struct Shell<S: ShellState = InMemoryShellState> {
    name: String,
    #[allow(dead_code)]
    state: Arc<tokio::sync::RwLock<S>>,
    commands: Vec<CommandDef>,
    history: Arc<Mutex<Vec<String>>>,
}

impl Shell {
    pub fn new(name: impl Into<String>) -> Result<Self> {
        Ok(Self {
            name: name.into(),
            state: Arc::new(tokio::sync::RwLock::new(InMemoryShellState::new())),
            commands: Vec::new(),
            history: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub fn register_command(&mut self, cmd: CommandDef) {
        self.commands.push(cmd);
    }

    pub async fn run(&self) -> Result<()> {
        println!("{} shell v{}", self.name, env!("CARGO_PKG_VERSION"));

        let stdin = BufReader::new(tokio::io::stdin());
        let mut lines = stdin.lines();
        let mut stdout = tokio::io::stdout();

        while let Ok(Some(line)) = lines.next_line().await {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            self.history.lock().unwrap().push(line.to_string());

            match self.execute_line(line).await {
                Ok(Some(output)) => {
                    stdout.write_all(output.as_bytes()).await?;
                    stdout.write_all(b"\n").await?;
                }
                Ok(None) => {}
                Err(e) => {
                    stdout
                        .write_all(format!("Error: {}\n", e).as_bytes())
                        .await?;
                }
            }
        }

        Ok(())
    }

    async fn execute_line(&self, line: &str) -> Result<Option<String>, ShellError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(None);
        }

        match parts[0] {
            "exit" | "quit" => std::process::exit(0),
            "help" => Ok(Some(self.help_text())),
            "history" => Ok(Some(self.history.lock().unwrap().join("\n"))),
            cmd => {
                if let Some(cmd_def) = self.commands.iter().find(|c| c.name == cmd) {
                    (cmd_def.handler)(&parts[1..]).await
                } else {
                    Err(ShellError::UnknownCommand(cmd.to_string()))
                }
            }
        }
    }

    fn help_text(&self) -> String {
        let mut text = String::from("Available commands:\n");
        text.push_str("  exit, quit   - Exit the shell\n");
        text.push_str("  help         - Show this help\n");
        text.push_str("  history      - Show command history\n");

        for cmd in &self.commands {
            text.push_str(&format!("  {:15} - {}\n", cmd.name, cmd.description));
        }
        text
    }
}

/// Command definition
#[allow(clippy::type_complexity)]
pub struct CommandDef {
    pub name: &'static str,
    pub description: &'static str,
    pub handler:
        fn(&[&str]) -> Pin<Box<dyn Future<Output = Result<Option<String>, ShellError>> + Send>>,
}

/// Shell state trait
pub trait ShellState: Send + Sync {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: String);
}

/// In-memory shell state
pub struct InMemoryShellState {
    data: std::collections::HashMap<String, String>,
}

impl InMemoryShellState {
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryShellState {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellState for InMemoryShellState {
    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    fn set(&mut self, key: &str, value: String) {
        self.data.insert(key.to_string(), value);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ShellError {
    #[error("unknown command: {0}")]
    UnknownCommand(String),
    #[error("invalid arguments: {0}")]
    InvalidArgs(String),
}
