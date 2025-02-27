//! Autogenerated weights for `pallet_collator_selection`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-11-03, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-15-118`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// target/release/generic-template-node
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=benchmarking/results/results-pallet_collator_selection.json
// --pallet=pallet_collator_selection
// --chain=dev
// --output=benchmarking/new-benchmarks/pallet_collator_selection.rs

// #![cfg_attr(rustfmt, rustfmt_skip)]
// #![allow(unused_parens)]
// #![allow(unused_imports)]
// #![allow(missing_docs)]

use core::marker::PhantomData;

use frame_support::{traits::Get, weights::Weight};

/// Weight functions for `pallet_collator_selection`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collator_selection::WeightInfo for WeightInfo<T> {
    /// Storage: `Session::NextKeys` (r:20 w:0)
    /// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `CollatorSelection::Invulnerables` (r:0 w:1)
    /// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
    /// The range of component `b` is `[1, 20]`.
    fn set_invulnerables(b: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `197 + b * (79 ±0)`
        //  Estimated: `1188 + b * (2555 ±0)`
        // Minimum execution time: 15_225_000 picoseconds.
        Weight::from_parts(11_571_267, 0)
            .saturating_add(Weight::from_parts(0, 1188))
            // Standard Error: 5_469
            .saturating_add(Weight::from_parts(4_047_420, 0).saturating_mul(b.into()))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b.into())))
            .saturating_add(T::DbWeight::get().writes(1))
            .saturating_add(Weight::from_parts(0, 2555).saturating_mul(b.into()))
    }

    /// Storage: `Session::NextKeys` (r:1 w:0)
    /// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `CollatorSelection::Invulnerables` (r:1 w:1)
    /// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
    /// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// The range of component `b` is `[1, 19]`.
    /// The range of component `c` is `[1, 99]`.
    fn add_invulnerable(b: u32, c: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `828 + b * (32 ±0) + c * (53 ±0)`
        //  Estimated: `6287 + b * (37 ±0) + c * (53 ±0)`
        // Minimum execution time: 51_115_000 picoseconds.
        Weight::from_parts(46_321_661, 0)
            .saturating_add(Weight::from_parts(0, 6287))
            // Standard Error: 18_486
            .saturating_add(Weight::from_parts(346_609, 0).saturating_mul(b.into()))
            // Standard Error: 3_504
            .saturating_add(Weight::from_parts(202_188, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
            .saturating_add(Weight::from_parts(0, 37).saturating_mul(b.into()))
            .saturating_add(Weight::from_parts(0, 53).saturating_mul(c.into()))
    }

    /// Storage: `CollatorSelection::CandidateList` (r:1 w:0)
    /// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::Invulnerables` (r:1 w:1)
    /// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
    /// The range of component `b` is `[5, 20]`.
    fn remove_invulnerable(b: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `153 + b * (32 ±0)`
        //  Estimated: `6287`
        // Minimum execution time: 14_756_000 picoseconds.
        Weight::from_parts(14_469_493, 0)
            .saturating_add(Weight::from_parts(0, 6287))
            // Standard Error: 2_837
            .saturating_add(Weight::from_parts(214_011, 0).saturating_mul(b.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }

    /// Storage: `CollatorSelection::DesiredCandidates` (r:0 w:1)
    /// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    fn set_desired_candidates() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 6_561_000 picoseconds.
        Weight::from_parts(6_777_000, 0)
            .saturating_add(Weight::from_parts(0, 0))
            .saturating_add(T::DbWeight::get().writes(1))
    }

    /// Storage: `CollatorSelection::CandidacyBond` (r:1 w:1)
    /// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
    /// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:100 w:100)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:100)
    /// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
    /// The range of component `c` is `[0, 100]`.
    /// The range of component `k` is `[0, 100]`.
    fn set_candidacy_bond(c: u32, k: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0 + c * (182 ±0) + k * (115 ±0)`
        //  Estimated: `6287 + c * (901 ±29) + k * (901 ±29)`
        // Minimum execution time: 13_013_000 picoseconds.
        Weight::from_parts(13_150_000, 0)
            .saturating_add(Weight::from_parts(0, 6287))
            // Standard Error: 212_739
            .saturating_add(Weight::from_parts(7_127_589, 0).saturating_mul(c.into()))
            // Standard Error: 212_739
            .saturating_add(Weight::from_parts(6_784_046, 0).saturating_mul(k.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(k.into())))
            .saturating_add(Weight::from_parts(0, 901).saturating_mul(c.into()))
            .saturating_add(Weight::from_parts(0, 901).saturating_mul(k.into()))
    }

    /// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
    /// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
    /// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
    /// The range of component `c` is `[3, 100]`.
    fn update_bond(c: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `356 + c * (49 ±0)`
        //  Estimated: `6287`
        // Minimum execution time: 34_412_000 picoseconds.
        Weight::from_parts(37_538_467, 0)
            .saturating_add(Weight::from_parts(0, 6287))
            // Standard Error: 2_658
            .saturating_add(Weight::from_parts(140_363, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }

    /// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
    /// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
    /// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
    /// Storage: `Session::NextKeys` (r:1 w:0)
    /// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
    /// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
    /// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
    /// The range of component `c` is `[1, 99]`.
    fn register_as_candidate(c: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `799 + c * (52 ±0)`
        //  Estimated: `6287 + c * (54 ±0)`
        // Minimum execution time: 45_366_000 picoseconds.
        Weight::from_parts(49_711_930, 0)
            .saturating_add(Weight::from_parts(0, 6287))
            // Standard Error: 3_239
            .saturating_add(Weight::from_parts(180_195, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(2))
            .saturating_add(Weight::from_parts(0, 54).saturating_mul(c.into()))
    }

    /// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
    /// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
    /// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `Session::NextKeys` (r:1 w:0)
    /// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
    /// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:2)
    /// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
    /// The range of component `c` is `[3, 100]`.
    fn take_candidate_slot(c: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `939 + c * (53 ±0)`
        //  Estimated: `6287 + c * (54 ±0)`
        // Minimum execution time: 68_436_000 picoseconds.
        Weight::from_parts(73_112_305, 0)
            .saturating_add(Weight::from_parts(0, 6287))
            // Standard Error: 4_118
            .saturating_add(Weight::from_parts(214_419, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(5))
            .saturating_add(T::DbWeight::get().writes(4))
            .saturating_add(Weight::from_parts(0, 54).saturating_mul(c.into()))
    }

    /// Storage: `CollatorSelection::CandidateList` (r:1 w:1)
    /// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
    /// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
    /// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
    /// The range of component `c` is `[3, 100]`.
    fn leave_intent(c: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `381 + c * (48 ±0)`
        //  Estimated: `6287`
        // Minimum execution time: 37_349_000 picoseconds.
        Weight::from_parts(41_362_337, 0)
            .saturating_add(Weight::from_parts(0, 6287))
            // Standard Error: 3_309
            .saturating_add(Weight::from_parts(158_689, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }

    /// Storage: `System::Account` (r:2 w:2)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
    /// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
    fn note_author() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `192`
        //  Estimated: `6196`
        // Minimum execution time: 56_862_000 picoseconds.
        Weight::from_parts(57_890_000, 0)
            .saturating_add(Weight::from_parts(0, 6196))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(3))
    }

    /// Storage: `CollatorSelection::CandidateList` (r:1 w:0)
    /// Proof: `CollatorSelection::CandidateList` (`max_values`: Some(1), `max_size`: Some(4802), added: 5297, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::LastAuthoredBlock` (r:100 w:0)
    /// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
    /// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
    /// Storage: `CollatorSelection::DesiredCandidates` (r:1 w:0)
    /// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:97 w:97)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// The range of component `r` is `[1, 100]`.
    /// The range of component `c` is `[1, 100]`.
    fn new_session(r: u32, c: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `2369 + c * (97 ±0) + r * (114 ±0)`
        //  Estimated: `6287 + c * (2519 ±0) + r * (2603 ±0)`
        // Minimum execution time: 23_061_000 picoseconds.
        Weight::from_parts(23_816_000, 0)
            .saturating_add(Weight::from_parts(0, 6287))
            // Standard Error: 378_020
            .saturating_add(Weight::from_parts(16_325_603, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
            .saturating_add(Weight::from_parts(0, 2519).saturating_mul(c.into()))
            .saturating_add(Weight::from_parts(0, 2603).saturating_mul(r.into()))
    }
}
