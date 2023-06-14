# How to use the RSA program

## Installing Rust

If you don't have Rust installed, you can follow the steps [here](https://www.rust-lang.org/tools/install).

## Running and using the program

Once you have Rust installed, clone the repository by running ```git clone https://github.com/asvorg/tiralabra```. Once you have the repository cloned, navigate to the rsa subfolder with your selected terminal. The program can be run with ```cargo run```.

Once the program runs you are greeted with this pretty self explanatory message:


```
Welcome to the RSA encryption and decryption program!
Please select the mode you would like to use:
1. Encrypt a message
2. Decrypt a message
3. Generate keys
4. Demo mode, generate relevant prime numbers and keys, encrypt and decrypt a message
5. Demo mode with user supplied keys
6. Exit
7. This message
Please enter the number of the mode you would like to use:
```

These options function as follows:

- Mode 1: Encrypt a message - This option allows you to encrypt a message using RSA encryption.
- Mode 2: Decrypt a message - This option allows you to decrypt a message using RSA decryption.
- Mode 3: Generate keys - Selecting this mode generates a new pair of RSA public and private keys.
- Mode 4: Demo mode - This mode demonstrates the complete process by generating relevant prime numbers, generating keys, encrypting a message, and decrypting it.
- Mode 5: Demo mode with user supplied keys - This mode allows you to enter your own RSA public and private keys for encryption and decryption.
- Mode 6: Exit - Choosing this mode will exit the program.
- Mode 7: This message - This option displays the menu again with the available modes.

REMEMBER THAT MESSAGE BITS HAS TO BE LESS THAN n! Message length check is not implemented.

Enter the number corresponding to your desired mode and press Enter. Follow the prompts and instructions displayed on the screen based on the selected mode. The program will guide you through the encryption and decryption process or key generation based on your chosen mode.Repeat the process as needed for encryption, decryption, or key generation with different messages or keys. If you want to exit the program, select Mode 6 (Exit) from the menu. If you need a reminder of the available modes, select Mode 7 to display the menu again.