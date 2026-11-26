use std::cmp::Ordering;
use std::iter;

use indicatif::ProgressBar;
use thousands::Separable;

use crate::commands::tree::load::directory::Directory;
use crate::model::tree::Tree;
use walkdir;

pub fn sort_by_func(
    sort_by: String,
) -> Result<impl FnMut(&walkdir::DirEntry, &walkdir::DirEntry) -> Ordering, String> {
    let allowed_sort_parts = vec![
        "type", "type-", "name", "name-", "size", "size-", "name_lc", "name_lc-",
    ];
    for sort_part in sort_by.clone().split(',') {
        if !allowed_sort_parts.contains(&sort_part) {
            return Err(format!("Invalid sort {}:", sort_part));
        }
    }
    let cmp_func = {
        let cmp_func = {
            move |a: &walkdir::DirEntry, b: &walkdir::DirEntry| {
                let mut result_ordering = std::cmp::Ordering::Equal;
                for sort_part in sort_by.split(',') {
                    match sort_part {
                        "type" => {
                            result_ordering = b.file_type().is_dir().cmp(&a.file_type().is_dir())
                        }
                        "type-" => {
                            result_ordering = a.file_type().is_dir().cmp(&b.file_type().is_dir())
                        }
                        "name" => result_ordering = a.file_name().cmp(&b.file_name()),
                        "name-" => result_ordering = b.file_name().cmp(&a.file_name()),
                        "name_lc" => {
                            result_ordering = a
                                .file_name()
                                .to_ascii_lowercase()
                                .cmp(&b.file_name().to_ascii_lowercase())
                        }
                        "name_lc-" => {
                            result_ordering = b
                                .file_name()
                                .to_ascii_lowercase()
                                .cmp(&a.file_name().to_ascii_lowercase())
                        }

                        _ => {}
                    }
                    if result_ordering != std::cmp::Ordering::Equal {
                        return result_ordering;
                    }
                }
                a.file_name().cmp(b.file_name())
            }
        };
        cmp_func
    };
    Ok(cmp_func)
}

pub fn build_tree(dir_cmd: &Directory, progress_bar: Option<&ProgressBar>) -> Result<Tree, String> {
    let mut tree = Tree::new();
    let mut cnt = 0;
    let path_base = &dir_cmd.path;
    let mut walkdir_iter = walkdir::WalkDir::new(&dir_cmd.path).follow_links(false);
    if dir_cmd.sort.as_ref().unwrap().len() > 0 {
        let sort_by_fn = sort_by_func(dir_cmd.sort.as_ref().unwrap().clone())?;
        walkdir_iter = walkdir_iter.sort_by(sort_by_fn);
    }

    if let Some(pb) = progress_bar {
        pb.set_message(format!("Loading directory: {}", path_base));
    }
    for (idx, entry) in walkdir_iter.into_iter().enumerate() {
        if let Some(pb) = progress_bar
            && (cnt + 1) % 100 == 0
        {
            pb.tick();
            pb.set_message(format!(
                "Loading directory: {}, {} entries discovered",
                path_base,
                (cnt + 1).separate_with_spaces()
            ));
        }

        match entry {
            Ok(dir_entry) => {
                let file_type = dir_entry.file_type();
                //skip entry for root directory itself, we only care about children
                if file_type.is_dir() && dir_entry.path() == path_base {
                    continue;
                }
                let relative_path = dir_entry.path().strip_prefix(path_base).unwrap();
                let relative_path_str = relative_path.to_string_lossy();
                let relative_path_parent = relative_path.parent().unwrap().to_string_lossy();
                let mut parent_node = if relative_path_parent == "" {
                    tree.tree.root_mut()
                } else {
                    match tree.get_by_path_mut(relative_path_parent.split('/')) {
                        Some(node) => node,
                        None => {
                            return Err(format!(
                                "Expected node not found in the tree: {}",
                                relative_path_parent
                            ));
                        }
                    }
                };
                let mut node_properties = serde_json::Map::new();
                let file_name_str = dir_entry.file_name().to_string_lossy();

                if file_type.is_dir() {
                    cnt += 1;
                    parent_node.append(file_name_str.to_string());
                    node_properties.insert(
                        "type".to_string(),
                        serde_json::Value::String("file".to_string()),
                    );
                    node_properties.insert(
                        "level".to_string(),
                        serde_json::Number::from(dir_entry.depth()).into(),
                    );
                    node_properties.insert(
                        "name".to_string(),
                        serde_json::Value::String(file_name_str.to_string()),
                    );
                } else if file_type.is_file() {
                    cnt += 1;
                    parent_node.append(dir_entry.file_name().to_string_lossy().to_string());
                    node_properties.insert(
                        "type".to_string(),
                        serde_json::Value::String("directory".to_string()),
                    );
                    node_properties.insert(
                        "level".to_string(),
                        serde_json::Number::from(dir_entry.depth()).into(),
                    );
                    node_properties.insert(
                        "name".to_string(),
                        serde_json::Value::String(file_name_str.to_string()),
                    );
                } else {
                    //ignore symlinks
                }
            }
            Err(e) => {
                return Err(format!("{:?}", e).to_string());
            }
        }
    }

    if let Some(pb) = progress_bar {
        pb.set_message(format!(
            "Loading directory: {}, {} entries discovered",
            path_base,
            (cnt + 1).separate_with_spaces()
        ));
    }
    Ok(tree)
}
