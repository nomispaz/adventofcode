defmodule Day3 do
  @moduledoc """
  Documentation for `Day3`.
  """

  @doc """
  Hello world.

  """
  def read_file(file) do
    {:ok, contents} = File.read(file)

    content_string = contents
    |> String.replace("\n", "")

    content_string
  end

  def eval_conditions(contents) do
    contents_split = contents
    # after the split at don't, a list of everything in between two don'ts is created
    |> String.split("don't()")
    |> (fn [firstitem|tail] ->
      # keep the first item as is since the at the start of the string, do is set implicitely
      [firstitem | Enum.map(tail, fn a -> 
          [_dontexecute|execute] = a 
          |> String.split("do()")
          execute
        end)]
    end).()
    |> Enum.join()



    # in the first part of the split, there are mul-commands before don't --> execute them
    result = eval_muls(contents_split)

  end

  def eval_muls(contents) do
    # use a regex
    regex = ~r/mul\([0-9]*,[0-9]*\)/
    result = Regex.scan(regex, contents)
      # fn [entry] since every entry in list is a list
      |> Enum.map(fn [entry] -> 
      entry
      |> String.replace("mul(", "")
      |> String.replace(")", "")
      |> String.split(",")
      # after this we have a list of a list of two integers
      |> Enum.map(&String.to_integer/1)
      # define anonymous function and directly execute with .()
      |> (fn [a, b] -> a * b end).()
      end)
    |> Enum.sum()

    result
  end

  def start do
   
    contents = Day3.read_file("input.txt")
    part1 = Day3.eval_muls(contents)
    IO.puts("Result part1: #{part1}")
    part2 = Day3.eval_conditions(contents)
    IO.puts("Result part2: #{part2}")
  end
  
end

Day3.start()

