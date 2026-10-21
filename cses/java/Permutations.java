import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class Permutations {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        int n = Integer.parseInt(br.readLine());

        if ( n == 2 || n == 3) {
            System.out.println("NO SOLUTION");
            return;
        }

        if ( n == 4 ) {
            System.out.println("2 4 1 3");
            return;
        }
        StringBuilder output = new StringBuilder();

        for (int i = 1; i < n; i += 2 ) {
            output.append(i).append(' ');
        }
        if (n % 2 != 0) {
             output.append(n).append(' ');
        }
        for (int i = 2; i < n; i += 2 ) {
            output.append(i).append(' ');
        }
        if (n % 2 == 0) {
             output.append(n);
        }
        System.out.println(output);
    }
}
