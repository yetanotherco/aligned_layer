defmodule SignatureVerifier do
  alias ExSecp256k1
  alias ExKeccak
  alias :binary, as: Binary

  # Hash the version string using Keccak256
  defp hash_version(version) do
    ExKeccak.hash_256(version)
  end

  # Recover the public key from the signature and hashed version
  defp recover_public_key(hash, signature, recovery_id) do
    case ExSecp256k1.recover_compact(hash, signature, recovery_id) do
      {:ok, public_key} -> {:ok, public_key}
      _error -> {:error, :bad_request, "Failed to recover public key"}
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

  @doc """
  Get the address from the version and signature

  Examples
      iex> version = "v0.7.0"
      iex> signature = N1UJOvjJT1W39MdQUYAOsKZj4aQ1Sjkwp31NJgafpjoUniGt24tSaLw6TlTKP68AkLtsIFoVEaJcJDj7TyvhLQA=
      iex> recover_address(version, signature)
      "0x..."
  """
  def recover_address(version, signature) do
    version_hash = hash_version(version)
    # Signature contains r, s and v (recovery_id)
    # r<>s is 64 bytes.
    # v is the last byte of the signature and have to be converted to integer
    {:ok, binary_signature} = Base.decode64(signature)
    signature_len = byte_size(binary_signature)
    rs = binary_part(binary_signature, 0, signature_len - 1)
    recovery_id = Binary.decode_unsigned(binary_part(binary_signature, signature_len - 1, 1))

    with {:ok, address} <- recover_public_key(version_hash, rs, recovery_id) do
      addr =
        public_key_to_address(address)
        |> Base.encode16(case: :lower)

      {:ok, addr}
    end
  end
end
