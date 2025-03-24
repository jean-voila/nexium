# Structure des blocs

## Bloc (*variable*)
- `bloc_size` (*2 octets*)
- `bloc_header` (*80 octets*)
- `transactions` (*variable*)

## `bloc_header` (*80 octets*)
- `version`(*2 octets*)
- `previous_block_hash` (*32 octets*)
- `merkle_root` (*32 octets*)
- `timestamp` (*4 octets*)
- `difficulty_target` (*4 octets*)
- `nonce` (*4 octets*)
- `transaction_count` (*2 octets*)

## `transaction` (*variable*)
- `transaction_size` (*2 octets*)
- `timestamp` (*4 octets*)
- `fees` (*2 octets*)
- `emitter` (*64 octets*)
- `transaction_type` (*1 octet*)
- `data` (*variable*)


### `transaction_type`
- `00000001` : Transaction classique
- `xxxxxxxx` : Autres types de transaction, à déterminer plus tard.

### `data`
- **Pour les transactions classiques** : 
    - `receiver` (*64 octets*)
    - `amount` (*4 octets*)
    - `description` (facultative)