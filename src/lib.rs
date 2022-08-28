#![allow(
dead_code,
unused_imports
)]


//! Core Library for the Textual Object Ecosystem
//!
//! This library is the core library for the Textual Object Ecosystem.
//!
//! # Main Features
//!
//! - Textual Object Machine
//!
//! - Textual Object Parser
//!
//! - Textual Object Ticket
//!
//! - Textual Object DB
//!
//! - Textual Object Tag
//!
//! - Textual Object Card

pub mod entities;
pub(crate) mod utils;
pub mod to_machine;
pub mod to_ticket;
pub mod enums;
pub mod to;
pub mod db;
pub mod to_card;
mod to_tag;
pub mod error;
pub mod to_parser;


