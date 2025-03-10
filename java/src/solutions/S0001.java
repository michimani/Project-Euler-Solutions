/**
 * Solution for Project Euler Problem 1.
 *
 * @see <a href="https://projecteuler.net/problem=1">Project Euler Problem 1</a>
 */
public class S0001 {
  public static void main(String[] args) {
    int sum = 0;
    for (int i = 1; i < 1000; i++) {
      if (i % 3 == 0 || i % 5 == 0) {
        sum += i;
      }
    }
    System.out.println("Solution for problem 1: " + sum);
  }
}