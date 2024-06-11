import os
from eth_account import Account
from eth_utils import to_checksum_address

def generate_random_address():
    # Generate a random private key
    private_key = os.urandom(32)
    # Derive the public key and address
    account = Account.from_key(private_key)
    # Get the checksummed address
    checksummed_address = to_checksum_address(account.address)
    return checksummed_address

if __name__ == "__main__":
    print(generate_random_address())
