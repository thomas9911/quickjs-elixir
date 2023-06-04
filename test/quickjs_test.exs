defmodule QuickjsTest do
  use ExUnit.Case, async: true
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

    test "std" do
      script = """
      console.log(1)
      """

      assert {:ok, nil} = Quickjs.simple_run_script(script)
    end
  end

  describe "simple_run_script_timeout" do
    test "works" do
      assert {:ok, 1234} == Quickjs.simple_run_script_timeout("123 * 10 + 4")
    end

    test "timeout should be able to stop infinite loops" do
      assert {:error, :timeout} == Quickjs.simple_run_script_timeout("while(true){}", 200)
    end

    @tag skip: "crashes nif (look into why)"
    test "timeout large memory function" do
      script = """
      function ack(m,n)
      {
          if (m == 0)
              {
                  return n + 1;
              }
              else if((m > 0) && (n == 0))
              {
                  return ack(m - 1, 1);
              }
              else if((m > 0) && (n > 0))
              {
                  return ack(m - 1, ack(m, n - 1));
              }else
              return n + 1;
      }

      ack(5, 3);
      12
      """

      assert {:error, :timeout} == Quickjs.simple_run_script_timeout(script, 500)

    end
  end
end
