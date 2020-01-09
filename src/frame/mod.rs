// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

//! Implements support for built-in runtime modules.

use codec::Encode;

pub mod balances;
pub mod contracts;
pub mod identity;
pub mod system;

/// Creates module calls
pub struct Call<T: Encode> {
    /// Module name
    pub module: &'static str,
    /// Function name
    pub function: &'static str,
    /// Call arguments
    pub args: T,
}

impl<T: Encode> Call<T> {
    /// Create a module call
    pub fn new(module: &'static str, function: &'static str, args: T) -> Self {
        Call {
            module,
            function,
            args,
        }
    }
}
