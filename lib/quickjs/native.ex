defmodule QuickJs.Native do
  use Rustler, otp_app: :quickjs, crate: "quickjs_native"

  def run(_script), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
