// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

fn main() {
    uniffi::generate_scaffolding("./src/wallet_modules.udl").unwrap();
}
