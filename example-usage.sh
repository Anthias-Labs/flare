#!/bin/bash

BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

mnemonic=$(cat ./.mnemonic)

if [ -z "$mnemonic" ]
then
    echo "Please provide a devnet seedphrase in the .mnemonic file, and use a faucet to provide it with some balance."
    exit 1
fi

if [ ! -f "./flare" ]
then
    echo "Building binary and copying to root..."
    cargo build && cp ./target/debug/flare .
fi

echo
echo "-------------------------------------"
echo -e "$BLUE Recovering keypair file and address... $NC"
echo -e "$RED ./flare wallet-recover --mnemonic \"$(echo $mnemonic)\" $NC"
./flare wallet-recover --mnemonic "$(echo $mnemonic)"

output=$(./flare wallet-recover --mnemonic "$(echo $mnemonic)")
address=$(echo "$output" | grep -oP 'Address: \K\w+')
keypair="$address.json"

echo
echo "-------------------------------------"
echo -e "$BLUE Checking wallet balance... $NC"
echo -e "$RED ./flare --cluster devnet balance $(echo $address) $NC"
./flare --cluster devnet balance $(echo $address)

echo
echo "-------------------------------------"
echo -e "$BLUE Recovering address from keypair file... $NC"
echo -e "$RED ./flare address-derive --keypair $(echo $keypair) $NC"
./flare address-derive --keypair $(echo $keypair)

echo
echo "-------------------------------------"
echo -e "$BLUE Signing arbitrary message... $NC"
echo -e "$RED ./flare sign --keypair $(echo $keypair) \"Hello Solana!\" $NC"
./flare sign --keypair $(echo $keypair) "Hello Solana!"

echo
echo "-------------------------------------"
echo -e "$BLUE Creating a new wallet... $NC"
echo -e "$RED ./flare wallet-create $NC"
./flare wallet-create

echo
echo "-------------------------------------"
echo -e "$BLUE Checking current epoch number... $NC"
echo "./flare --cluster devnet epoch"
./flare --cluster devnet epoch

echo
echo "-------------------------------------"
echo -e "$BLUE Checking current block-height number... $NC"
echo -e "$RED ./flare --cluster devnet block-height $NC"
./flare --cluster devnet block-height

echo
echo "-------------------------------------"
echo -e "$BLUE Sending SOL to another account... $NC"
echo -e "$RED ./flare --cluster devnet send --keypair $(echo $keypair) --to 67pUbVFbg4Q94id7NgLFiC5GdtWKgmKYgNXp2CtwsWty 100 $NC"
./flare --cluster devnet send --keypair $(echo $keypair) --to 67pUbVFbg4Q94id7NgLFiC5GdtWKgmKYgNXp2CtwsWty 100

echo
echo "-------------------------------------"
echo -e "$BLUE Calling a method from a program... $NC"
echo -e "$RED ./flare --cluster devnet call --keypair $(echo $keypair) --program WixFUMVqBSTygzeFy9Wuy5XxkeH8xHnUEGvfyyJYqve --accounts 78vJRdkATNZm7cJHaLscYu1HZq24EH3FV6Eppx3BS9qA,$(echo $address) --signers $(echo $keypair) --idl ./example/onchain_voting.json  gibVote GM $NC"
./flare call --keypair $(echo $keypair) --program WixFUMVqBSTygzeFy9Wuy5XxkeH8xHnUEGvfyyJYqve --accounts 78vJRdkATNZm7cJHaLscYu1HZq24EH3FV6Eppx3BS9qA,$(echo $address) --signers $(echo $keypair) --idl ./example/onchain_voting.json  gibVote GM

echo
echo "-------------------------------------"
echo -e "$BLUE Reading and deserializing an account from a program... $NC"
echo -e "$RED ./flare --cluster devnet read-account --program WixFUMVqBSTygzeFy9Wuy5XxkeH8xHnUEGvfyyJYqve --account 78vJRdkATNZm7cJHaLscYu1HZq24EH3FV6Eppx3BS9qA --idl ./example/onchain_voting.json $NC"
./flare --cluster devnet read-account --program WixFUMVqBSTygzeFy9Wuy5XxkeH8xHnUEGvfyyJYqve --account 78vJRdkATNZm7cJHaLscYu1HZq24EH3FV6Eppx3BS9qA --idl ./example/onchain_voting.json

echo
echo "-------------------------------------"
echo -e "$BLUE Fetching published IDL... $NC"
echo -e "$RED ./flare fetch-idl --program cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ $NC"
./flare fetch-idl --program cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ


