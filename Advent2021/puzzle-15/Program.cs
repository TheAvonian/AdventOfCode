
using var srs = new StreamReader("../../../input.txt");
//Part1(srs);
Part2(srs);

void Part1(StreamReader sr) {
    Block[,] map = ReadInput(sr);

    map[0,0].value = 0;

    PriorityQueue<Block, int> paths = new PriorityQueue<Block, int>();
    paths.Enqueue(map[0,0], 0);

    //PrintMap(map, false);

    Console.WriteLine("Finding best path...");

    Dijkstras(map, paths);

    PrintMap(map, true);

    Console.WriteLine($"The best path is valued at {map[map.GetLength(0) - 1, map.GetLength(1) - 1].value}");
}

 void Part2(StreamReader sr) {
    Block[,] map = ReadInput(sr);

    //PrintMap(map, false);

    map = MultiplyMap(map);

    //PrintMap(map, false);

    PriorityQueue<Block, int> paths = new PriorityQueue<Block, int>();
    paths.Enqueue(map[0,0], 0);

    map[0,0].value = 0;

    Dijkstras(map, paths);

    //PrintMap(map, true);

    Console.WriteLine($"The best path is valued at {map[map.GetLength(0) - 1, map.GetLength(1) - 1].value}");
}

 Block[,] ReadInput(StreamReader sr) {
    string line = sr.ReadLine();
    Block[,] map = new Block[line.Length, line.Length];

    for(int i = 0; line != null && i < line.Length; i++) {
        for(int j = 0; j < line.Length; j++) {
            map[i,j] = new Block(int.Parse(line.Substring(j,1)), i, j);
        }
        line = sr.ReadLine();
    }

    return map;
}

 void Dijkstras(Block[,] map, PriorityQueue<Block, int> paths) {
    while(paths.Count > 0) {
        Block node = paths.Dequeue();
        if(node.visited) 
            continue;

        node.visited = true;

        for(int i = -1; i <= 1; i++) {
            for(int j = -1; j <= 1; j++) {
                if(Math.Abs(i) != Math.Abs(j)) {
                    var (x, y) = node.coords;
                    if(x + i >= 0 && x + i < map.GetLength(0) && y + j >= 0 && y + j < map.GetLength(1) && !map[x+i,y+j].visited) {
                        Block nextNode = map[x+i,y+j];
                        nextNode.value = Math.Min(nextNode.value, node.value + nextNode.weight);
                        if(x+i == map.GetLength(0) - 1 && y + j == map.GetLength(1) - 1) 
                            return;
                        paths.Enqueue(nextNode, nextNode.value);
                    }
                }
            }
        }
    }
}

 void PrintMap(Block[,] map, bool drawWeighted) {
    for(int i = 0; i < map.GetLength(0); i++) {
        for(int j = 0; j < map.GetLength(1); j++) {
            if(drawWeighted) Console.Write($"{map[i,j].value} ");
            else Console.Write($"{map[i,j].weight} ");
        }
        Console.WriteLine();
    }
}

 Block[,] MultiplyMap(Block[,] map) {
    Block[,] bigMap = new Block[map.GetLength(0) * 5, map.GetLength(1) * 5];

    for(int i = 0; i < map.GetLength(0); i++) {
        for(int j = 0; j < map.GetLength(1); j++) {
            Block currNode = map[i,j];

            for(int k = 0; k < 5; k++) {
                for(int l = 0; l < 5; l++) {
                    int newValue = (map[i,j].weight + k + l) % 10 + ((map[i,j].weight + k + l) / 10);
                    
                    Block newBlock = new Block(newValue, (map.GetLength(0) * k) + i, (map.GetLength(1) * l) + j);
                    var (x, y) = newBlock.coords;

                    bigMap[x, y] = newBlock;
                }
            }
        }
    }

    return bigMap;
}

class Block : IComparable {
    public int weight;
    public int value;
    public bool visited;
    public (int, int) coords;

    public Block(int weight, int x, int y) {
        this.weight = weight;
        this.value = int.MaxValue;
        this.visited = false;
        this.coords = (x, y);
    }

    public int CompareTo(object obj) {
        return this.value.CompareTo(((Block)obj).value);
    }
}