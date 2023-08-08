// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

import "./IToken.sol";

import "./vibc-core/Ibc.sol";
import "./vibc-core/IbcDispatcher.sol";
import "./vibc-core/IbcReceiver.sol";

contract IbcLendingBorrowing is IbcReceiver, IToken, Ownable{
    // Some string type variables to identify the token.
    string public name = "LendingBorrowing Token";
    string public symbol = "LBTK";

    // The fixed amount of tokens, stored in an unsigned integer type variable.
    uint256 public totalSupply = 1000000000000000000;

    uint256 lbTokenPriceWei = 1000000 gwei;
    uint borrowLimit = 80; // in percentage

    // A mapping is a key/value map. Here we store each account's balance.
    mapping(address => uint256) balances;

    mapping(uint => address) collateral;
    uint collateralId;

    // The Transfer event helps off-chain applications understand
    // what happens within your contract.
    event Transfer(address indexed _from, address indexed _to, uint256 _value);

    // we'll store the ERC20 that have been supplied for lending
    // denominated in the "lbToken", i.e. the auxiliary token for the lending/borrowing contract
    mapping (address => uint) public tokenPrices;
    mapping (uint8 => address) public contractAddresses;

    struct PollPacketData {
        uint8 poll_id;
        uint8 voted;
    }
    
    IbcPacket[] public recvedPackets;
    IbcPacket[] public ackPackets;
    IbcPacket[] public timeoutPackets;
    bytes32[] public connectedChannels;

    string[] supportedVersions = ['1.0', '2.0'];
    bytes32[] supportedVersionsBytes = [bytes32('1.0'), bytes32('2.0')]; // Temp solution, will be removed.

    // at initialization, we'll mint 1e18 of initial supply of the "lbToken"
    constructor(address _plym, address _oibc, address _neb) {
        // The totalSupply is assigned to the transaction sender, which is the
        // account that is deploying the contract.
        balances[address(this)] = totalSupply;

        contractAddresses[1] = _plym;
        contractAddresses[2] = _oibc;
        contractAddresses[3] = _neb;

        // supply:
        // PLYM: 1,000,000
        // OIBC: 200,000
        // NEB: 50,000
        tokenPrices[_oibc] = 500;
        tokenPrices[_plym] = 100;
        tokenPrices[_neb] = 2000;
    }

    function transfer(address to, uint256 amount) external {
        require(balances[msg.sender] >= amount, "Not enough tokens");

        // Transfer the amount.
        balances[msg.sender] -= amount;
        balances[to] += amount;

        // Notify off-chain applications of the transfer.
        // emit Transfer(msg.sender, to, amount);
    }

    function transferFrom(address from, address to, uint256 amount) external {
        require(balances[from] >= amount, "Not enough tokens");

        balances[from] -= amount;
        balances[to] += amount;

        // emit Transfer(from, to, amount);
    }

    
    // Read only function to retrieve the token balance of a given account.
    function balanceOf(address account) external view returns (uint256) {
        return balances[account];
    }

    function supplyFunds(address _token, uint _amount) public {
        // when calling this function, the msg.sender transfers _amount of an ERC20 of their choice
        // the receiver address is this contract's address
        require(tokenPrices[_token]>0);

        // ERC20.approve(address(this), _amount); has to be called first by the sender.
        IToken(_token).transferFrom(msg.sender, address(this), _amount);

        IToken(address(this)).transferFrom(address(this), msg.sender, _amount * tokenPrices[_token]);
    }

    function supplyCollateral() payable public {
        IToken(address(this)).transferFrom(address(this), msg.sender, msg.value/lbTokenPriceWei);
        collateralId++;
        collateral[collateralId] = msg.sender;
    }

    function borrow(address _token, address recipient, uint _amountLB) public {
        // we need to calculate the amount of tokens we can get in exchange for the amount LB tokens
        // this will depend on the borrowLimit
        uint _cAmount = (_amountLB * 80) / (tokenPrices[_token] * 100);

        IToken(address(this)).transferFrom(msg.sender, address(this), _amountLB);
        IToken(_token).transferFrom(address(this), recipient, _cAmount);

    }

    // IBC Packet callbacks

    // TODO
    function onRecvPacket(IbcPacket calldata packet) external returns (AckPacket memory ackPacket) {
        recvedPackets.push(packet);
        //PollPacketData memory _pollPacketData = abi.decode(packet.data, (PollPacketData));

        // hard coding this until we have solution for encoding/decoding scheme
        address choice = contractAddresses[3];
        this.borrow(choice, collateral[1], 25000);

        return AckPacket(true, abi.encodePacked('{ "poll_id": 1, "voted": 3 }'));
    }

    function onAcknowledgementPacket(IbcPacket calldata packet) external {
        ackPackets.push(packet);
    }

    function onTimeoutPacket(IbcPacket calldata packet) external {
        timeoutPackets.push(packet);
    }

    function onOpenIbcChannel(
        string calldata version,
        ChannelOrder ordering,
        string[] calldata connectionHops,
        string calldata counterpartyPortId,
        bytes32 counterpartyChannelId,
        string calldata counterpartyVersion
    ) external returns (string memory selectedVersion) {
        require(bytes(counterpartyPortId).length > 8, 'Invalid counterpartyPortId');
        /**
         * Version selection is determined by if the callback is invoked on behalf of ChanOpenInit or ChanOpenTry.
         * ChanOpenInit: self version should be provided whereas the counterparty version is empty.
         * ChanOpenTry: counterparty version should be provided whereas the self version is empty.
         * In both cases, the selected version should be in the supported versions list.
         */
        bool foundVersion = false;
        selectedVersion = keccak256(abi.encodePacked(version)) == keccak256(abi.encodePacked(''))
            ? counterpartyVersion
            : version;
        for (uint i = 0; i < supportedVersions.length; i++) {
            if (keccak256(abi.encodePacked(selectedVersion)) == keccak256(abi.encodePacked(supportedVersions[i]))) {
                foundVersion = true;
                break;
            }
        }
        require(foundVersion, 'Unsupported version');

        return selectedVersion;
    }

    function onConnectIbcChannel(
        bytes32 channelId,
        bytes32 counterpartyChannelId,
        string calldata counterpartyVersion
    ) external {
        // ensure negotiated version is supported
        bool foundVersion = false;
        for (uint i = 0; i < supportedVersions.length; i++) {
            if (keccak256(abi.encodePacked(counterpartyVersion)) == keccak256(abi.encodePacked(supportedVersions[i]))) {
                foundVersion = true;
                break;
            }
        }
        require(foundVersion, 'Unsupported version');
        connectedChannels.push(channelId);
    }

    function onCloseIbcChannel(
        bytes32 channelId,
        string calldata counterpartyPortId,
        bytes32 counterpartyChannelId
    ) external {
        // logic to determin if the channel should be closed
        bool channelFound = false;
        for (uint i = 0; i < connectedChannels.length; i++) {
            if (connectedChannels[i] == channelId) {
                delete connectedChannels[i];
                channelFound = true;
                break;
            }
        }
        require(channelFound, 'Channel not found');
    }

    /**
     * This func triggers channel closure from the dApp.
     * Func args can be arbitary, as long as dispatcher.closeIbcChannel is invoked propperly.
     */
    function triggerChannelClose(bytes32 channelId, IbcDispatcher dispatcher) external onlyOwner {
        dispatcher.closeIbcChannel(channelId);
    }

    function sendMessage(
        IbcDispatcher dispatcher,
        string calldata message,
        bytes32 channelId,
        uint64 timeoutTimestamp,
        PacketFee calldata fee
    ) external payable {
        uint256 maxFee = fee.ackFee > fee.timeoutFee ? fee.ackFee : fee.timeoutFee;
        dispatcher.sendPacket{value: fee.recvFee + maxFee}(channelId, bytes(message), timeoutTimestamp, fee);
    }

}
