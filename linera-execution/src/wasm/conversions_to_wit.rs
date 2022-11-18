// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Conversions to types generated by `wit-bindgen`.
//!
//! Allows converting types used in `linera-execution` to types that can be sent to the guest WASM
//! module.

use super::runtime::{contract, service};
use crate::{CalleeContext, EffectContext, EffectId, OperationContext, QueryContext, SessionId};
use linera_base::{
    crypto::HashValue,
    messages::{ApplicationId, ChainId},
};

impl From<OperationContext> for contract::OperationContext {
    fn from(host: OperationContext) -> Self {
        contract::OperationContext {
            chain_id: host.chain_id.into(),
            height: host.height.0,
            index: host
                .index
                .try_into()
                .expect("Operation index should fit in an `u64`"),
        }
    }
}

impl From<EffectContext> for contract::EffectContext {
    fn from(host: EffectContext) -> Self {
        contract::EffectContext {
            chain_id: host.chain_id.into(),
            height: host.height.0,
            effect_id: host.effect_id.into(),
        }
    }
}

impl From<EffectId> for contract::EffectId {
    fn from(host: EffectId) -> Self {
        contract::EffectId {
            chain_id: host.chain_id.into(),
            height: host.height.0,
            index: host
                .index
                .try_into()
                .expect("Effect index should fit in an `u64`"),
        }
    }
}

impl From<CalleeContext> for contract::CalleeContext {
    fn from(host: CalleeContext) -> Self {
        contract::CalleeContext {
            chain_id: host.chain_id.into(),
            authenticated_caller_id: host
                .authenticated_caller_id
                .map(contract::ApplicationId::from),
        }
    }
}

impl From<QueryContext> for service::QueryContext {
    fn from(host: QueryContext) -> Self {
        service::QueryContext {
            chain_id: host.chain_id.into(),
        }
    }
}

impl From<SessionId> for contract::SessionId {
    fn from(host: SessionId) -> Self {
        contract::SessionId {
            application_id: host.application_id.into(),
            kind: host.kind,
            index: host.index,
        }
    }
}

impl From<ApplicationId> for contract::ApplicationId {
    fn from(host: ApplicationId) -> Self {
        match host {
            ApplicationId::System => contract::ApplicationId::System,
            ApplicationId::User { bytecode, creation } => {
                contract::ApplicationId::User(contract::UserApplicationId {
                    bytecode: bytecode.0.into(),
                    creation: creation.into(),
                })
            }
        }
    }
}

impl From<ChainId> for contract::ChainId {
    fn from(chain_id: ChainId) -> Self {
        chain_id.0.into()
    }
}

impl From<ChainId> for service::ChainId {
    fn from(chain_id: ChainId) -> Self {
        chain_id.0.into()
    }
}

impl From<HashValue> for contract::HashValue {
    fn from(hash_value: HashValue) -> Self {
        let bytes = hash_value.as_bytes();

        contract::HashValue {
            part1: u64::from_le_bytes(bytes[0..8].try_into().expect("incorrect indices")),
            part2: u64::from_le_bytes(bytes[8..16].try_into().expect("incorrect indices")),
            part3: u64::from_le_bytes(bytes[16..24].try_into().expect("incorrect indices")),
            part4: u64::from_le_bytes(bytes[24..32].try_into().expect("incorrect indices")),
            part5: u64::from_le_bytes(bytes[32..40].try_into().expect("incorrect indices")),
            part6: u64::from_le_bytes(bytes[40..48].try_into().expect("incorrect indices")),
            part7: u64::from_le_bytes(bytes[48..56].try_into().expect("incorrect indices")),
            part8: u64::from_le_bytes(bytes[56..64].try_into().expect("incorrect indices")),
        }
    }
}

impl From<HashValue> for service::HashValue {
    fn from(hash_value: HashValue) -> Self {
        let bytes = hash_value.as_bytes();

        service::HashValue {
            part1: u64::from_le_bytes(bytes[0..8].try_into().expect("incorrect indices")),
            part2: u64::from_le_bytes(bytes[8..16].try_into().expect("incorrect indices")),
            part3: u64::from_le_bytes(bytes[16..24].try_into().expect("incorrect indices")),
            part4: u64::from_le_bytes(bytes[24..32].try_into().expect("incorrect indices")),
            part5: u64::from_le_bytes(bytes[32..40].try_into().expect("incorrect indices")),
            part6: u64::from_le_bytes(bytes[40..48].try_into().expect("incorrect indices")),
            part7: u64::from_le_bytes(bytes[48..56].try_into().expect("incorrect indices")),
            part8: u64::from_le_bytes(bytes[56..64].try_into().expect("incorrect indices")),
        }
    }
}
