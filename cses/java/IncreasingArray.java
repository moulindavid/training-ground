import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class IncreasingArray {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        int n = Integer.parseInt(br.readLine());
        String[] parts = br.readLine().split(" ");
        Long turn = 0l;

        for (int i = 0 ; i < n-1; i++) {
            var j = i+1;
            var current = Long.parseLong(parts[i]);
            var above = Long.parseLong(parts[j]);
            while (above < current) {
                var diff = current - above;
                above += diff;
                turn += diff;
            }
            parts[j] = String.valueOf(above);
        }

        System.out.println(turn);
    }
}
