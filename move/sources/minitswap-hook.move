module publisher::minitswap_hook {
    use minitia_std::address::to_sdk;
    use minitia_std::cosmos;
    use minitia_std::json::marshal;
    use minitia_std::string::{Self, String};

    use std::signer;

    struct MsgInitiateTokenWithdrawal has drop {
        _type_: String,
        sender: String,
        to: String,
        amount: Coin
    }

    struct Coin has drop {
        denom: String,
        amount: u64,
    }

    public entry fun minitswap_hook(
        account: &signer,
        denom: String,
        amount: u64,
        receiver: String,
    ) {
        let msg = MsgInitiateTokenWithdrawal {
            _type_: string::utf8(b"/opinit.opchild.v1.MsgInitiateTokenWithdrawal"),
            sender: to_sdk(signer::address_of(account)),
            to: receiver,
            amount: Coin { denom, amount }
        };
        cosmos::stargate(account, marshal(&msg))
    }
}
