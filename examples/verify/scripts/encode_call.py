from argparse import ArgumentParser
from json import load
from eth_abi import encode
from Crypto.Hash import keccak


def encode_call(file):
    with open(file) as f:
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

        return signature + output.hex()


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument('--aligned-verification-data', help='Path to JSON file with the verification data')
    args = parser.parse_args()

    data = encode_call(args.aligned_verification_data)
    print(data)