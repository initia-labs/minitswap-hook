// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.24;

import "./interface/ICosmos.sol";
import "./interface/IERC20.sol";

contract MinitswapHook {
    function minitswapHook(
        string memory denom,
        uint amount,
        string memory receiver
    ) public {
        IERC20(COSMOS_CONTRACT.to_evm_address(denom)).transferFrom(
            msg.sender,
            address(this),
            amount
        );

        string memory message = string(
            abi.encodePacked(
                '{"@type": "/opinit.opchild.v1.MsgInitiateTokenWithdrawal",',
                '"sender": "',
                COSMOS_CONTRACT.to_cosmos_address(address(this)),
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
