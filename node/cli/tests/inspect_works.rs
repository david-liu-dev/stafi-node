// Copyright 2019-2021 Stafi Protocol.
// This file is part of Stafi.

// Stafi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Stafi.  If not, see <http://www.gnu.org/licenses/>.

#![cfg(unix)]

use assert_cmd::cargo::cargo_bin;
use std::process::Command;
use tempfile::tempdir;

pub mod common;

#[test]
fn inspect_works() {
	let base_path = tempdir().expect("could not create a temp dir");

	common::run_dev_node_for_a_while(base_path.path());

	let status = Command::new(cargo_bin("stafi"))
		.args(&["inspect", "--dev", "--pruning", "archive", "-d"])
		.arg(base_path.path())
		.args(&["block", "1"])
		.status()
		.unwrap();
	assert!(status.success());
}
