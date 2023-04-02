use clap::error;

#[derive(clap::Parser)]
#[clap(about, author, version)]
pub(crate) struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

impl Cli {
    pub(crate) fn parse() -> Result<Self, error::Error<error::DefaultFormatter>> {
        <Self as clap::Parser>::try_parse()
    }

    pub const fn verbose(&self) -> &clap_verbosity_flag::Verbosity {
        &self.verbose
    }
}
