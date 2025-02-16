# The Nexium blockchain architecture

## Blockchain storage
The blockchain is stored in a binary file (example : `nxm_blckchn.dat`) wich contains the encoded data of the blocks, from the **genesis block** to the last oen.

We can search through any block in the blockchain by jumping to the next block with the size of the current block.

For verifying the integrity or validity of a transaction, we can verify its existence in a certain block with the help of the Merkle root, by a complexity of O(log n).

## Memory pool (*mempool*)

The memory pool is a temporary storage for all the transactions that are not yet included in the block.

This memory pool is continuously synchronized between all the nodes of the Nexium Network.

When the memory pool is full, all the nodes begin to "mine" the transactions to create the new block. The first node that finds the **nonce** that satisfies the difficulty target will *broadcast* the block to the network. That's how the **proof of work** is done.

## Block structure

- `Block size` **(4 bytes)**
- **Block header** **(78 bytes)**
    - `version` (2 bytes): The version of the block structure.
    - `previous_block_hash` (32 bytes): The hash of the previous block.
    - `merkle_root` (32 bytes): The Merkle root of the transactions in the block.
    - `timestamp` (4 bytes): The timestamp of the block.
    - `bits` (4 bytes): The difficulty target for the block.
    - `nonce` (4 bytes): The nonce for the block.
- **Transaction count** **(2 bytes)**

## Transaction structure
`TODO`

## Network architecture
`TODO`


---




