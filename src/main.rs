use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal;
use terminal_size::{terminal_size, Width};

#[derive(Debug, Clone)]
struct Config {
    field: Option<usize>,
    delimiter: char,
    top: usize,
    interval_ms: u64,
}

const DEFAULT_TOP: usize = 10;
const DEFAULT_INTERVAL_MS: u64 = 200;
const DEFAULT_DELIMITER: char = ' ';

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match parse_args(&args[1..]) {
        Ok(config) => config,
        Err(message) => {
            eprintln!("{message}");
            print_usage();
            std::process::exit(2);
        }
    };

    let stop_flag = Arc::new(AtomicBool::new(false));
    let handler_flag = Arc::clone(&stop_flag);
    ctrlc::set_handler(move || {
        handler_flag.store(true, Ordering::SeqCst);
    })
    .expect("failed to set Ctrl-C handler");

    let _ = terminal::enable_raw_mode();
    let counts = process_stream(&config, &stop_flag);
    let _ = terminal::disable_raw_mode();
    print_final(&counts, config.top);
}

fn parse_args(args: &[String]) -> Result<Config, String> {
    let mut field: Option<usize> = None;
    let mut delimiter: char = DEFAULT_DELIMITER;
    let mut top: usize = DEFAULT_TOP;
    let mut interval_ms: u64 = DEFAULT_INTERVAL_MS;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            "-f" | "--field" => {
                let value = next_value(args, &mut i, "field")?;
                let parsed = value.parse::<usize>().map_err(|_| {
                    format!("invalid value for --field: {value}")
                })?;
                if parsed == 0 {
                    return Err("field must be 1 or greater".to_string());
                }
                field = Some(parsed);
            }
            "-d" | "--delimiter" => {
                let value = next_value(args, &mut i, "delimiter")?;
                let mut chars = value.chars();
                let first = chars.next().ok_or_else(|| {
                    "delimiter must be a single character".to_string()
                })?;
                if chars.next().is_some() {
                    return Err("delimiter must be a single character".to_string());
                }
                delimiter = first;
            }
            "-n" | "--top" => {
                let value = next_value(args, &mut i, "top")?;
                let parsed = value.parse::<usize>().map_err(|_| {
                    format!("invalid value for --top: {value}")
                })?;
                if parsed == 0 {
                    return Err("top must be 1 or greater".to_string());
                }
                top = parsed;
            }
            "--interval" => {
                let value = next_value(args, &mut i, "interval")?;
                let parsed = value.parse::<u64>().map_err(|_| {
                    format!("invalid value for --interval: {value}")
                })?;
                if parsed == 0 {
                    return Err("interval must be 1 or greater".to_string());
                }
                interval_ms = parsed.clamp(50, 2000);
            }
            unknown => {
                return Err(format!("unknown option: {unknown}"));
            }
        }

        i += 1;
    }

    Ok(Config {
        field,
        delimiter,
        top,
        interval_ms,
    })
}

fn next_value<'a>(args: &'a [String], i: &mut usize, name: &str) -> Result<&'a str, String> {
    if *i + 1 >= args.len() {
        return Err(format!("missing value for --{name}"));
    }
    *i += 1;
    Ok(args[*i].as_str())
}

fn print_usage() {
    let mut stderr = io::stderr();
    let _ = write_usage(&mut stderr);
}

fn write_usage<W: Write>(mut out: W) -> io::Result<()> {
    write!(
        out,
        "Usage: tally [OPTIONS]\n\
         \n\
         Options:\n\
           -f, --field <N>       Field index (1-based)\n\
           -d, --delimiter <C>   Delimiter character (default: space, 1 char)\n\
           -n, --top <N>         Show top N entries (default: 10)\n\
           --interval <MS>       Refresh interval in ms (default: 200, 50-2000)\n\
           -h, --help            Show this help\n"
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RunState {
    Running,
    Paused,
    Quitting,
}

impl RunState {
    fn label(self) -> &'static str {
        match self {
            RunState::Running => "running",
            RunState::Paused => "paused",
            RunState::Quitting => "quitting",
        }
    }
}

fn process_stream(config: &Config, stop_flag: &Arc<AtomicBool>) -> HashMap<String, usize> {
    let rx = spawn_reader(Arc::clone(stop_flag));
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut last_render = Instant::now();
    let mut rendered_once = false;
    let mut state = RunState::Running;

    loop {
        if stop_flag.load(Ordering::SeqCst) {
            break;
        }

        while let Ok(line) = rx.try_recv() {
            let trimmed = line.trim_end_matches(|c| c == '\n' || c == '\r');
            update_counts(trimmed, config, &mut counts);
        }

        if event::poll(Duration::from_millis(50)).unwrap_or(false) {
            if let Ok(Event::Key(key)) = event::read() {
                match key.code {
                    KeyCode::Char('q') => {
                        state = RunState::Quitting;
                        stop_flag.store(true, Ordering::SeqCst);
                    }
                    KeyCode::Char(' ') => {
                        state = if state == RunState::Paused {
                            RunState::Running
                        } else {
                            RunState::Paused
                        };
                        render_tui(&counts, config.top, state);
                        last_render = Instant::now();
                        rendered_once = true;
                    }
                    KeyCode::Char('r') => {
                        counts.clear();
                        render_tui(&counts, config.top, state);
                        last_render = Instant::now();
                        rendered_once = true;
                    }
                    _ => {}
                }
            }
        }

        if state == RunState::Quitting {
            break;
        }

        if state == RunState::Running
            && (!rendered_once || last_render.elapsed() >= Duration::from_millis(config.interval_ms))
        {
            render_tui(&counts, config.top, state);
            last_render = Instant::now();
            rendered_once = true;
        }
    }

    counts
}

fn spawn_reader(stop_flag: Arc<AtomicBool>) -> Receiver<String> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut buffer: Vec<u8> = Vec::new();
        loop {
            if stop_flag.load(Ordering::SeqCst) {
                break;
            }
            buffer.clear();
            let bytes = match reader.read_until(b'\n', &mut buffer) {
                Ok(bytes) => bytes,
                Err(_) => break,
            };
            if bytes == 0 {
                break;
            }
            let line = decode_line(&buffer);
            if tx.send(line).is_err() {
                break;
            }
        }
    });
    rx
}

fn decode_line(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn tally_from_input(input: &str, config: &Config) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for raw_line in input.split('\n') {
        let trimmed = raw_line.trim_end_matches(|c| c == '\n' || c == '\r');
        update_counts(trimmed, config, &mut counts);
    }
    counts
}

fn update_counts(line: &str, config: &Config, counts: &mut HashMap<String, usize>) {
    if line.is_empty() {
        return;
    }
    if let Some(key) = extract_key(line, config) {
        *counts.entry(key).or_insert(0) += 1;
    }
}

fn extract_key(line: &str, config: &Config) -> Option<String> {
    let field_index = match config.field {
        None => return Some(line.to_string()),
        Some(value) => value,
    };
    let index = field_index.saturating_sub(1);

    if config.delimiter == DEFAULT_DELIMITER {
        let value = line.split_whitespace().nth(index)?;
        Some(value.to_string())
    } else {
        let value = line.split(config.delimiter).nth(index)?;
        Some(value.to_string())
    }
}

fn top_n(counts: &HashMap<String, usize>, limit: usize) -> Vec<(String, usize)> {
    let mut entries: Vec<(String, usize)> = counts
        .iter()
        .map(|(key, count)| (key.clone(), *count))
        .collect();

    entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    if entries.len() > limit {
        entries.truncate(limit);
    }
    entries
}

fn render_tui(counts: &HashMap<String, usize>, limit: usize, state: RunState) {
    let top = top_n(counts, limit);
    let mut out = String::new();
    out.push_str("\x1b[2J\x1b[H");
    out.push_str(&format!("tally (top) [{}]\n\n", state.label()));

    if top.is_empty() {
        out.push_str("no data yet\n");
        let mut stdout = io::stdout();
        let _ = stdout.write_all(out.as_bytes());
        let _ = stdout.flush();
        return;
    }

    let max = top.first().map(|entry| entry.1).unwrap_or(0);
    let cols = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);
    let bar_width = cols.saturating_sub(28).clamp(10, 60);
    let label_width = cols.saturating_sub(bar_width + 12).max(8);

    for (key, count) in top {
        let bar_len = if max == 0 {
            0
        } else {
            (count * bar_width) / max
        };
        let bar = "#".repeat(bar_len);
        let mut label = key;
        if label.chars().count() > label_width {
            let take_len = label_width.saturating_sub(3);
            label = label.chars().take(take_len).collect::<String>() + "...";
        }
        out.push_str(&format!(
            "{count:>8} | {bar:<width$} {label}\n",
            width = bar_width
        ));
    }

    let mut stdout = io::stdout();
    let _ = stdout.write_all(out.as_bytes());
    let _ = stdout.flush();
}

fn print_final(counts: &HashMap<String, usize>, limit: usize) {
    let top = top_n(counts, limit);
    for (key, count) in top {
        println!("{count}\t{key}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_key_with_whitespace() {
        let config = Config {
            field: Some(2),
            delimiter: DEFAULT_DELIMITER,
            top: 10,
            interval_ms: 200,
        };
        let key = extract_key("alpha  beta  gamma", &config);
        assert_eq!(key.as_deref(), Some("beta"));
    }

    #[test]
    fn extract_key_with_delimiter_keeps_empty() {
        let config = Config {
            field: Some(2),
            delimiter: ',',
            top: 10,
            interval_ms: 200,
        };
        let key = extract_key("a,,c", &config);
        assert_eq!(key.as_deref(), Some(""));
    }

    #[test]
    fn top_n_sorts_by_count_then_key() {
        let mut counts = HashMap::new();
        counts.insert("b".to_string(), 2);
        counts.insert("a".to_string(), 2);
        counts.insert("c".to_string(), 1);

        let top = top_n(&counts, 3);
        assert_eq!(top[0], ("a".to_string(), 2));
        assert_eq!(top[1], ("b".to_string(), 2));
        assert_eq!(top[2], ("c".to_string(), 1));
    }

    #[test]
    fn update_counts_skips_empty_and_missing() {
        let config = Config {
            field: Some(2),
            delimiter: DEFAULT_DELIMITER,
            top: 10,
            interval_ms: 200,
        };
        let mut counts = HashMap::new();
        update_counts("", &config, &mut counts);
        update_counts("onlyone", &config, &mut counts);
        update_counts("a b", &config, &mut counts);
        update_counts("a b", &config, &mut counts);

        assert_eq!(counts.get("b"), Some(&2));
        assert_eq!(counts.len(), 1);
    }

    #[test]
    fn parse_rejects_invalid_field() {
        let args = vec!["--field".to_string(), "0".to_string()];
        let err = parse_args(&args).unwrap_err();
        assert!(err.contains("field must be 1 or greater"));
    }

    #[test]
    fn parse_rejects_empty_delimiter() {
        let args = vec!["--delimiter".to_string(), "".to_string()];
        let err = parse_args(&args).unwrap_err();
        assert!(err.contains("delimiter must be a single character"));
    }

    #[test]
    fn parse_rejects_multi_char_delimiter() {
        let args = vec!["--delimiter".to_string(), "::".to_string()];
        let err = parse_args(&args).unwrap_err();
        assert!(err.contains("delimiter must be a single character"));
    }

    #[test]
    fn parse_clamps_interval() {
        let args = vec!["--interval".to_string(), "5".to_string()];
        let config = parse_args(&args).expect("config");
        assert_eq!(config.interval_ms, 50);
    }

    #[test]
    fn parse_rejects_negative_field() {
        let args = vec!["--field".to_string(), "-1".to_string()];
        let err = parse_args(&args).unwrap_err();
        assert!(err.contains("invalid value for --field"));
    }

    #[test]
    fn extract_key_returns_none_for_large_field() {
        let config = Config {
            field: Some(99),
            delimiter: DEFAULT_DELIMITER,
            top: 10,
            interval_ms: 200,
        };
        let key = extract_key("a b c", &config);
        assert!(key.is_none());
    }

    #[test]
    fn help_output_contains_defaults() {
        let mut buffer = Vec::new();
        write_usage(&mut buffer).expect("write");
        let text = String::from_utf8(buffer).expect("utf8");
        assert!(text.contains("default: 10"));
        assert!(text.contains("50-2000"));
    }

    #[test]
    fn decode_line_replaces_invalid_utf8() {
        let bytes = vec![b'f', b'o', 0xff, b'o', b'\n'];
        let line = decode_line(&bytes);
        assert!(line.starts_with("fo"));
        assert!(line.ends_with("o\n"));
    }

    #[test]
    fn integration_access_log_top_paths() {
        let config = Config {
            field: Some(7),
            delimiter: DEFAULT_DELIMITER,
            top: 3,
            interval_ms: 200,
        };
        let input = include_str!("../samples/access.log");
        let counts = tally_from_input(input, &config);
        let top = top_n(&counts, config.top);
        assert_eq!(top[0], ("/api/v1/users".to_string(), 4));
        assert_eq!(top[1], ("/api/v1/orders".to_string(), 3));
        assert_eq!(top[2], ("/health".to_string(), 2));
    }

    #[test]
    fn integration_app_log_levels() {
        let config = Config {
            field: Some(2),
            delimiter: DEFAULT_DELIMITER,
            top: 3,
            interval_ms: 200,
        };
        let input = include_str!("../samples/app.log");
        let counts = tally_from_input(input, &config);
        let top = top_n(&counts, config.top);
        assert_eq!(top[0], ("ERROR".to_string(), 4));
        assert_eq!(top[1], ("INFO".to_string(), 4));
        assert_eq!(top[2], ("WARN".to_string(), 2));
    }
}
