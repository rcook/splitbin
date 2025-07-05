mod args;
mod run;
mod util;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
