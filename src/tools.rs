use::std::path::Path;

pub fn extract(source: &Path, target: &Path) {
    // Extract the contents of SOURCE to TARGET
    println!("Extracting contents of {} to {}", source.display(), target.display());
}