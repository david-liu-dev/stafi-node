// Copyright 2019-2021 Stafi Protocol.
// This file is part of Stafi.

// Stafi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Stafi.  If not, see <http://www.gnu.org/licenses/>.

//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0-rc6

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

pub struct WeightInfo;
impl pallet_collective::WeightInfo for WeightInfo {
	fn set_members(m: u32, n: u32, p: u32, ) -> Weight {
		(0 as Weight)
			.saturating_add((21040000 as Weight).saturating_mul(m as Weight))
			.saturating_add((173000 as Weight).saturating_mul(n as Weight))
			.saturating_add((31595000 as Weight).saturating_mul(p as Weight))
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
			.saturating_add(DbWeight::get().writes(2 as Weight))
			.saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	fn execute(b: u32, m: u32, ) -> Weight {
		(43359000 as Weight)
			.saturating_add((4000 as Weight).saturating_mul(b as Weight))
			.saturating_add((123000 as Weight).saturating_mul(m as Weight))
			.saturating_add(DbWeight::get().reads(1 as Weight))
	}
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		(54134000 as Weight)
			.saturating_add((4000 as Weight).saturating_mul(b as Weight))
			.saturating_add((239000 as Weight).saturating_mul(m as Weight))
			.saturating_add(DbWeight::get().reads(2 as Weight))
	}
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		(90650000 as Weight)
			.saturating_add((5000 as Weight).saturating_mul(b as Weight))
			.saturating_add((152000 as Weight).saturating_mul(m as Weight))
			.saturating_add((970000 as Weight).saturating_mul(p as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	fn vote(m: u32, ) -> Weight {
		(74460000 as Weight)
			.saturating_add((290000 as Weight).saturating_mul(m as Weight))
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		(86360000 as Weight)
			.saturating_add((232000 as Weight).saturating_mul(m as Weight))
			.saturating_add((954000 as Weight).saturating_mul(p as Weight))
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(123653000 as Weight)
			.saturating_add((1000 as Weight).saturating_mul(b as Weight))
			.saturating_add((287000 as Weight).saturating_mul(m as Weight))
			.saturating_add((920000 as Weight).saturating_mul(p as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		(95395000 as Weight)
			.saturating_add((236000 as Weight).saturating_mul(m as Weight))
			.saturating_add((965000 as Weight).saturating_mul(p as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(135284000 as Weight)
			.saturating_add((4000 as Weight).saturating_mul(b as Weight))
			.saturating_add((218000 as Weight).saturating_mul(m as Weight))
			.saturating_add((951000 as Weight).saturating_mul(p as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn disapprove_proposal(p: u32, ) -> Weight {
		(50500000 as Weight)
			.saturating_add((966000 as Weight).saturating_mul(p as Weight))
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
}
