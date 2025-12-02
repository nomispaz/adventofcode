
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

public class main {

    public static List<String> file_to_string_array(String filepath) {
        List<String> contents_list = new ArrayList<>();

        try {
            contents_list = Files.readAllLines(Path.of(filepath));
        } catch (IOException e) {
            System.err.println(String.format("Error while reading input file %s: %s", filepath, e));
        }

        return contents_list;
    }

    public static int get_next_state(String current_rotation_direction, int current_rotation_number, int last_state) {
        switch (current_rotation_direction) {
            case "R" -> {
                return Math.abs(Math.floorMod(last_state + current_rotation_number, 100));
            }
            case "L" -> {
                return Math.abs(Math.floorMod(last_state + 100 - current_rotation_number, 100));
            }
        }
        // this should never happen
        return -1;
    }

    public static int count_rotations(int current_rotation_number, String current_rotation_direction, int last_state) {
        int number_rotations = Math.floorDiv(current_rotation_number, 100);
        int remaining_clicks = Math.floorMod(current_rotation_number, 100);
        int next_state = 0;
        switch (current_rotation_direction) {
            case "R" -> {
                next_state = last_state + remaining_clicks;
            }
            case "L" -> {
                next_state = last_state - remaining_clicks;
            }
        }
        if (next_state >= 100 || (next_state <= 0 && last_state != 0)) {
            next_state = Math.abs(Math.floorMod(next_state + 100, 100));
            number_rotations += 1;

        }
        return number_rotations;
    }

    public static int check_state(int next_state) {
        if (next_state == 0) {
            return 1;
        } else {
            return 0;
        }
    }

    public static void main(String[] args) {
        // part 1
        int last_state = 50;
        int next_state = 50;
        int current_rotation_number = 0;
        String current_rotation_direction = "";
        int password = 0;
        List<String> file_contents;

        file_contents = file_to_string_array("../input.input");
        for (String line : file_contents) {
            last_state = next_state;
            current_rotation_number = Integer.parseInt(line.substring(1));
            current_rotation_direction = line.substring(0, 1);

            next_state = get_next_state(current_rotation_direction, current_rotation_number, last_state);

            password += check_state(next_state);
        }
        System.out.println(String.format("The password for part 1 is %s", password));

        password = 0;
        last_state = 50;
        next_state = 50;
        current_rotation_direction = "";
        current_rotation_number = 0;

        for (String line : file_contents) {
            last_state = next_state;
            current_rotation_number = Integer.parseInt(line.substring(1));
            current_rotation_direction = line.substring(0, 1);

            next_state = get_next_state(current_rotation_direction, current_rotation_number, last_state);
            password += count_rotations(current_rotation_number, current_rotation_direction, last_state);

            System.out.println(next_state);
            System.out.println(count_rotations(current_rotation_number, current_rotation_direction, last_state));

        }
        System.out.println(String.format("The password for part 2 is %s", password));

    }
}
