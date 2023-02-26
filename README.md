## Documentation pour le projet Rust

Ce projet est constitué de trois fichiers : client.rs, connexion.rs et md5hashcash.rs. Le fichier client.rs contient le code principal pour le client qui se connecte au serveur, tandis que connexion.rs contient des fonctions pour la sérialisation et la désérialisation des messages échangés entre le client et le serveur. Le fichier md5hashcash.rs contient les structures et les fonctions nécessaires pour résoudre le challenge MD5 Hashcash.

# client.rs
Le fichier client.rs contient la fonction principale main, qui se connecte au serveur et joue 15 rounds du jeu. Il utilise également les fonctions inscription et play_rounds pour s'inscrire au serveur et jouer un round, respectivement.

Fonction inscription
La fonction inscription envoie un message "Hello" au serveur, attend la réponse "Welcome", envoie un message "Subscribe" pour s'abonner en tant que joueur, puis attend la réponse "SubscribeResult".

Fonction play_rounds
La fonction play_rounds lit un message du serveur, analyse le message pour déterminer le type de challenge à résoudre, résout le challenge MD5 Hashcash et envoie la réponse au serveur. Elle utilise également les messages "PublicLeaderBoard" pour trouver le joueur suivant à défier.

# connexion.rs
Le fichier connexion.rs contient deux fonctions : serialize_and_send_message et read_message. Ces fonctions sont utilisées pour sérialiser et désérialiser les messages échangés entre le client et le serveur.

Fonction serialize_and_send_message
La fonction serialize_and_send_message sérialise un message en JSON, calcule la longueur du message et envoie le message sérialisé et sa longueur au serveur.

Fonction read_message
La fonction read_message lit la longueur d'un message envoyé par le serveur, lit le message lui-même et le renvoie sous forme de chaîne de caractères.

# md5hashcash.rs
Le fichier md5hashcash.rs contient les structures et les fonctions nécessaires pour résoudre le challenge MD5 Hashcash.

Structure MD5HashCashInput
La structure MD5HashCashInput contient les informations nécessaires pour résoudre le challenge MD5 Hashcash : la complexité du challenge et le message à hasher.

Structure MD5HashCashOutput
La structure MD5HashCashOutput contient les informations renvoyées par la résolution du challenge MD5 Hashcash : le seed utilisé pour calculer le hash et le hashcode obtenu.

Structure MD5HashCash
La structure MD5HashCash implémente le trait Challengee pour résoudre le challenge MD5 Hashcash. Elle contient une structure MD5HashCashInput et implémente les fonctions new, solve et verify.

Fonction check_hash
La fonction check_hash vérifie si un hash satisfait la complexité du challenge MD5 Hashcash. Elle prend en entrée la complexité et le hash à vérifier et renvoie un booléen.

## Conclusion
