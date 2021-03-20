pub mod chess;
pub mod math;
pub mod reaction_roles;
pub mod utils;

use chess::*;
use math::*;
use reaction_roles::*;
use serenity::framework::standard::macros::group;
use utils::*;

#[group("general")]
#[commands(math)]
pub struct General;

#[group("admin")]
#[commands(reactionrole)]
pub struct Admin;

#[group("utility")]
#[commands(ping, ferris)]
pub struct Utility;

#[group("game")]
#[commands(create, stop, start, join, play)]
pub struct Game;
