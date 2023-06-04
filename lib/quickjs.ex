defmodule Quickjs do
  @moduledoc """
  Documentation for `Quickjs`.
  """

  def simple_run_script(js_script) do
    case QuickJs.Native.run(js_script) do
      {:ok, json} -> {:ok, Jason.decode!(json)}
      e -> e
    end
  end
end
