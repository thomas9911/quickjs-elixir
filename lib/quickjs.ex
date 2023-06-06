defmodule Quickjs do
  @moduledoc """
  Documentation for `Quickjs`.
  """

  def simple_run_script(js_script) do
    case QuickJs.Native.run(js_script) do
      {:ok, json} -> {:ok, Jason.decode!(json)}
      e -> e
    end
  rescue
    e ->
      {:error, e}
  end

  def simple_run_script_timeout(js_script, timeout \\ 5000) do
    task = Task.async(fn -> simple_run_script(js_script) end)
    Task.await(task, timeout)
  catch
    :exit, {:timeout, _task} ->
      {:error, :timeout}
  end
end
