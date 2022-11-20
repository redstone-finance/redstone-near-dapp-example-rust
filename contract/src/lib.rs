use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use redstone_near_connector_rs::decode_hex;

const BTC_BYTES_32_HEX_STR: &str =
    "4254430000000000000000000000000000000000000000000000000000000000";

const REDSTONE_MAIN_DEMO_SIGNER_PUB_KEY_HEX: &str =
  "009dd87eb41d96ce8ad94aa22ea8b0ba4ac20c45e42f71726d6b180f93c3f298e333ae7591fe1c9d88234575639be9e81e35ba2fe5ad2c2260f07db49ccb9d0d";

fn get_pub_key(hex_pub_key: &str) -> [u8; 64] {
    let pub_key_vec = decode_hex(hex_pub_key).unwrap();
    pub_key_vec.try_into().unwrap()
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct RedstoneExample {
    val: i8,
}

#[near_bindgen]
impl RedstoneExample {
    // Public read-only method: Returns the extracted and verified oracle value
    pub fn get_oracle_value(&self, redstone_payload: String) -> u128 {
        let data_feed_id_vec = decode_hex(BTC_BYTES_32_HEX_STR).unwrap();
        let data_feed_id: [u8; 32] = data_feed_id_vec.try_into().unwrap();

        let redstone_payload_bytes = decode_hex(&redstone_payload).unwrap();
        let authorised_signers: Vec<[u8; 64]> =
            vec![get_pub_key(REDSTONE_MAIN_DEMO_SIGNER_PUB_KEY_HEX)];
        let unique_signers_threshold = 1;

        let current_timestamp_milliseconds =
            u128::from(near_sdk::env::block_timestamp() / 1_000_000);

        let oracle_value = redstone_near_connector_rs::get_oracle_value(
            &data_feed_id,
            unique_signers_threshold,
            &authorised_signers,
            current_timestamp_milliseconds,
            &redstone_payload_bytes,
        );

        oracle_value
    }
}
