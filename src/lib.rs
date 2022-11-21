// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![doc = include_str!("../README.md")]

pub mod errors;
pub mod maestro;

/// ### Purpose:
/// The global [`Result`] type to be used throughout the application.
pub type Result<T> = std::result::Result<T, crate::errors::Error>;
