#![no_std]
#![warn(
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
    // clippy::cargo,
    unused,
    rust_2018_idioms,
    future_incompatible,
    // missing_docs
)]
#![allow(
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,
    clippy::unwrap_used,
    clippy::unseparated_literal_suffix,
    clippy::self_named_module_files,
    clippy::unwrap_in_result,
    clippy::missing_inline_in_public_items,
    // clippy::panic_in_result_fn,
    clippy::question_mark_used,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::must_use_candidate,
    // docs:
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

pub mod tokens;
pub mod tx_manager;
