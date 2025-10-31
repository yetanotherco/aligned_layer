from argparse import ArgumentParser
from json import load
from eth_abi import encode
from Crypto.Hash import keccak


def encode_call(file, sender_address):
    with open(file) as f:
        data = load(f)

        proof_commitment = bytearray.fromhex(data['proof_commitment'])
        pub_input_commitment = bytearray.fromhex(data['pub_input_commitment'])
        proving_system_aux_data_commitment = bytearray.fromhex(data['program_id_commitment'])
        proof_generator_addr = bytearray.fromhex(data['proof_generator_addr'])
        batch_merkle_root = bytearray.fromhex(data['batch_merkle_root'])
        merkle_proof = bytearray.fromhex(data['merkle_proof'])
        verification_data_batch_index = data['verification_data_batch_index']

        output = encode(['bytes32', 'bytes32', 'bytes32', 'bytes20', 'bytes32', 'bytes', 'uint256', 'address'],
                        [proof_commitment, pub_input_commitment, proving_system_aux_data_commitment,
                         proof_generator_addr, batch_merkle_root, merkle_proof, verification_data_batch_index,
                         sender_address])

        k = keccak.new(digest_bits=256)
        k.update(b'verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256,address)')
        signature = k.hexdigest()[:8]
        return '0x' + signature + output.hex()


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument('--aligned-verification-data', help='Path to JSON file with the verification data')
    parser.add_argument('--sender-address',  help='Address that sent the batch to Aligned')
    args = parser.parse_args()

    data = encode_call(args.aligned_verification_data, args.sender_address)
    print(data)
