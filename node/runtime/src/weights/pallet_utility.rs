// Copyright 2019-2021 Stafi Protocol.
// This file is part of Stafi.

// Stafi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Stafi.  If not, see <http://www.gnu.org/licenses/>.

//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0-rc5

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

pub struct WeightInfo;
impl pallet_utility::WeightInfo for WeightInfo {
	fn batch(c: u32, ) -> Weight {
		(16461000 as Weight)
			.saturating_add((1982000 as Weight).saturating_mul(c as Weight))
	}
	// WARNING! Some components were not used: ["u"]
	fn as_derivative() -> Weight {
		(4086000 as Weight)
	}
}
