// See https://aka.ms/new-console-template for more information

Console.WriteLine("Hello, World!");

using var sr = new StreamReader("../../../input.txt");
//Part1(sr);
Part2(sr);

void Part1(StreamReader sr) {
    Cave start = CreateCaveSystem(sr);

    List<string> paths = new List<string>();
    MapCaves(start, "", paths, false);

    Console.WriteLine($"There are {paths.Count} paths through the caves");
}

void Part2(StreamReader sr) {
    Cave start = CreateCaveSystem(sr);

    List<string> paths = new List<string>();
    MapCaves(start, "", paths, true);

    Console.WriteLine($"There are {paths.Count} paths through the caves");
}

Cave CreateCaveSystem(StreamReader sr) {
    string line = sr.ReadLine();
    List<Cave> caves = new List<Cave>();
    List<string> mappedCaves = new List<string>();
    Cave root = new Cave("error");

    while(line != null) {
        string[] connection = line.Split("-");

        Cave cave1;
        Cave cave2;

        if(mappedCaves.Contains(connection[0])) {
            cave1 = caves[mappedCaves.IndexOf(connection[0])];
        } else {
            cave1 = new Cave(connection[0]);
            caves.Add(cave1);
            mappedCaves.Add(connection[0]);
        }
        
        if(mappedCaves.Contains(connection[1])) {
            cave2 = caves[mappedCaves.IndexOf(connection[1])];
        } else {
            cave2 = new Cave(connection[1]);
            caves.Add(cave2);
            mappedCaves.Add(connection[1]);
        }

        cave1.addConnection(cave2);

        if(cave1.name == "start") root = cave1;
        else if(cave2.name == "start") root = cave2;

        line = sr.ReadLine();
    }

    return root;
}

void MapCaves(Cave root, string currPath, List<string> paths, bool cheatLeft) {
    if(root.name == "end") {
        currPath += "end";
        paths.Add(currPath);
        Console.WriteLine(currPath);
        return;
    }

    foreach(Cave cave in root.connections) {
        if(cave.visit()) {
            MapCaves(cave, currPath + $"{root.name},", paths, cheatLeft);
            cave.visitable = true;
        } else if(cave.name != "start" && cheatLeft) {
            MapCaves(cave, currPath + $"{root.name},", paths, false);
        }
    }
}

class Cave {
    public bool big;
    public string name;
    public List<Cave> connections;
    public bool visitable;

    public Cave(string name) {
        this.name = name;
        this.big = Char.IsUpper(name[0]);
        this.visitable = (name == "start") 
            ? false 
            : true;
        this.connections = new List<Cave>();
    }

    public void addConnection(Cave cave) {
        if(!connections.Contains(cave)) connections.Add(cave);
        if(!cave.connections.Contains(this)) cave.connections.Add(this);
    }

    //Returns true if visited, false if failed. Also sets visitability if small cave.
    public bool visit() {
        if(big) {
            return true;
        } else if(visitable) {
            visitable = false;
            return true;          
        } else {
            return false;
        }
    }

    public bool isEnd() {
        return name == "End";
    }

}