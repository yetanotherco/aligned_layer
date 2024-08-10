defmodule StrategyManager do
  require Logger

  @environment System.get_env("ENVIRONMENT")

  file_path =
    "../contracts/script/output/#{@environment}/eigenlayer_deployment_output.json"

  {status, config_json_string} = File.read(file_path)

  case status do
    :ok ->
      Logger.debug("Eigenlayer deployment file read successfully")

    :error ->
      raise(
        "Config file not read successfully, did you run make create-env? If you did,\n make sure Eigenlayer config file is correctly stored"
      )
  end
  
  @strategy_manager Jason.decode!(config_json_string)
                         |> Map.get("addresses")
                         |> Map.get("strategyManager")

  def get_strategy_manager() do
    @strategy_manager
  end

  use Ethers.Contract,
    abi_file: "lib/abi/StrategyManager.json",
    default_address: @strategy_manager

  def get_staker_deposits(%Operators{id: operator_id}) do
    operator_address = Operators.get_operator_by_id(operator_id).address
    case StrategyManager.get_deposits(operator_address) |> Ethers.call do
      {:ok, deposits} ->
        dbg deposits
        deposits
      error ->
        dbg("Error fetching deposits")
        dbg(error)
        error
    end
  end
end
