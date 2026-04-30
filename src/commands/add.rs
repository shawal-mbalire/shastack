use anyhow::Result;
use crate::workspace;

pub fn exec(feature: String) -> Result<()> {
    let root = workspace::find_root()?;
    println!("Adding feature {} to workspace at {:?}", feature, root);

    workspace::add_feature(&root, &feature)?;

    println!("Feature {} added successfully!", feature);
    Ok(())
}
