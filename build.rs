use anyhow::Result;


fn main() -> Result<()> {
    #[cfg(windows)] // Build host machine have to be WindowsOS in order to compile with manifest/icon
    {
        use winres;

        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");

        res.compile()?;
    }

    Ok(())
}
