use std::fs;

fn main() -> std::io::Result<()> {
    fs::create_dir("../CIOAI/DATACORE/")?;
    fs::write("../CIOAI/DATACORE/what.txt", b"The datacore is initialized.")?;
    fs::write("../CIOAI/DATACORE/data.7z", b"")?;
    Ok(())
}
