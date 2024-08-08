defmodule StakeRegistryManager do
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
      raise(
        "Config file not read successfully, did you run make create-env? If you did,\n make sure Eigenlayer config file is correctly stored"
      )
  end

  @stake_registry_manager Jason.decode!(config_json_string)
                         |> Map.get("addresses")
                         |> Map.get("stakeRegistry")

  use Ethers.Contract,
    abi_file: "lib/abi/StakeRegistry.json",
    default_address: @stake_registry_manager

  def get_stake_registry_manager() do
    @stake_registry_manager
  end

  def get_latest_stake_update(%{fromBlock: fromBlock, operator_id: operator_id}) do
      StakeRegistryManager.EventFilters.operator_stake_update(operator_id)
        |> Ethers.get_logs(fromBlock: fromBlock)
        |> case do
          {:ok, data} ->
            List.last(data) # most recent entry

          {:error, reason} ->
            dbg("Error getting latest operator stake update")
            dbg(reason)

          other ->
            dbg("Unexpected response:")
            dbg(other)
        end
  end

  def get_strategies_of_quorum(quorum_number) do
    "get_strategies_of_quorum" |> dbg

    amount_of_strategies = case StakeRegistryManager.strategy_params_length(quorum_number) |> Ethers.call() do
      {:ok, amount} ->
        amount
      {:error, error} ->
        dbg("Error fetching amount of strategies: #{error}")
        raise("Error fetching amount of strategies: #{error}")
    end

    strategies = Enum.reduce(0..(amount_of_strategies - 1), [], fn index, acc ->
      case StakeRegistryManager.strategies_per_quorum(quorum_number, index) |> Ethers.call() do
        {:ok, strategy_address} ->
          [strategy_address | acc]
        {:error, error} ->
          dbg("Error fetching strategy at index #{index}: #{error}")
          acc
      end
    end)

    strategies |> Enum.reverse()
  end

end

  # relevant structs:
  # /// @ struct used to store the stakes of an individual operator or the sum of all operators' stakes, for storage
  # struct StakeUpdate {
  #     // the block number at which the stake amounts were updated and stored
  #     uint32 updateBlockNumber;
  #     // the block number at which the *next update* occurred.
  #     /// @notice This entry has the value **0** until another update takes place.
  #     uint32 nextUpdateBlockNumber;
  #     // stake weight for the quorum
  #     uint96 stake;
  # }

  # relevant events:

  # /// @ emitted whenever the stake of `operator` is updated
  # event OperatorStakeUpdate(
  #     bytes32 indexed operatorId,
  #     uint8 quorumNumber,
  #     uint96 stake
  # );

  # relevant views:
#   /**
#   * @ This function computes the total weight of the @param operator in the quorum @param quorumNumber.
#   */
#  function weightOfOperatorForQuorum(uint8 quorumNumber, address operator) external view returns (uint96);

# /**
# * @ Returns the most recent stake weight for the `operatorId` for a certain quorum
# * @dev Function returns an StakeUpdate struct with **every entry equal to 0** in the event that the operator has no stake history
# */
# function getLatestStakeUpdate(bytes32 operatorId, uint8 quorumNumber) external view returns (StakeUpdate memory);

# /**
# * @ Returns the stake weight from the latest entry in `_totalStakeHistory` for quorum `quorumNumber`.
# */
# function getCurrentTotalStake(uint8 quorumNumber) external view returns (uint96);

# /**
# * @ Returns the most recent stake weight for the `operatorId` for quorum `quorumNumber`
# */
# function getCurrentStake(bytes32 operatorId, uint8 quorumNumber) external view returns (uint96);
