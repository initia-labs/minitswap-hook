module publisher::minitswap_hook {
    use minitia_std::address::to_sdk;
    use minitia_std::cosmos;
    use minitia_std::json;
    use minitia_std::simple_json;
    use minitia_std::string::{Self, String};
    use minitia_std::option;
    use minitia_std::string_utils::to_string;

    use std::signer;

    struct MsgInitiateTokenWithdrawal has drop {
        _type_: String,
        sender: String,
        to: String,
        amount: : Coin
    }

    struct Coin has drop, {
        denom: String,
        amount: u64,
    }

    public entry fun minitswap_hook(
        account: &signer,
        denom: String,
        amount: u64,
        receiver: String,
    ) {
        let obj = simple_json::empty();
        simple_json::set_object(&mut obj, option::none<String>());
        simple_json::increase_depth(&mut obj);
        simple_json::set_string(&mut obj, option::some(string::utf8(b"@type")), string::utf8(b"/opinit.opchild.v1.MsgInitiateTokenWithdrawal"));
        simple_json::set_string(&mut obj, option::some(string::utf8(b"sender")), to_sdk(signer::address_of(account)));
        simple_json::set_string(&mut obj, option::some(string::utf8(b"to")), receiver);
        simple_json::set_object(&mut obj, option::some(string::utf8(b"amount")));
        simple_json::increase_depth(&mut obj);
        simple_json::set_string(&mut obj, option::some(string::utf8(b"denom")), denom);
        simple_json::set_string(&mut obj, option::some(string::utf8(b"amount")), to_string(&amount));

        let req = json::stringify(simple_json::to_json_object(&obj));

        cosmos::stargate(account, req);
    }
}
