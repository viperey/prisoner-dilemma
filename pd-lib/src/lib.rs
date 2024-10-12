//! Rust implementation of the Prisoner's Dilemma.
//!
//! Inspired by [Veritasium's video](https://www.youtube.com/watch?v=mScpHTIi-kM).
//!
//! Entities topology: Generation > Tournament > Match > Game > Round

//#![deny(missing_docs)]
#![deny(rustdoc::all)]
#![allow(dead_code)]

pub mod domain;
pub mod evolution;
pub mod r#match;
pub mod prisoner;

mod game;
mod game_result;
mod generation;
mod round;
mod strategies;
mod strategy;
mod tournament;
