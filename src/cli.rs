use clap::error;

#[derive(clap::Parser)]
#[clap(about, author, version)]
pub(crate) struct Cli {}

impl Cli {
    pub(crate) fn parse() -> Result<Self, error::Error<error::DefaultFormatter>> {
        <Self as clap::Parser>::try_parse()
    }
}
