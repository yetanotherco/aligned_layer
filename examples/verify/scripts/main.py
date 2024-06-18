from web3 import Web3
from Crypto.Hash import keccak
from eth_abi import encode
from argparse import ArgumentParser
from json import load


def main():
    parser = ArgumentParser()
    parser.add_argument('--rpc-url', default='https://ethereum-holesky-rpc.publicnode.com',
                        help='RPC URL (default: https://ethereum-holesky-rpc.publicnode.com)')
    parser.add_argument('--aligned-verification-data', help='Path to JSON file with the verification data',
                        required=True)
    parser.add_argument('--contract-address', help='Verifier Contract address', required=True)

    args = parser.parse_args()

    provider = Web3(Web3.HTTPProvider(args.rpc_url))

    with open(args.aligned_verification_data) as f:
        data = load(f)

        verification_data_commitment = data['verification_data_commitment']
        proof_commitment = bytearray(verification_data_commitment['proof_commitment'])
        pub_input_commitment = bytearray(verification_data_commitment['pub_input_commitment'])
        proving_system_aux_data_commitment = bytearray(
            verification_data_commitment['proving_system_aux_data_commitment'])
        proof_generator_addr = bytearray(verification_data_commitment['proof_generator_addr'])
        batch_merkle_root = bytearray(data['batch_merkle_root'])

        merkle_path_arr = data['batch_inclusion_proof']['merkle_path']
        merkle_proof = bytearray()
        for i in range(0, len(merkle_path_arr)):
            merkle_proof += bytearray(merkle_path_arr[i])

        index = data['verification_data_batch_index']

        output = encode(['bytes32', 'bytes32', 'bytes32', 'bytes20', 'bytes32', 'bytes', 'uint256'],
                        [proof_commitment, pub_input_commitment, proving_system_aux_data_commitment,
                         proof_generator_addr, batch_merkle_root, merkle_proof, index])

        k = keccak.new(digest_bits=256)
        k.update(b'verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256)')
        signature = k.hexdigest()[:8]

        data = signature + output.hex()

        result = provider.eth.call({
            'to': args.contract_address,
            'data': data
        })

        # Check result last byte is 1
        if result[-1] == 1:
            print("Batch inclusion proof is valid")
        else:
            print("Batch inclusion proof is invalid")


if __name__ == "__main__":
    main()
