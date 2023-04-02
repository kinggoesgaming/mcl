pub fn init_logger(verbose: &clap_verbosity_flag::Verbosity) -> Result<(), log::SetLoggerError> {
    env_logger::builder()
        .filter_level(verbose.log_level_filter())
        .init();

    log::trace!("Initialized env_logger");

    Ok(())
}
