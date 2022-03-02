global using System.IO;
using System.Reflection;

const int TOTAL_PUZZLES = 25;


if (args.Length > 0 && int.TryParse(args[0], out int puzzleToRun)) {
    InvokePuzzle(puzzleToRun);
}
else {
    for (int i = 1; i <= TOTAL_PUZZLES; i++) {
        var puzzleClass = Type.GetType($"Puzzles.Puzzle{i}", false);
        puzzleClass
            ?.GetMethod("RunPuzzle", BindingFlags.Static | BindingFlags.Public)
            ?.Invoke(null, null);
    }
}


void InvokePuzzle(int puzzleNumber) {
    var puzzleClass = Type.GetType($"Puzzles.Puzzle{puzzleNumber}", false);
    puzzleClass
        ?.GetMethod("RunPuzzle", BindingFlags.Static | BindingFlags.Public)
        ?.Invoke(null, null);
}

namespace Puzzles {
    
    public class InputReader {
        
        private readonly string _path;
        
        public InputReader(string path) {
            _path = path;
        }
        
        public List<T> ReadFileToList<T>() {
            List<T> list = new();
            
            using StreamReader sr = new(_path);

            string? line;
            while ((line = sr.ReadLine()) is not null) {
                list.Add((T)Convert.ChangeType(line, typeof(T)));
            }

            return list;
        }

        public string ReadFileToLine() {
            return File.ReadAllText(_path);
        }
    }

}