defmodule Day4 do
  @moduledoc """
  Documentation for `Day4`.
  """

  @doc """
  Hello world.

  """
  def read_file(file) do
    {:ok, contents} = File.read(file)
    contents
    |> String.split("\n", trim: true)
    # directly replace any character that doesn't fit XMAS with .
    |> Enum.map(fn entry -> Regex.replace(~r/[^XMAS]/, entry, ".") end)
  end

  defp find_next_char(contents, xmas_idx, cur_x, cur_y, direction_x, direction_y) do
    search_string = "XMAS"
    current_char = String.at(search_string, xmas_idx)

    if cur_x == -1 || cur_y == -1 do
      0
    else
      case Enum.fetch(contents, cur_y) do
        :error -> #IO.puts("Out of bounds --> stop this try")
                  0
        {:ok, row} -> 
          # extract the character from the row returned by Enum.fetch
          c = String.at(row, cur_x)
          cond do
            c == current_char and xmas_idx < 3 -> find_next_char(contents, xmas_idx + 1, cur_x + direction_x, cur_y + direction_y, direction_x, direction_y)
            c == current_char and xmas_idx == 3 -> 1
            c == current_char -> 0
            true -> 0
          end
      end
    end
  end


  defp search_xmas(contents, directions, x_pos, y_pos, sum) do
    # get length of current row
    {:ok, entry} = Enum.fetch(contents, y_pos)
    entry_length = String.length(entry)
    number_rows = length(contents)
    
    # start iteration for all directions for the current direction starting with X
    sum_at_pos = 
      Enum.map(directions, 
        fn {direction_x, direction_y} -> find_next_char(contents, 0, x_pos, y_pos, direction_x, direction_y)
        end)
      # here we have a list of 8 return values (one for each iteration) --> Sum for the total result
      |> Enum.sum()

    cond do
      # there still is another entry in the row
      x_pos < entry_length -> search_xmas(contents, directions, x_pos + 1, y_pos, sum_at_pos + sum)
      # the last entry in the row was processed -> restart with the first entry in the next row
      x_pos == entry_length and y_pos < number_rows - 1 -> search_xmas(contents, directions, 0, y_pos + 1, sum_at_pos + sum)
      true -> sum_at_pos + sum
    end
  end

  defp search_x_mas(contents, directions, x_pos, y_pos, sum) do
    
  end

  def start do
    IO.puts(Time.utc_now())
    contents = Day4.read_file("input.txt")
    directions = [{0, -1}, {1, -1}, {1, 0}, {1, 1}, {0, 1}, {-1, 1}, {-1, 0}, {-1, -1}]
    part1 = search_xmas(contents, directions, 0, 0, 0)
    directions = [{1, -1}, {1, 1}, {-1, 1}, {-1, -1}]
    IO.inspect("Result: #{part1}")
    IO.puts(Time.utc_now())
  end
end

Day4.start()
