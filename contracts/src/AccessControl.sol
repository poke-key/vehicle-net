// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "./DataMarketplace.sol";

contract AccessControl is ReentrancyGuard, Ownable {
    DataMarketplace public immutable dataMarketplace;
    VehicleRegistry public immutable vehicleRegistry;

    struct AccessSession {
        address user;
        uint256 productId;
        uint256 startTime;
        uint256 endTime;
        bool isActive;
        uint256 dataRequests;
        bytes32 sessionKey;
    }

    struct DataRequest {
        address user;
        uint256 productId;
        uint256 timestamp;
        string requestType;
        bool wasAuthorized;
    }

    mapping(bytes32 => AccessSession) public accessSessions;
    mapping(address => bytes32[]) public userSessions;
    mapping(uint256 => bytes32[]) public productSessions;
    mapping(bytes32 => DataRequest[]) public sessionRequests;
    
    uint256 public maxConcurrentSessions = 3;
    uint256 public sessionTimeoutBuffer = 300; // 5 minutes buffer
    uint256 private sessionNonce = 0;
    
    event AccessSessionCreated(
        bytes32 indexed sessionKey,
        address indexed user,
        uint256 indexed productId,
        uint256 endTime
    );
    
    event AccessSessionTerminated(bytes32 indexed sessionKey, string reason);
    
    event DataAccessRequested(
        bytes32 indexed sessionKey,
        address indexed user,
        uint256 indexed productId,
        string requestType,
        bool authorized
    );
    
    event SessionLimitUpdated(uint256 newLimit);

    error SessionNotFound();
    error SessionExpired();
    error SessionNotActive();
    error UnauthorizedAccess();
    error TooManyConcurrentSessions();
    error InvalidSessionKey();

    modifier validSession(bytes32 sessionKey) {
        if (accessSessions[sessionKey].user == address(0)) {
            revert SessionNotFound();
        }
        _;
    }

    modifier activeSession(bytes32 sessionKey) {
        AccessSession memory session = accessSessions[sessionKey];
        if (!session.isActive) {
            revert SessionNotActive();
        }
        if (block.timestamp > session.endTime + sessionTimeoutBuffer) {
            revert SessionExpired();
        }
        _;
    }

    constructor(address _dataMarketplace, address _vehicleRegistry) Ownable(msg.sender) {
        dataMarketplace = DataMarketplace(_dataMarketplace);
        vehicleRegistry = VehicleRegistry(_vehicleRegistry);
    }

    function createAccessSession(
        uint256 productId,
        address user
    ) external returns (bytes32 sessionKey) {
        (bool hasAccess, uint256 accessEndTime) = dataMarketplace.hasValidAccess(user, productId);
        
        if (!hasAccess) {
            revert UnauthorizedAccess();
        }

        if (getUserActiveSessions(user).length >= maxConcurrentSessions) {
            revert TooManyConcurrentSessions();
        }

        sessionNonce++;
        sessionKey = keccak256(
            abi.encodePacked(
                user,
                productId,
                block.timestamp,
                block.prevrandao,
                msg.sender,
                sessionNonce
            )
        );

        accessSessions[sessionKey] = AccessSession({
            user: user,
            productId: productId,
            startTime: block.timestamp,
            endTime: accessEndTime,
            isActive: true,
            dataRequests: 0,
            sessionKey: sessionKey
        });

        userSessions[user].push(sessionKey);
        productSessions[productId].push(sessionKey);

        emit AccessSessionCreated(sessionKey, user, productId, accessEndTime);
        
        return sessionKey;
    }

    function requestDataAccess(
        bytes32 sessionKey,
        string memory requestType
    ) external validSession(sessionKey) activeSession(sessionKey) returns (bool authorized) {
        AccessSession storage session = accessSessions[sessionKey];
        
        if (session.user != msg.sender) {
            revert UnauthorizedAccess();
        }

        authorized = block.timestamp <= session.endTime;
        
        if (authorized) {
            session.dataRequests++;
        }

        DataRequest memory request = DataRequest({
            user: msg.sender,
            productId: session.productId,
            timestamp: block.timestamp,
            requestType: requestType,
            wasAuthorized: authorized
        });

        sessionRequests[sessionKey].push(request);

        emit DataAccessRequested(sessionKey, msg.sender, session.productId, requestType, authorized);
        
        return authorized;
    }

    function terminateSession(bytes32 sessionKey, string memory reason) 
        external 
        validSession(sessionKey) 
    {
        AccessSession storage session = accessSessions[sessionKey];
        
        if (session.user != msg.sender && msg.sender != owner()) {
            revert UnauthorizedAccess();
        }

        session.isActive = false;
        
        emit AccessSessionTerminated(sessionKey, reason);
    }

    function extendSession(bytes32 sessionKey) 
        external 
        validSession(sessionKey) 
        activeSession(sessionKey) 
    {
        AccessSession storage session = accessSessions[sessionKey];
        
        if (session.user != msg.sender) {
            revert UnauthorizedAccess();
        }

        (bool hasAccess, uint256 newEndTime) = dataMarketplace.hasValidAccess(msg.sender, session.productId);
        
        if (!hasAccess) {
            revert UnauthorizedAccess();
        }

        session.endTime = newEndTime;
    }

    function validateAccess(bytes32 sessionKey, address user) 
        external 
        view 
        returns (bool isValid, uint256 timeRemaining) 
    {
        AccessSession memory session = accessSessions[sessionKey];
        
        if (session.user != user || !session.isActive) {
            return (false, 0);
        }

        if (block.timestamp > session.endTime) {
            return (false, 0);
        }

        return (true, session.endTime - block.timestamp);
    }

    function getAccessSession(bytes32 sessionKey) 
        external 
        view 
        validSession(sessionKey) 
        returns (AccessSession memory) 
    {
        return accessSessions[sessionKey];
    }

    function getUserSessions(address user) 
        external 
        view 
        returns (bytes32[] memory) 
    {
        return userSessions[user];
    }

    function getUserActiveSessions(address user) 
        public 
        view 
        returns (bytes32[] memory activeSessions) 
    {
        bytes32[] memory allSessions = userSessions[user];
        uint256 activeCount = 0;

        for (uint256 i = 0; i < allSessions.length; i++) {
            AccessSession memory session = accessSessions[allSessions[i]];
            if (session.isActive && block.timestamp <= session.endTime) {
                activeCount++;
            }
        }

        activeSessions = new bytes32[](activeCount);
        uint256 index = 0;

        for (uint256 i = 0; i < allSessions.length; i++) {
            AccessSession memory session = accessSessions[allSessions[i]];
            if (session.isActive && block.timestamp <= session.endTime) {
                activeSessions[index] = allSessions[i];
                index++;
            }
        }

        return activeSessions;
    }

    function getProductSessions(uint256 productId) 
        external 
        view 
        returns (bytes32[] memory) 
    {
        return productSessions[productId];
    }

    function getSessionRequests(bytes32 sessionKey) 
        external 
        view 
        returns (DataRequest[] memory) 
    {
        return sessionRequests[sessionKey];
    }

    function setMaxConcurrentSessions(uint256 newLimit) external onlyOwner {
        maxConcurrentSessions = newLimit;
        emit SessionLimitUpdated(newLimit);
    }

    function setSessionTimeoutBuffer(uint256 newBuffer) external onlyOwner {
        sessionTimeoutBuffer = newBuffer;
    }

    function cleanupExpiredSessions(bytes32[] memory sessionKeys) external {
        for (uint256 i = 0; i < sessionKeys.length; i++) {
            AccessSession storage session = accessSessions[sessionKeys[i]];
            if (session.isActive && block.timestamp > session.endTime + sessionTimeoutBuffer) {
                session.isActive = false;
                emit AccessSessionTerminated(sessionKeys[i], "Expired");
            }
        }
    }
}