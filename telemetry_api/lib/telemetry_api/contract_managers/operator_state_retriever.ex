defmodule TelemetryApi.ContractManagers.OperatorStateRetriever do
  require Logger

  @aligned_config_file System.get_env("ALIGNED_CONFIG_FILE")

  config_file_path =
    case @aligned_config_file do
      nil -> raise("ALIGNED_CONFIG_FILE not set in .env")
      file -> file
    end

  {status, config_json_string} = File.read(config_file_path)

  case status do
    :ok ->
      Logger.debug("Aligned deployment file read successfully")

    :error ->
      raise("Config file not read successfully")
  end

  @contract_address Jason.decode!(config_json_string)
                         |> Map.get("addresses")
                         |> Map.get("operatorStateRetriever")

  @registry_coordinator_address Jason.decode!(config_json_string)
                         |> Map.get("addresses")
                         |> Map.get("registryCoordinator")
                         |> String.trim_leading("0x")
                         |> Base.decode16!(case: :mixed)

  use Ethers.Contract,
    abi_file: "priv/abi/OperatorStateRetriever.json",
    default_address: @contract_address

  def get_contract_address() do
    @contract_address
  end


  def get_operators() do
    with {:ok, block_number} = Ethers.current_block_number() do
          quorum_numbers = <<0>>
          response = __MODULE__.get_operator_state(@registry_coordinator_address, quorum_numbers, block_number) |> Ethers.call()
          case response do
            {:ok, [operators | _]} -> 
              operators = 
                operators |> Enum.map(fn op_data -> 
                  {address, id, stake} = op_data
                  id = id |> String.trim_leading("0x") |> String.upcase()
                  address = address |> String.trim_leading("0x") |> String.upcase()
                  %{
                    id: "0x" <> Base.encode16(id, case: :lower),
                    address: "0x" <> address,
                    stake: Integer.to_string(stake)
                  }
                end)
              {:ok, operators}
            {:error, message} -> {:error, message}
            _ -> {:error, "Bad formated data received from OperatorStateRetriever::getOperatorState"}
          end
    end
  end
end
