// See https://aka.ms/new-console-template for more information

using System.Diagnostics;

Console.WriteLine("Hello, World!");

const string PATH = "../../../input.txt";

List<string> lines = new ();

string? line;
using var sr = new StreamReader(PATH);
while ((line = sr.ReadLine()) != null) {
    lines.Add(line);
}

var map = new Map(lines);
int steps = 1;
while (map.Step()) {
    steps++;
    Console.WriteLine($"Step: {steps}");
    Console.WriteLine(map);
}

Console.WriteLine(steps);

internal class SeaCucumber {
    
    public (int,int) MyPosition { get; set; }
    public bool IsMoving { get; set; }
    public Direction MyDirection { get; set; }

    public enum Direction {
        East,
        South
    }
    // East go first, then South
    public static readonly Dictionary<char, Direction> DirectionDictionary = new() {
        {'>', Direction.East},
        {'v', Direction.South}
    };
}

internal class Map {
    public SeaCucumber?[,] SeaCucumbers { get; }
    
    public List<SeaCucumber> EastCucumbers { get; }
    public List<SeaCucumber> SouthCucumbers { get; }

    public SeaCucumber? this[int i, int j] {
        get => SeaCucumbers[i, j];
        set => SeaCucumbers[i, j] = value;
    }

    public SeaCucumber? this[(int, int) position] {
        get => SeaCucumbers[position.Item1, position.Item2];
        set => SeaCucumbers[position.Item1, position.Item2] = value;
    }

    public Map(IReadOnlyList<string> lines) {
        SeaCucumbers = new SeaCucumber?[lines.Count, lines[0].Length];
        EastCucumbers = new List<SeaCucumber>();
        SouthCucumbers = new List<SeaCucumber>();
        
        for (int i = 0; i < SeaCucumbers.GetLength(0); i++) {
            char[] rowChars = lines[i].ToCharArray();
            for (int j = 0; j < SeaCucumbers.GetLength(1); j++) {
                var seaCucumber = rowChars[j] != '.' ? new SeaCucumber() : null;
                if (seaCucumber is not null) {
                    seaCucumber.MyPosition = (i, j);
                    seaCucumber.MyDirection = SeaCucumber.DirectionDictionary[rowChars[j]];
                    switch (seaCucumber.MyDirection) {
                        case SeaCucumber.Direction.East:
                            EastCucumbers.Add(seaCucumber);
                            break;
                        case SeaCucumber.Direction.South:
                            SouthCucumbers.Add(seaCucumber);
                            break;
                    }
                }
                SeaCucumbers[i, j] = seaCucumber;
            }
        }
    }

    public bool Step() {
        bool moved = false;
        foreach (var cucumber in EastCucumbers) {
            if ((cucumber.MyPosition.Item2 + 1 >= SeaCucumbers.GetLength(1) &&
                 SeaCucumbers[cucumber.MyPosition.Item1, 0] is null) ||
                (cucumber.MyPosition.Item2 + 1 < SeaCucumbers.GetLength(1) &&
                 SeaCucumbers[cucumber.MyPosition.Item1, cucumber.MyPosition.Item2 + 1] is null)) {
                cucumber.IsMoving = true;
                moved = true;
            }
        }

        foreach (var cucumber in EastCucumbers) {
            if (!cucumber.IsMoving) continue;
            var oldPosition = cucumber.MyPosition;
            this[cucumber.MyPosition]!.MyPosition = (cucumber.MyPosition.Item1,
                cucumber.MyPosition.Item2 + 1 >= SeaCucumbers.GetLength(1)
                ? 0
                : cucumber.MyPosition.Item2 + 1);
            this[cucumber.MyPosition] = cucumber;
            this[oldPosition] = null;
            cucumber.IsMoving = false;
        }
        
        foreach (var cucumber in SouthCucumbers) {
            if ((cucumber.MyPosition.Item1 + 1 >= SeaCucumbers.GetLength(0) &&
                SeaCucumbers[0, cucumber.MyPosition.Item2] is null) ||
                (cucumber.MyPosition.Item1 + 1 < SeaCucumbers.GetLength(0) &&
                SeaCucumbers[cucumber.MyPosition.Item1 + 1, cucumber.MyPosition.Item2] is null)) {
                cucumber.IsMoving = true;
                moved = true;
            }
        }
        
        foreach (var cucumber in SouthCucumbers) {
            if (!cucumber.IsMoving) continue;
            var oldPosition = cucumber.MyPosition;
            this[cucumber.MyPosition]!.MyPosition = (cucumber.MyPosition.Item1 + 1 >= SeaCucumbers.GetLength(0)
                    ? 0
                    : cucumber.MyPosition.Item1 + 1,
                cucumber.MyPosition.Item2);
            this[cucumber.MyPosition] = cucumber;
            this[oldPosition] = null;
            cucumber.IsMoving = false;
        }

        return moved;
    }

    public override string ToString() {
        string endString = "";
        for (int i = 0; i < SeaCucumbers.GetLength(0); i++) {
            for (int j = 0; j < SeaCucumbers.GetLength(1); j++) {
                endString += SeaCucumbers[i, j] is null ? "." :
                    SeaCucumbers[i, j]!.MyDirection == SeaCucumber.Direction.East ? ">" : "v";
            }

            endString += "\n";
        }

        return endString;
    }
}