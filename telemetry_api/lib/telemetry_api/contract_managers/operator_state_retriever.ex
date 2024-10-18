defmodule TelemetryApi.ContractManagers.OperatorStateRetriever do
  require Logger
  alias TelemetryApi.ContractManagers.OperatorStateRetriever

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

  # -------- PUBLIC FUNCTIONS --------

  def get_contract_address() do
    @contract_address
  end

  def get_operators() do
    with {:ok, block_number} <- Ethers.current_block_number(),
         {:ok, operators_state} <- fetch_operators_state(block_number) do
      parse_operators(operators_state)
    else
      {:error, %{reason: :econnrefused}} -> {:error, "Blockchain is not reachable"}
      {:error, reason} -> {:error, reason}
    end
  end

  # -------- PRIVATE FUNCTIONS --------

  defp parse_operators(operators_state) do
    operators =
      operators_state
      |> Enum.map(fn {address, id, stake} ->
        id = "0x" <> Base.encode16(id, case: :lower)
        address = address |> String.downcase()

        %{
          id: id,
          address: address,
          stake: Integer.to_string(stake)
        }
      end)

    {:ok, operators}
  end

  defp fetch_operators_state(block_number) do
    quorum_numbers = <<0>>

    response =
      OperatorStateRetriever.get_operator_state(
        @registry_coordinator_address,
        quorum_numbers,
        block_number
      )
      |> Ethers.call()

    case response do
      {:ok, [operators | _]} -> {:ok, operators}
      {:error, message} -> {:error, message}
      _ -> {:error, "Bad formated data received from OperatorStateRetriever::getOperatorState"}
    end
  end
end
