n = 1000
script = """
let arr = []
for (let i = 0;  i < 1000; i++) {
  arr.push(i)
}
arr[arr.length - 1]
"""
IO.inspect(Quickjs.simple_run_script(script))

microseconds =
  fn -> Enum.map(0..n, fn _ -> Quickjs.simple_run_script(script) end) end
  |> :timer.tc()
  |> elem(0)
  |> Kernel.div(n)

IO.puts(microseconds)
