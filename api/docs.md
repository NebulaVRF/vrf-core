# NebulaVRF API Documentation

> **Deployed API Base URL:** [https://vrf-core-production.up.railway.app](https://vrf-core-production.up.railway.app)
>
> You can test all endpoints live at this URL.

This document describes the HTTP API endpoints for the NebulaVRF randomness and proof service. All endpoints are available under the base URL.

---

## Endpoints

### 1. `GET /get-random`

**Description:**
Generates a new random seed, VRF output, and optionally returns the public key and commitment. You may supply your own seed (as a 32-byte hex string) or let the server generate one for you.

**Query Parameters:**
- `seed` (optional, hex string): If provided and valid (32 bytes), this seed will be used. Otherwise, a random seed is generated.
- `proof` (optional, bool): If true, includes the public key in the response.
- `commit` (optional, bool): If true, includes the commitment in the response.

**Examples:**
- Generate with a random seed:
  ```sh
  curl "http://localhost:3000/get-random?proof=true&commit=true"
  ```
- Generate with your own seed:
  ```sh
  curl "http://localhost:3000/get-random?seed=0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef&proof=true&commit=true"
  ```

**Response:**
```json
{
  "seed": "<hex-encoded 32-byte seed>",
  "randomness": "<hex-encoded VRF output>",
  "public_key": "<hex-encoded public key, optional>",
  "commitment": "<hex-encoded commitment, optional>"
}
```
- `seed`: The random seed used for VRF generation (hex string).
- `randomness`: The VRF output (hex string, 48 bytes).
- `public_key`: The BLS public key used for verification (hex string, 96 bytes, present if `proof=true`).
- `commitment`: The SHA256 commitment to the seed (hex string, present if `commit=true`).

---

### 2. `GET /payloads`

**Description:**
Generates a Soroban testnet payload bundle (seed, salt, commitment, pubkey, signature)
in both **hex** and **base64**. This is intended for testnet integration and Stellar Lab
input formatting.

**Query Parameters:**
- `seed_len` (optional, integer): Length of the random seed in bytes (default: 8).
- `salt_len` (optional, integer): Length of the random salt in bytes (default: 8).

**Example:**
```sh
curl "http://localhost:3000/payloads?seed_len=8&salt_len=8"
```

**Response:**
```json
{
  "hex": {
    "seed": "<hex>",
    "salt": "<hex>",
    "commitment": "<hex>",
    "pubkey": "<hex>",
    "signature": "<hex>"
  },
  "base64": {
    "seed": "<base64>",
    "salt": "<base64>",
    "commitment": "<base64>",
    "pubkey": "<base64>",
    "signature": "<base64>"
  }
}
```

Notes:
- `commitment = sha256(seed || salt)`
- `pubkey` is G1 (96 bytes, uncompressed)
- `signature` is G2 (192 bytes, uncompressed)

---

### 3. `POST /verify-random`

**Description:**
Verifies a VRF proof given a seed, output, and public key. Returns whether the proof is valid.

**Request Body (JSON):**
```json
{
  "seed": "<hex-encoded seed>",
  "output": "<hex-encoded VRF output>",
  "public_key": "<hex-encoded public key>"
}
```

**Example:**
```sh
curl -X POST "http://localhost:3000/verify-random" \
  -H "Content-Type: application/json" \
  -d '{
    "seed": "...",
    "output": "...",
    "public_key": "..."
  }'
```

**Response:**
```json
{ "valid": true }
```
- `valid`: Boolean indicating if the proof is valid for the given seed and public key.

---

### 4. `POST /commit`

**Description:**
Returns a SHA256 commitment for a given seed.

**Request Body (JSON):**
```json
{
  "seed": "<hex-encoded seed>"
}
```

**Example:**
```sh
curl -X POST "http://localhost:3000/commit" \
  -H "Content-Type: application/json" \
  -d '{
    "seed": "..."
  }'
```

**Response:**
```json
{ "commitment": "<hex-encoded commitment>" }
```
- `commitment`: The SHA256 hash of the seed (hex string).

---

### 5. `POST /verify-commit`

**Description:**
Verifies that a given seed matches a provided commitment.

**Request Body (JSON):**
```json
{
  "seed": "<hex-encoded seed>",
  "commitment": "<hex-encoded commitment>"
}
```

**Example:**
```sh
curl -X POST "http://localhost:3000/verify-commit" \
  -H "Content-Type: application/json" \
  -d '{
    "seed": "...",
    "commitment": "..."
  }'
```

**Response:**
```json
{ "valid": true }
```
- `valid`: Boolean indicating if the seed matches the commitment.

---

## Field Explanations
- **Hex-encoded fields:** All binary data (seed, randomness, public key, commitment) is encoded as a lowercase hexadecimal string for safe transport in JSON.
- **randomness:** The output of the VRF, which is cryptographically secure and can be used as a source of randomness in your application.
- **public_key:** The BLS public key used to verify the VRF output.
- **commitment:** A SHA256 hash of the seed, used for commit-reveal schemes to prevent bias.

## Usage Notes
- Always use cryptographically secure random seeds for best results.
- The `/get-random` endpoint is useful for generating new randomness and commitments.
- Use `/verify-random` to check the validity of a VRF proof (e.g., in a smart contract or backend service).
- Use `/commit` and `/verify-commit` for commit-reveal flows to ensure fairness and prevent manipulation.
- Use `/payloads` when you need Soroban-compatible testnet payloads for `commit` and `reveal`.

## Error Handling
- If a request is malformed or contains invalid hex, the endpoint will return `valid: false` or an appropriate error response.
- Always check the `valid` field in verification responses.

---

## Example Workflow
1. Call `/get-random` to generate a seed, randomness, public key, and commitment (optionally supply your own seed).
2. Call `/payloads` to generate testnet-ready payloads for the Soroban contract.
3. Use `/verify-random` to verify the randomness and proof.
4. Use `/commit` to get a commitment for a custom seed.
5. Use `/verify-commit` to verify a seed against a commitment.

---

For further questions or integration help, see the main project README or contact the maintainers. 