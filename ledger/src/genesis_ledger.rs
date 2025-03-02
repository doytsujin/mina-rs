// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! A Genesis ledger is specialization of a ledger that holds initial account data to be included at genesis
//!
//! A genesis ledger is the only way time locked accounts can be added to the ledger and
//! is also a way to initially allocate funds to accounts
//!

use mina_merkle::*;
use proof_systems::mina_hasher::Hashable;

/// Type alias for mina merkle ledger hasher
pub type MinaLedgerMerkleHasherLegacy<Account> = MinaPoseidonMerkleHasherLegacy<Account>;

/// Type alias for mina merkle ledger
pub type MinaLedgerMerkleTreeLegacy<Account> = MinaMerkleTree<
    <MinaLedgerMerkleHasherLegacy<Account> as MerkleHasher>::Item,
    <MinaLedgerMerkleHasherLegacy<Account> as MerkleHasher>::Hash,
    MinaLedgerMerkleHasherLegacy<Account>,
    MinaPoseidonMerkleMergerLegacy,
    FixedHeightMode,
>;

/// Type alias for mina merkle ledger hasher
pub type MinaLedgerMerkleHasher<Account> = MinaPoseidonMerkleHasher<Account>;

/// Type alias for mina merkle ledger
pub type MinaLedgerMerkleTree<Account> = MinaMerkleTree<
    <MinaLedgerMerkleHasher<Account> as MerkleHasher>::Item,
    <MinaLedgerMerkleHasher<Account> as MerkleHasher>::Hash,
    MinaLedgerMerkleHasher<Account>,
    MinaPoseidonMerkleMerger,
    FixedHeightMode,
>;

/// A genesis ledger provides access to its accounts by implementing IntoIterator
/// This implementation must be provided to meet the trait requirements
///
/// A Genesis ledger has a compile time pre-defined depth which is set here as a const generic
/// This ensures compile-time checking that the correct depth ledger is being used in the correc place
pub trait GenesisLedger<'a, const DEPTH: usize, Account: Hashable>
where
    <Account as Hashable>::D: Default,
    Self: 'a,
    &'a Self: IntoIterator<Item = Result<Account, Self::Error>>,
{
    /// Error type that can be produces when trying to access the underlying store
    type Error;

    /// Return the depth of the ledger
    fn depth(&self) -> usize {
        DEPTH
    }

    /// Return a iterator over the accounts in this genesis ledger without consuming self
    fn accounts(&'a self) -> <&'a Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    /// Build legacy mina merkle ledger tree with a fixed height that uses legacy hasher
    fn to_mina_merkle_ledger_legacy(&'a self) -> MinaLedgerMerkleTreeLegacy<Account> {
        // ledger_depth is defined at <https://github.com/MinaProtocol/mina/blob/develop/docs/specs/types_and_structures/serialized_key.md#constraint_constants>
        const MINA_LEDGER_HEIGHT: u32 = 20;

        let mut tree = MinaLedgerMerkleTreeLegacy::new(MINA_LEDGER_HEIGHT);
        tree.add_batch(self.accounts().flatten());
        tree
    }

    /// Build mina merkle ledger tree with a fixed height that uses kimchi hasher
    fn to_mina_merkle_ledger(&'a self) -> MinaLedgerMerkleTree<Account> {
        // ledger_depth is defined at <https://github.com/MinaProtocol/mina/blob/develop/docs/specs/types_and_structures/serialized_key.md#constraint_constants>
        const MINA_LEDGER_HEIGHT: u32 = 20;

        let mut tree = MinaLedgerMerkleTree::new(MINA_LEDGER_HEIGHT);
        tree.add_batch(self.accounts().flatten());
        tree
    }

    // TODO: Add additional methods when they are required
    // https://github.com/MinaProtocol/mina/blob/65b59f56b6e98e1d9648280c2153d809abb42ba3/src/lib/genesis_ledger/intf.ml#L84
}
