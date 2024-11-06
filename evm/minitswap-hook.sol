// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.24;

import "./i_cosmos/ICosmos.sol";

contract MinitSwapHook {
    function minitswap_hook(
        string memory denom,
        uint amount,
        string memory receiver
    ) public {
        string memory message = string(
            abi.encodePacked(
                '{"@type": "/opinit.opchild.v1.MsgInitiateTokenWithdrawal",',
                '"sender": "',
                COSMOS_CONTRACT.to_cosmos_address(msg.sender),
                '","to":"',
                receiver,
                '","amount": [ { "denom":"',
                denom,
                '","amount":"',
                amount,
                "}]}"
            )
        );
        COSMOS_CONTRACT.execute_cosmos(message);
    }
}
