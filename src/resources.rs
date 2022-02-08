
use crate::identifier::FileId;

pub fn get_resource_path(tmp_path: &std::path::Path) -> eyre::Result<()> {
    let mut resource_path = std::env::current_dir()?;
    let mut full_tmp_path = resource_path.clone();
    full_tmp_path.push(tmp_path);
    let file_contents = std::fs::read(&full_tmp_path)?;
    let id = FileId::new(&file_contents);
    resource_path.push(id.dir());
    std::fs::create_dir_all(&resource_path)?;
    resource_path.push(id.name());

    if !resource_path.is_file() {
        std::fs::hard_link(&full_tmp_path, &resource_path)?;
    }

    std::fs::remove_file(&full_tmp_path)?;
    let mut final_path = full_tmp_path.clone();
    let stem = full_tmp_path.file_stem().unwrap();
    final_path.pop();
    final_path.push(stem);
    std::fs::hard_link(resource_path, final_path)?;
    Ok(())
}
