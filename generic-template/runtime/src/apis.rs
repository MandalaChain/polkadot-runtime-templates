use openzeppelin_pallet_abstractions_proc::openzeppelin_runtime_apis;

#[cfg(all(not(feature = "async-backing"), not(feature = "tanssi")))]
use crate::Aura;
#[cfg(feature = "runtime-benchmarks")]
use crate::{
    configs::xcm_config::RelayLocation,
    constants::currency::{CENTS, EXISTENTIAL_DEPOSIT},
};
#[cfg(feature = "async-backing")]
use crate::{constants::SLOT_DURATION, types::ConsensusHook};
use crate::{
    constants::VERSION,
    types::{AccountId, AssetId, Balance, Block, Executive, Nonce},
    InherentDataExt, Oracle, ParachainSystem, Runtime, RuntimeBlockWeights, RuntimeCall,
    RuntimeGenesisConfig, SessionKeys, System, TransactionPayment,
};

#[cfg(feature = "runtime-benchmarks")]
type ExistentialDeposit = sp_core::ConstU128<EXISTENTIAL_DEPOSIT>;

#[cfg(not(feature = "tanssi"))]
#[openzeppelin_runtime_apis]
mod apis {
    type Runtime = Runtime;
    type Block = Block;

    mod assets {
        type RuntimeCall = RuntimeCall;
        type TransactionPayment = TransactionPayment;
        type Balance = Balance;
        type Oracle = Oracle;
        type OracleKey = AssetId;
    }

    mod consensus {
        type SessionKeys = SessionKeys;
        #[cfg(not(feature = "async-backing"))]
        type Aura = Aura;
        #[cfg(feature = "async-backing")]
        type SlotDuration = SLOT_DURATION;
        #[cfg(feature = "async-backing")]
        type ConsensusHook = ConsensusHook;
    }

    mod system {
        type Executive = Executive;
        type System = System;
        type ParachainSystem = ParachainSystem;
        type RuntimeVersion = VERSION;
        type AccountId = AccountId;
        type Nonce = Nonce;
        type RuntimeGenesisConfig = RuntimeGenesisConfig;
        type RuntimeBlockWeights = RuntimeBlockWeights;
    }

    mod benchmarks {
        type AllPalletsWithSystem = AllPalletsWithSystem;
        type Assets = Assets;
        type AssetManager = AssetManager;
        type AssetType = AssetType;
        type RuntimeOrigin = RuntimeOrigin;
        type RelayLocation = RelayLocation;
        type ParachainSystem = ParachainSystem;
        type System = System;
        type ExistentialDeposit = ExistentialDeposit;
        type AssetId = AssetId;
        type XCMConfig = XcmExecutorConfig;
        type AccountId = AccountId;
        type Cents = CENTS;
        type FeeAssetId = FeeAssetId;
        type TransactionByteFee = TransactionByteFee;
        type Address = Address;
        type Balances = Balances;
    }
}

#[cfg(feature = "tanssi")]
#[openzeppelin_runtime_apis]
mod apis {
    type Runtime = Runtime;
    type Block = Block;

    mod assets {
        type RuntimeCall = RuntimeCall;
        type TransactionPayment = TransactionPayment;
        type Balance = Balance;
        type Oracle = Oracle;
        type OracleKey = AssetId;
    }

    mod tanssi {
        type SessionKeys = SessionKeys;
    }

    mod system {
        type Executive = Executive;
        type System = System;
        type ParachainSystem = ParachainSystem;
        type RuntimeVersion = VERSION;
        type AccountId = AccountId;
        type Nonce = Nonce;
        type RuntimeGenesisConfig = RuntimeGenesisConfig;
        type RuntimeBlockWeights = RuntimeBlockWeights;
    }

    mod benchmarks {
        type AllPalletsWithSystem = AllPalletsWithSystem;
        type Assets = Assets;
        type AssetManager = AssetManager;
        type AssetType = AssetType;
        type RuntimeOrigin = RuntimeOrigin;
        type RelayLocation = RelayLocation;
        type ParachainSystem = ParachainSystem;
        type System = System;
        type ExistentialDeposit = ExistentialDeposit;
        type AssetId = AssetId;
        type XCMConfig = XcmExecutorConfig;
        type AccountId = AccountId;
        type Cents = CENTS;
        type FeeAssetId = FeeAssetId;
        type TransactionByteFee = TransactionByteFee;
        type Address = Address;
        type Balances = Balances;
    }
}
