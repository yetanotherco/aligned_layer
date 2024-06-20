from web3 import Web3
from argparse import ArgumentParser
from encode_call import encode_call


def main():
    parser = ArgumentParser()
    parser.add_argument('--rpc-url', default='https://ethereum-holesky-rpc.publicnode.com',
                        help='RPC URL (default: https://ethereum-holesky-rpc.publicnode.com)')
    parser.add_argument('--aligned-verification-data', help='Path to JSON file with the verification data',
                        required=True)
    parser.add_argument('--contract-address', help='Verifier Contract address', required=True)

    args = parser.parse_args()

    provider = Web3(Web3.HTTPProvider(args.rpc_url))

    data = encode_call(args.aligned_verification_data)

    result = provider.eth.call({
        'to': args.contract_address,
        'data': data
    })

    # Check result last byte is 1
    if result[-1] == 1:
        print("Submitted proof with associated data is verified in ethereum blockchain")
    else:
        print("Not verified in ethereum blockchain")


if __name__ == "__main__":
    main()
