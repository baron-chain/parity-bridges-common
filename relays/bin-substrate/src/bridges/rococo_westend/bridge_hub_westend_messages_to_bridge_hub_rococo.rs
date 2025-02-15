// Copyright 2022 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! BridgeHubWestend-to-BridgeHubRococo messages sync entrypoint.

use crate::cli::bridge::{CliBridgeBase, MessagesCliBridge};
use relay_bridge_hub_rococo_client::BridgeHubRococo;
use relay_bridge_hub_westend_client::BridgeHubWestend;
use substrate_relay_helper::{messages::SubstrateMessageLane, UtilityPalletBatchCallBuilder};

pub struct BridgeHubWestendToBridgeHubRococoMessagesCliBridge {}

impl CliBridgeBase for BridgeHubWestendToBridgeHubRococoMessagesCliBridge {
	type Source = BridgeHubWestend;
	type Target = BridgeHubRococo;
}

impl MessagesCliBridge for BridgeHubWestendToBridgeHubRococoMessagesCliBridge {
	type MessagesLane = BridgeHubWestendMessagesToBridgeHubRococoMessageLane;
}

substrate_relay_helper::generate_receive_message_proof_call_builder!(
	BridgeHubWestendMessagesToBridgeHubRococoMessageLane,
	BridgeHubWestendMessagesToBridgeHubRococoMessageLaneReceiveMessagesProofCallBuilder,
	relay_bridge_hub_rococo_client::RuntimeCall::BridgeWestendMessages,
	relay_bridge_hub_rococo_client::BridgeMessagesCall::receive_messages_proof
);

substrate_relay_helper::generate_receive_message_delivery_proof_call_builder!(
	BridgeHubWestendMessagesToBridgeHubRococoMessageLane,
	BridgeHubWestendMessagesToBridgeHubRococoMessageLaneReceiveMessagesDeliveryProofCallBuilder,
	relay_bridge_hub_westend_client::runtime::Call::BridgeRococoMessages,
	relay_bridge_hub_westend_client::runtime::BridgeRococoMessagesCall::receive_messages_delivery_proof
);

/// Description of BridgeHubWestend -> BridgeHubRococo messages bridge.
#[derive(Clone, Debug)]
pub struct BridgeHubWestendMessagesToBridgeHubRococoMessageLane;

impl SubstrateMessageLane for BridgeHubWestendMessagesToBridgeHubRococoMessageLane {
	type SourceChain = BridgeHubWestend;
	type TargetChain = BridgeHubRococo;

	type ReceiveMessagesProofCallBuilder =
		BridgeHubWestendMessagesToBridgeHubRococoMessageLaneReceiveMessagesProofCallBuilder;
	type ReceiveMessagesDeliveryProofCallBuilder =
		BridgeHubWestendMessagesToBridgeHubRococoMessageLaneReceiveMessagesDeliveryProofCallBuilder;

	type SourceBatchCallBuilder = UtilityPalletBatchCallBuilder<BridgeHubWestend>;
	type TargetBatchCallBuilder = UtilityPalletBatchCallBuilder<BridgeHubRococo>;
}
