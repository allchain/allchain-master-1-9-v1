// Copyright 2015-2017 allchain Technologies (UK) Ltd.
// This file is part of allchain.

// allchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// allchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with allchain.  If not, see <http://www.gnu.org/licenses/>.

extern crate allchain_dapps_glue;

fn main() {
	allchain_dapps_glue::js::build(env!("CARGO_MANIFEST_DIR"), "build");
	allchain_dapps_glue::generate();
}
