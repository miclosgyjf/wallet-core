// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

uniffi::include_scaffolding!("wallet_modules");

pub fn tw_example_module_request(coin: u32) -> String {
    let response = reqwest::blocking::get("https://google.com").unwrap();
    let payload = response.text().unwrap();

    let res = unsafe {
        wallet_core_rs::ffi::ethereum::abi::tw_ethereum_abi_decode_contract_call(
            coin,
            std::ptr::null(),
        )
    };
    assert!(res.is_null());
    payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tw_wallet_module_request() {
        let payload = unsafe { tw_example_module_request(1) };
        assert!(!payload.is_empty());
        println!("{payload}");
    }
}
