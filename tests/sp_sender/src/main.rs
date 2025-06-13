mod wallet;
mod rpc;
mod metadata;
mod test;

use crate::rpc::Connection;
use dotenvy;
use std::env;

fn main() -> anyhow::Result<()> {
    dotenvy::from_filename(".private/.env").ok();
    let rpc_url = env::var("RPC_URL")?;
    let connection = Connection::new(&rpc_url);

    // Choose which test to run
    //test::test1::run(&connection)?;  // Run test1 -> Test with 3 SP, one sending 0% one 34% and one 66%.
    //test::test2::run(&connection)?;  // Uncomment to run test2 instead
    //test::test3::run(&connection)?;  // Uncomment to run test2 instead
    //test::test4::run(&connection)?;  // Uncomment to run test2 instead
    test::test5::run(&connection)?;  // Uncomment to run test2 instead

    Ok(())
}
