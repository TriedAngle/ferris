// pub mod chess;
pub mod code;
pub mod math;
pub mod utils;

// use chess::*;
use code::*;
use math::*;
use serenity::framework::standard::macros::group;
use utils::*;

#[group("general")]
#[commands(math, code)]
pub struct General;

#[group("admin")]
// #[commands(reactionrole)]
pub struct Admin;

#[group("utility")]
#[commands(ping, ferris)]
pub struct Utility;

#[group("game")]
// #[commands(create, stop, start, join, play)]
pub struct Game;
