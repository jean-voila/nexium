//String (infos du bloc d'avant qu'on veut hash (possible d'avoir un message long))
//-> Bin (allouer de la place pour la taille du message etttttttt multiple 512 bits)
//-> manipulations farfelues:
//-On va faire 8 constantes sur lesquelles on fait des operations tout le long du traitement du tableau
//-Si plusieurs blocs, on garde les memes constantes sur la continuité lorsqu'on change de bloc

//-> 8 x 8 caractères en hexa qu'on concatène pour obtenir le code SHA-256 soit 64 caracteres hexa
