# Structure des blocs

## Bloc (*variable*)
- `bloc_header` (*82 octets*)
    - En-tête du bloc.
- `transactions` (*de 330 à (2048\*`transaction_count`)*)
    - Liste des transactions, concaténées.

### `bloc_header` (*82 octets*)
- `version`(*2 octets*)
    - Version du protocole adoptée (**ex: `1`**).
- `previous_block_hash` (*32 octets*)
    - Hash (= **identifiant unique**) du bloc précédant.
- `merkle_root` (*32 octets*)
    - Hash racine de l'**arbre de Merkle**.
- `timestamp` (*4 octets*)
    - **Unix Timestamp** (secondes depuis le 01/01/1970) de la création du bloc.
- `difficulty_target` (*4 octets*)
    - Nombres de zéros nécessaires au début du hash pour que le **nonce** soit valide.
- `nonce` (*4 octets*)
    - Valeur trouvée par le mineur qui remplit les conditions de **difficulty_target**.
- `transactions_size` (*4 octets*)
    - Taille totale des transactions.

---

## `transaction` (*de 330 à 2048 octets*)
- `transaction_header` (*73 octets*)
    - En-tête de la transaction.
- `data` (*de 1 à 1719 octets*)
    - Données de la transaction, suivant ou non le format consensuel lié à la valeur de `data_type`.
- `signature` (*256 octets*)
    - Signature de la transaction par l'emetteur. Il s'agit de **la signature de la concaténation de `transaction_header` et `data`**.

### `transaction_header` (*73 octets*)
- `transaction_size` (*2 octets*)
    - Taille de la transaction, en octets.
- `timestamp` (*4 octets*)
    - **Unix Timestamp** (*secondes depuis le 01/01/1970*) de la création de la transaction.
- `fees` (*2 octets*)
    - **Frais de transaction**: nombre de `µNEX` par octet écrit (*1 micro-NEX* = `0,000001 NEX`) <=> `µNEX/Byte`.
- `emitter` (*64 octets*)
    - **login** de l'émetteur de la transaction.
- `data_type` (*1 octet*)
    - Type d'information contenue dans le data de la transaction. Détermine comment le noeud validateur doit traiter la transaction (*transfert de crédits, contrats intelligents...*).

    
### `data_type` (*1 octet*)
- `00000001` : Transaction classique.
    - Transfert de **NEX** classique de l'emetteur vers le récepteur.
- `xxxxxxxx` : Autres types de transaction
    - Pourront être définies par les prochaines versions du protocole. La transaction est incluse dans la blockchain même si le noeud validateur ne reconnait pas la valeur du `data_type`.

### `data` (*de 1 à 1719 octets*)
- **Pour les transactions classiques** (*69 ou 325 octets*):
    - `receiver` (*64 octets*)
        - **login** du destinataire de la transaction.
    - `amount` (*4 octets*)
        - **montant** de la transaction, en `NEX`.
    - `has_description` (*1 octet*)
        - `00000001` s'il y a une description
        - `00000000` sinon.
    - `description` (facultative) (*256 octets*)
        - Descriptif de la transaction chiffré avec la clé publique du destinataire. Le destinataire est donc le seul à pouvoir lire le descriptif de la transaction. L'existence de ce champ dépend de la valeur de `has_description`.

---

La limite de taille de **1719** octets du `data` d'une transaction est établie de telle sorte à ce que la taille totale de la transaction (`data`+`transaction_header`+`signature`) n'excède pas **2048 octets**.

## Tailles théoriques de blocs

Les tailles pour un **bloc de n transactions** seraient donc :
- *transactions sans description* : `82 + n*(73+256+69)` octets.
- *transactions avec description* : `82 + n*(73+256+325)` octets.

Ces tailles de blocs partent du principe que les transactions contenues par ledit bloc sont toutes **avec** ou **sans** description. 

Cette approche n'est pas réaliste mais permet d'encadrer la taille d'un bloc de **n** transactions.

Par exemple, **pour un bloc de 1000 transactions** :
- *transactions sans description* : `82 + 1000*(73+256+69)` = **398082** octets.
- *transactions avec description* : `82 + 1000*(73+256+325)` = **654082** octets.

**Donc,** la taille d'un bloc contenant 1000 transactions est comprise **entre environ 398 Ko et 654 Ko**. 

En supposant la taille du `bloc_header` (**80 octets**) négligeable devant la taille des transactions, on peut considérer la taille d'un bloc *proportionnelle* à son nombre de transactions : `10 fois moins de transactions <=> bloc 10 fois moins gros`.
