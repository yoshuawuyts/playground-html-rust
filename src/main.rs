#![feature(async_await, futures_api)]
#![recursion_limit = "256"]
#![feature(proc_macro_hygiene, decl_macro)]

use exitfailure::ExitFailure;
use human_panic::setup_panic;
use log::info;
use playground_html_rust::{index, Cli};
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
  setup_panic!();

  let args = Cli::from_args();
  args.log(env!("CARGO_PKG_NAME"))?;
  info!("program started");

  let mut app = tide::App::new(());
  app.at("/").get(index);
  app.serve("127.0.0.1:8000");

  Ok(())
}
