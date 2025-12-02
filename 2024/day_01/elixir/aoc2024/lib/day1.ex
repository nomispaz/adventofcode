defmodule Day1 do
  @moduledoc """
  Documentation for `Day1`.
  """

  @doc """

  """
  def read_and_sort_file_stream(file) do
    # pattern matching. Only continue if the return of File.read is ok
    # {:ok, contents} = File.read("/mnt/nvme2/git_repos/prog_tests/elixir/aoc2024/input/day1.txt")
    # contents_split = 
    #  contents
    #  |> String.split("\n", trim: true)

    {list1, list2} =
      file
      # read line by line
      |> File.stream!()
      # remove trailing or preceding whitespaces and \n
      |> Stream.map(&String.trim/1)
      # Split every line into a list and convert to tuple and convert to integer
      |> Stream.map(fn line ->
        [a, b] = String.split(line) |> Enum.map(&String.to_integer/1)
        {a, b}
      end)
      # extract tuple from stream and return as a tuple (but now as enum)
      |> Enum.unzip()

    sortedlist1 = Enum.sort(list1)
    sortedlist2 = Enum.sort(list2)

    # returnvalue
    {sortedlist1, sortedlist2}
  end

  def read_and_sort_file(file) do
    # read complete file into string. Directly failes if return value of read is not :ok
    {:ok, contents} = File.read(file)

    {list1, list2} =
      contents
      # trim: true removes empty lines
      |> String.split("\n", trim: true)
      # Split every line into a list and convert to tuple and convert to integer
      |> Enum.map(fn line ->
        [a, b] = String.split(line) |> Enum.map(&String.to_integer/1)
        {a, b}
      end)
      # returns two lists
      |> Enum.unzip()

    sortedlist1 = Enum.sort(list1)
    sortedlist2 = Enum.sort(list2)

    # returnvalue
    {sortedlist1, sortedlist2}
  end

  def compute_diff_part1(list1, list2, sum) do
    [head1 | tail1] = list1
    [head2 | tail2] = list2
    sum = sum + abs(head1 - head2)

    cond do
      # if there is no remaining element in the list, stop computing
      length(tail1) > 0 -> compute_diff_part1(tail1, tail2, sum)
      true -> sum
    end
  end

  def compute_diff_part2(list1, list2, sum) do
    [head1 | tail1] = list1

    entry_count =
      Enum.count(list2, fn entry ->
        entry == head1
      end)

    sum = sum + entry_count * head1

    cond do
      # if there is no remaining element in the list, stop computing
      length(tail1) > 0 -> compute_diff_part2(tail1, list2, sum)
      true -> sum
    end
  end
end

{list1, list2} = Day1.read_and_sort_file("input.txt")
# IO.inspect(list1)
# IO.inspect(list2)

sum = Day1.compute_diff_part1(list1, list2, 0)
IO.puts("Result for part1: #{sum}")

sum = Day1.compute_diff_part2(list1, list2, 0)
IO.puts("Result for part2: #{sum}")
