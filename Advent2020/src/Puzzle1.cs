using System.Reflection.Emit;

namespace Puzzles; 

public class Puzzle1 {
    public static void RunPuzzle() {
        const int PUZZLE_NUMBER = 1;
        InputReader ir = new(@$"./inputs/input{PUZZLE_NUMBER}.txt");

        List<int> numbers = ir.ReadFileToList<int>();
        // part 1
        foreach (int firstNum in numbers) {
            foreach (int secondNum in numbers.Where(secondNum => firstNum + secondNum == 2020)) {
                Console.WriteLine($"Output: {firstNum * secondNum}");
                return;
            }
        }
        // part 2
        foreach (int firstNum in numbers) {
            foreach (int secondNum in numbers.Where(secondNum => firstNum + secondNum < 2020)) {
                foreach (int thirdNum in numbers.Where(thirdNum => firstNum + secondNum + thirdNum == 2020)) {
                    Console.WriteLine($"Output: {firstNum * secondNum * thirdNum}");
                    return;
                }
            }
        }
    }
}