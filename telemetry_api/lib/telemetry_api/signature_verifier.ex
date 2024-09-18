defmodule SignatureVerifier do
  alias ExSecp256k1
  alias ExKeccak
  alias :binary, as: Binary

  # Hash the version string using Keccak256
  def hash_version(version) do
    version
    |> ExKeccak.hash_256()
  end

  # Recover the public key from the signature and hashed version
  defp recover_public_key(hash, signature, recovery_id) do
    case ExSecp256k1.recover_compact(hash, signature,  recovery_id) do
      {:ok, public_key} -> public_key
      _error -> {:error, "Failed to recover public key"}
    end
  end

  # Convert public key to Ethereum-like address
  def public_key_to_address(public_key) do
    # Remove the first byte (which is 0x04 for uncompressed public keys)
    public_key = binary_part(public_key, 1, 64)

    # Hash the public key with Keccak256
    public_key_hash = ExKeccak.hash_256(public_key)

    # Get the last 20 bytes (Ethereum address format)
    <<_::binary-size(12), address::binary-size(20)>> = public_key_hash
    address
  end

  # Main function to get the address from the version and signature
  def get_address(version, signature) do
    version_hash = hash_version(version)
    {:ok, binary_signature} = Base.decode64(signature)
    byte_size = byte_size(binary_signature)
    IO.inspect(byte_size)
    # r<>s is 64 bytes. Get r and s from the signature
    rs = binary_part(binary_signature, 0, byte_size - 1)
    # v is the last byte of the signature
    recovery_id = Binary.decode_unsigned(binary_part(binary_signature, byte_size - 1, 1))
    IO.inspect(recovery_id)
    recover_public_key(version_hash, rs, recovery_id)
    |> public_key_to_address()
    |> Base.encode16()
  end
end
