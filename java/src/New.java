import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

/**
 * Utility to create a new Project Euler solution file based on the template.
 *
 * Usage: java New [solution_number]
 * Example: java New 2 (creates S0002.java)
 */
public class New {
  /**
   * Main method to create a new solution file.
   *
   * @param args Command line arguments. Expects the solution number as the first
   *             argument.
   */
  public static void main(String[] args) {
    if (args.length == 0) {
      System.err.println("Please provide a solution number");
      System.exit(1);
    }

    try {
      int solutionNumber = Integer.parseInt(args[0]);
      createSolutionFile(solutionNumber);
    } catch (NumberFormatException e) {
      System.err.println("Invalid solution number: " + args[0]);
      System.exit(1);
    } catch (IOException e) {
      System.err.println("Error creating solution file: " + e.getMessage());
      System.exit(1);
    }
  }

  /**
   * Creates a new solution file based on the template.
   *
   * @param solutionNumber The number of the solution to create
   * @throws IOException If there is an error reading or writing files
   */
  private static void createSolutionFile(int solutionNumber) throws IOException {
    System.out.println("Creating solution file for problem " + solutionNumber);
    String newFileName = String.format("S%04d.java", solutionNumber);
    String newFilePath = "src/solutions/" + newFileName;

    Path newFile = Paths.get(newFilePath);

    if (Files.exists(newFile)) {
      System.err.println("File already exists: " + newFilePath);
      System.exit(1);
    }

    String template = """
        /**
         * Solution for Project Euler Problem %d.
         *
         * @see <a href="https://projecteuler.net/problem=%d">Project Euler Problem %d</a>
         */
        public class S%04d {
          public static void main(String[] args) {
            int sum = 0;
            System.out.println("The answer for problem %d: " + sum);
          }
        }
        """;

    String content = template.formatted(solutionNumber, solutionNumber, solutionNumber, solutionNumber, solutionNumber);

    // Update class name
    // Ensure the directory exists
    Files.createDirectories(newFile.getParent());

    // Write the new file
    Files.writeString(newFile, content);

    System.out.println("Successfully created " + newFilePath);
  }
}
