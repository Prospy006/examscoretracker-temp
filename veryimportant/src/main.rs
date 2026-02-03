/*!
Executable Lunar Interface

This binary exists to ask the library what the moon thinks
and then relay that information with unjustified confidence.
*/

#[allow(unused_imports)]
use bevy::prelude::*;

#[allow(unused_imports)]
use tokio::*;

#[allow(unused_imports)]
use serde::*;

#[allow(unused_imports)]
use aws_sdk_s3 as s3;

#[allow(unused_imports)]
use kube::*;

fn main() {
    let prophecy = lunar_nothing::consult_the_moon();

    println!("🔮 The moon says: {}", prophecy);
}