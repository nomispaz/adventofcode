defmodule Day2 do
  @moduledoc """
  Documentation for `Day2`.
  """

  @doc """
  Hello world.
  """
  def read_file(file) do
    {:ok, contents} = File.read(file)
    contents
    |> String.split("\n", trim: true)
    |> Enum.map(fn line -> String.split(line, " ", trim: true) |> Enum.map(&String.to_integer/1) end)
#    |> IO.inspect()
  end

  def parse_list(list, sum, run_dampener) do
    [currentrow|tails] = list
    sum = check_for_errors(currentrow, currentrow, nil, "unknown", run_dampener, sum)
    # IO.inspect("Sum: #{sum}")
    cond do
      length(tails) > 0 -> parse_list(tails, sum, run_dampener)
      true -> sum
    end
  end

  defp is_ascending?(val1, val2) do
      cond do
        val1 < val2 -> true
        val1 > val2 -> false
        true -> "unknown"
      end
  end

  # used in the error case. removes one of the possible error-values and checks if after the change everything is fine.
  defp dampener(complete_row, row, last_item, direction, sum) do
    [firstitem|tails] = row
    [seconditem|tails2] = tails
    
    # special case: direction changes between items 2 and 3 --> it might help to remove the very first item of the original list
    new_direction = is_ascending?(firstitem, seconditem)

    case last_item do
      # there is no last item --> this happened during the first two items of the list
      # --> try by removing the first resp. the second item of the list
      nil -> max(check_for_errors(complete_row, [firstitem|tails2], nil, direction, false, sum), check_for_errors(complete_row, tails, nil, direction, false, sum))
      _ -> cond do
             # problem is solved if last item is removed
             tails2 == [] -> sum + 1
             # else
             true -> cond do
                       # special case: when the direction changes between the second and third entry, the first entry could be the problem and the problem is solved by reversing the direction
                       # --> restart with unknown direction and without the first entry of the complete_row. Additionally try to remove the other two relevant items
                       length(complete_row) - length(row) == 1 && direction != new_direction -> 
                         sum1 = check_for_errors(complete_row, row, nil, new_direction, false, sum)
                         sum2 = check_for_errors(complete_row, [last_item|tails], nil, "unknown", false, sum)
                         sum3 = check_for_errors(complete_row, [firstitem|tails2], nil, direction, false, sum)
                         sum = max(max(sum1, sum2), sum3)
                         sum
                       true -> max(check_for_errors(complete_row, [last_item|tails], nil, direction, false, sum), check_for_errors(complete_row, [firstitem|tails2], nil, direction, false, sum))
                    end
           end
    end
  end

  defp check_for_errors(complete_row, row, last_item, direction, run_dampener, sum) do
    #IO.inspect(row, charlists: :as_lists)
    [firstitem|tails] = row
    [seconditem|tails2] = tails

    ascending = 
      case direction do
        "unknown" -> is_ascending?(firstitem, seconditem)
        _ -> direction
      end

    distance = firstitem - seconditem
    cond do
      # no error, not at the end --> next run
      distance < 0 && distance > -4 && ascending == true && tails2 != [] -> check_for_errors(complete_row, [seconditem|tails2], firstitem, ascending, run_dampener, sum)
      # no error, last item visited --> return and increase sum
      distance < 0 && distance > -4 && ascending == true && tails2 == [] -> sum + 1
      distance > 0 && distance < 4 && ascending == false && tails2 != [] -> check_for_errors(complete_row, [seconditem|tails2], firstitem, ascending, run_dampener, sum)
      distance > 0 && distance < 4 && ascending == false && tails2 == [] -> sum + 1

      # some error occured --> start the dampener if dampener is on
      true ->
        case run_dampener do
          true -> dampener(complete_row, row, last_item, direction, sum)
          false -> sum
        end    
    end
  end
end

list = Day2.read_file("input.txt")
sum = Day2.parse_list(list, 0, true)
IO.puts("Result: #{sum}")
