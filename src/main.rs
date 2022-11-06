
#![allow(unused)]
use ggez::{Context, ContextBuilder, GameResult, GameError};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez;
use ggez::input::keyboard::{self, KeyCode};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{io};
use std::collections::HashMap;
use rand::{self, thread_rng,Rng};
//use serde_derive;
use serde::{Deserialize, Serialize};
//use serde_json::{Result, Value};


fn main() -> GameResult {
    // Make a Context.
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Rusty Silly Wars", "Chris Harty")
        .build()
        .expect("aieee, could not create ggez context!");

    Ok(())
    
}
