use ciri::args::package::Run;
use ciri::validators::detect_language;

pub fn run(args: Run) -> miette::Result<()> {
    let lang = detect_language()?;
    // info!("{:#?}", args);
    info!("{:#?}", lang);

    Ok(())
}
