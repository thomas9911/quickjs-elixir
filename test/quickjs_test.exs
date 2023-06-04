defmodule QuickjsTest do
  use ExUnit.Case
  doctest Quickjs

  describe "simple_run_script" do
    test "addition" do
      script = "1 + 123"
      assert {:ok, 124} = Quickjs.simple_run_script(script)
    end

    test "function" do
      script = """
      const func = (input) => {
        return input * 4
      }

      func(50)
      """
      assert {:ok, 200} = Quickjs.simple_run_script(script)
    end
  end
end
