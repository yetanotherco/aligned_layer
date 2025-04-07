defmodule Explorer.Utils do
  def sha256_hash(data) do
    ## Base 16 encoder needed as crypto.hash returns raw bytes
    :crypto.hash(:sha256, data) |> Base.encode16(case: :lower)
  end

  ## Returns hash raw bytes
  def sha256_hash_raw(data) do
    :crypto.hash(:sha256, data)
  end

  def decimal_to_hex(number) do
    "0x#{Integer.to_string(number, 16)}"
  end
end
