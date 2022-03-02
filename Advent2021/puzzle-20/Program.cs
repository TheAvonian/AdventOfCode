
Console.WriteLine("Hello, World!");

using var sr = new StreamReader("../../../input.txt");

string algorithm = sr.ReadLine()!;
sr.ReadLine();

List<string> lines = new();
string? line;
while ((line = sr.ReadLine()) != null) {
    lines.Add(line);
}

Console.WriteLine("Default: ");
Map map = new Map(lines, algorithm);
Console.WriteLine();
map.EnhanceMap();
Console.WriteLine("Step 1: ");
Console.WriteLine(map);
Console.WriteLine();
int amount = map.EnhanceMap();
Console.WriteLine("Step 2: ");
Console.WriteLine(map);

Console.WriteLine(amount);

class Map {
    public string Algorithm { get; set; }
    public List<List<int>> MapInts { get; set; }

    public Map(IReadOnlyList<string> lines, string algorithm) {
        MapInts = new();
        Algorithm = algorithm;

        for (int i = 0; i < lines.Count; i++) {
            char[] line = lines[i].ToCharArray();
            MapInts.Add(new ());
            for (int j = 0; j < lines[i].Length; j++) {
                MapInts[i].Add(line[j] switch {
                    '.' => 0,
                    '#' => 1,
                    _ => -1
                });
            }
        }
        
        Console.WriteLine(this);
    }

    public int EnhanceMap() {
        var newMap = new List<List<int>>();
        int total = 0;
        for (int i = -1; i <= MapInts.Count; i++) {
            newMap.Add(new());
            for (int j = -1; j <= MapInts[0].Count; j++) {
                newMap[i + 1].Add(Enhance(GetDecimalPixel(i,j)));
                if (newMap[i + 1][j + 1] == 1) total++;
                //Console.Write(newMap[i][j] == 1 ? '#' : '.');
            }

            //Console.WriteLine();
        }

        MapInts = new();
        for (int i = 0; i < newMap.Count; i++) {
            MapInts.Add(new());
            for (int j = 0; j < newMap[i].Count; j++) {
                MapInts[i].Add(newMap[i][j]);
            }
        }
        //Console.WriteLine(this);
        return total;
    }
    
    int GetDecimalPixel(int x, int y) {
        string str = "";
        for (int i = x - 1; i <= x + 1; i++) {
            for (int j = y - 1; j <= y + 1; j++) {
                int val;
                if (i < 0 || i >= MapInts.Count || j >= MapInts[0].Count || j < 0) {
                    val = 0;
                }
                else {
                    //Console.WriteLine(i + " " + j + " " + x + " " + y);
                    val = MapInts[i][j];
                }
                str += val.ToString();
            }
        }
        //Console.WriteLine(str);
        return Convert.ToInt32(str, 2);
    }

    int Enhance(int val) {
        return Algorithm[val] == '#' ? 1 : 0;
    }

    public override string ToString() {
        string endString = "";
        foreach (var line in MapInts) {
            foreach (int val in line) {
                endString += val == 1 ? '#' : '.';
            }

            endString += "\n";
        }

        return endString;
    }
}