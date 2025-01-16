use indicatif::{ProgressBar, ProgressStyle};

pub fn create_spinner(msg: &str) -> Result<ProgressBar, indicatif::style::TemplateError> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        indicatif::ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
            .template("{spinner:.green} {msg}")?,
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));
    spinner.set_message(msg.to_string());
    Ok(spinner)
}

pub fn pb_with_len(msg: &str, len: u64) -> Result<ProgressBar, indicatif::style::TemplateError> {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")?
            .progress_chars("#>-"),
    );
    pb.set_message(msg.to_string());

    Ok(pb)
}
