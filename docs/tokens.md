# Authentication System Documentation

## Overview

This document outlines the token-based authentication system used in web-agent-rs. The system uses FIPS204 signatures to
generate secure session tokens containing user data.

## Core Components

TODO: In the meantime, here's some hamfisted knowledge. 


```javascript
class FIPS204KeyPair {
    constructor() {
        this.publicKey = "FIPS204_PUBLIC_KEY"; // Placeholder
        this.privateKey = "FIPS204_PRIVATE_KEY"; // Placeholder
    }

    sign(data) {
        // Placeholder for actual FIPS204 signing logic
        return `FIPS204_${data}_SIGNED`;
    }

    verify(data, signature) {
        // Placeholder for actual FIPS204 verification
        return true;
    }
}

/* NOTES:
- the public key needs to be retrievable, so it can be used to verify payload signature at the time of the request.
- the private key is disposed so it can't be used to create more signatures
- future tokens should use a completely new keypair


- The fips204 authentication scheme was selected for its performance, flexibility, and key-length.
- It would be wise to configure additional protections like ip whitelisting and rate limiting.   
*/

// User object representing token payload data
const user = {
    sub: "user123",
    name: "John Doe",
    email: "john@example.com",
    roles: ["user"],
    iat: Math.floor(Date.now() / 1000),
    exp: Math.floor(Date.now() / 1000) + (60 * 60) // 1 hour from now
};

const keyPair = new FIPS204KeyPair();
const signature = keyPair.sign(JSON.stringify(user));

function createToken(payload, signature) {
    const encodedPayload = Buffer.from(JSON.stringify(payload)).toString('base64');
    const encodedSignature = Buffer.from(signature).toString('base64');
    return `${encodedPayload}.${encodedSignature}`;
}

const token = createToken(user, signature);


async function createStreamRequest(eventHost = "https://agent.example.com") {
    
    const requestParams = {
        // will automagically include the session token as a cookie, where it will be parsed by the agent server
        credentials: "include"
    }

    const response = await fetch(eventHost, requestParams);
    
    const {streamId} = await response.json();
    
    // This stream id is then supplied as a path parameter to stream, the token is validated to ensure the stream belongs to the user, and the stream is returned.
    return streamId;
}
```