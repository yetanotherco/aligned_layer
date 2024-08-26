defmodule EigenOperatorMetadataStruct do
  defstruct [:name, :website, :description, :logo, :twitter]

  def map_to_struct(%{
    "name" => name,
    "website" => website,
    "description" => description,
    "logo" => logo,
    "twitter" => twitter
  }) do
    %EigenOperatorMetadataStruct{
      name: name,
      website: website,
      description: description,
      logo: logo,
      twitter: twitter
    }
  end

  def map_to_struct(other) do
    dbg(other)
    {:error, :invalid_format}
  end

end
