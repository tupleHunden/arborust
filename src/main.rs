use ignore::WalkBuilder;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let binding = env::args().nth(1);
    let path = binding.as_deref().unwrap_or(".");
    let entries = collect_entries(path);
    print_tree(&entries);
}

fn collect_entries(root: &str) -> Vec<PathBuf> {
    WalkBuilder::new(root)
        .git_ignore(true)
        .git_exclude(true)
        .git_global(true)
        .build()
        .filter_map(|entry| match entry {
            Ok(e) => {
                if e.path() != Path::new(root) {
                    return Some(e.into_path());
                }

                None
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                None
            }
        })
        .collect()
}

fn print_tree(entries: &[PathBuf]) {
    for (idx, path) in entries.iter().enumerate() {
        let current_components: Vec<_> = path.components().collect();
        let depth = current_components.len();

        let has_next_sibling = entries
            .get(idx + 1)
            .map(|next_path| next_path.components().take(depth).eq(path.components()))
            .unwrap_or(false);

        let prefix: String = (0..depth - 1)
            .map(|d| {
                let has_sibling_or_child_below = entries.iter().skip(idx + 1).any(|next_entry| {
                    next_entry
                        .components()
                        .take(d + 1)
                        .eq(current_components.iter().cloned().take(d + 1))
                });
                if has_sibling_or_child_below {
                    "│  "
                } else {
                    "   "
                }
            })
            .collect();

        let symbol = if has_next_sibling {
            "├── "
        } else {
            "└── "
        };

        println!(
            "{}{}{}",
            prefix,
            symbol,
            current_components
                .last()
                .unwrap()
                .as_os_str()
                .to_string_lossy()
        );
    }
}
