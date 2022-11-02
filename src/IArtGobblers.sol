pragma solidity ^0.8.0;

interface ArtGobblers {
    // GobblersERC721

    event Transfer(address indexed from, address indexed to, uint256 indexed id);

    event Approval(address indexed owner, address indexed spender, uint256 indexed id);

    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);

    // ArtGobblers

    event GooBalanceUpdated(address indexed user, uint256 newGooBalance);

    event GobblerClaimed(address indexed user, uint256 indexed gobblerId);
    event GobblerPurchased(address indexed user, uint256 indexed gobblerId, uint256 price);
    event LegendaryGobblerMinted(address indexed user, uint256 indexed gobblerId, uint256[] burnedGobblerIds);
    event ReservedGobblersMinted(address indexed user, uint256 lastMintedGobblerId, uint256 numGobblersEach);

    event RandomnessFulfilled(uint256 randomness);
    event RandomnessRequested(address indexed user, uint256 toBeRevealed);
    event RandProviderUpgraded(address indexed user, address indexed newRandProvider);

    event GobblersRevealed(address indexed user, uint256 numGobblers, uint256 lastRevealedId);

    event ArtGobbled(address indexed user, uint256 indexed gobblerId, address indexed nft, uint256 id);
}
