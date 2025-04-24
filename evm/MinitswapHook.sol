// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.25;
import "./Strings.sol";
import "./interface/ICosmos.sol";
import "./interface/IERC20.sol";

contract MinitswapHook {
    using Strings for uint256;

    function minitswapHook(
        string memory denom,
        uint amount,
        string memory receiver
    ) public {
        IERC20(COSMOS_CONTRACT.to_erc20(denom)).transferFrom(
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
                '","amount":{ "denom":"',
                denom,
                '","amount":"',
                amount.toString(),
                '"}}'
            )
        );
        COSMOS_CONTRACT.execute_cosmos(message, 300000);
    }
}
