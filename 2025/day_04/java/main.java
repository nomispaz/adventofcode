import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class main {

    enum Direction {
        N,
        NE,
        E,
        SE,
        S,
        SW,
        W,
        NW
    }

    private static List<Integer> decode_direction(Direction direction) {
        switch (direction) {
            case N:
                return Arrays.asList(0, -1);
            case NE:
                return Arrays.asList(1, -1);
            case E:
                return Arrays.asList(1, 0);
            case SE:
                return Arrays.asList(1, 1);
            case S:
                return Arrays.asList(0, 1);
            case SW:
                return Arrays.asList(-1, 1);
            case W:
                return Arrays.asList(-1, 0);
            case NW:
                return Arrays.asList(-1, -1);
            // this should never happen
            default:
                return Arrays.asList(-100, -100);
        }
    }

    public static List<String> file_to_string_array(String filepath) {
        List<String> contents_list = new ArrayList<>();

        try {
            contents_list = Files.readAllLines(Path.of(filepath));
        } catch (IOException e) {
            System.err.println(String.format("Error while reading input file %s: %s", filepath, e));
        }

        return contents_list;
    }

    // build matrix from input-file (rows = y, cols = x)
    public static char[][] buildMatrix(List<String> lines) {
        int rows = lines.size();
        int cols = lines.stream().mapToInt(String::length).max().orElse(0);
        char[][] matrix = new char[rows][cols];
        for (int y = 0; y < rows; y++) {
            String line = lines.get(y);
            for (int x = 0; x < cols; x++) {
                matrix[y][x] = x < line.length() ? line.charAt(x) : ' ';
            }
        }
        return matrix;
    }

    // new: safe accessor by (x,y) where x = column, y = row
    public static char getCharAt(char[][] matrix, int x, int y) {
        if (matrix == null) {
            throw new IllegalArgumentException("matrix is null");
        }
        if (y < 0 || y >= matrix.length) {
            return ' ';
        }
        if (matrix.length == 0) {
            throw new IndexOutOfBoundsException("matrix has zero rows");
        }
        if (x < 0 || x >= matrix[0].length) {
            return ' ';
        }
        return matrix[y][x];
    }

    public static void main(String[] args) {
        List<String> input_lines = file_to_string_array("../input.input");
        List<Direction> directions = List.of(Direction.N, Direction.NE, Direction.E, Direction.SE, Direction.S,
                Direction.SW, Direction.W, Direction.NW);
        List<Integer> next_direction;
        Integer number_hits = 0;
        Integer result = 0;

        // build matrix so you can access characters by x,y
        char[][] matrix = buildMatrix(input_lines);

        // build a List of matrix-entries that contain "@" that had >= 4 adjacent "@"
        List<List<Integer>> remaining_entries = new ArrayList<>();
        ;

        // loop through all lines of the input
        for (int line_idx = 0; line_idx < input_lines.size(); line_idx++) {
            String line = input_lines.get(line_idx);
            // loop through all chars or the current line
            for (int idx = 0; idx < line.length(); idx++) {
                number_hits = 0;
                char ch = line.charAt(idx);
                if (ch != '@') {
                    continue;
                } else {
                    if (matrix.length > 1 && matrix[0].length > 2) {
                        for (Direction direction : directions) {
                            next_direction = decode_direction(direction);
                            if (getCharAt(matrix, idx + next_direction.get(0),
                                    line_idx + next_direction.get(1)) == '@') {
                                number_hits += 1;
                            }
                            if (number_hits >= 4) {
                                // add entry as remaining entry
                                remaining_entries.add(Arrays.asList(idx, line_idx));
                                break;
                            }
                        }
                        if (number_hits < 4) {
                            result += 1;
                        }
                    }
                }
            }
        }
        System.out.println("Result for Part 1: " + result);
    }
}
