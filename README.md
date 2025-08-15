# CrackaCesar
rusty cesar cipher cracker

# USAGE:
`CrackaCesar cipher text words here.`

# Example:
`CrackaCesar Jr ubcr lbhe oht obhagl pnerre vf shyy bs ohtf naq tbyq`

# How it works:
It keeps a hashmap where every letter of the alphabet are the values, and their index is the key.

It then takes your cipher and breaks out the small words > 4 characters and decrypts them with every key between 0 and 25. I then checks those words against a list of commonly used short words in the english language. If every word is valid then it decrypts the rest of the cipher with that key, if more then half the words are valid it will prompt you to attempt to decrypt it with that key, if less then half the words are valid it will print out every word and every decryption with the attempted keys, and prompt you to choose the most correct one.
