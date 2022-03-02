// See https://aka.ms/new-console-template for more information

Console.WriteLine("Hello, World!");

using var srs = new StreamReader("../../../input.txt");
//Part1(srs);
Part2(srs);

void Part1(StreamReader sr) {
   var (elements, insertionRules, activePairs) = ReadInput(sr);

    for(int i = 0; i < 10; i++) {
        activePairs = NextReaction(elements, activePairs, insertionRules);
    }

    PrintElementCount(elements);

    long polymerScore = ScorePolymer(elements);

    Console.WriteLine($"The score of the polymer is {polymerScore}");
}

 void Part2(StreamReader sr) {
   var (elements, insertionRules, activePairs) = ReadInput(sr);

    for(int i = 0; i < 40; i++) {
        activePairs = NextReaction(elements, activePairs, insertionRules);
    }

    PrintElementCount(elements);

    long polymerScore = ScorePolymer(elements);

    Console.WriteLine($"The score of the polymer is {polymerScore}");
}

 (Dictionary<char, long>, Dictionary<string,char>, Dictionary<string, long>) ReadInput(StreamReader sr) {
    string polymer = sr.ReadLine();
    sr.ReadLine(); //Trash empty string

    Dictionary<char, long> elements = new Dictionary<char, long>();

    foreach(char ch in polymer) {
        if(elements.ContainsKey(ch)) elements[ch]++;
        else elements.Add(ch, 1);
    }

    Dictionary<string,char> insertionRules = new Dictionary<string, char>();

    string line = sr.ReadLine();
    while(line != null) {
        string[] rule = line.Split(" -> ");
        insertionRules.Add(rule[0], char.Parse(rule[1]));

        line = sr.ReadLine();
    }

    Dictionary<string, long> activePairs = new Dictionary<string, long>();
    for(int i = 1; i < polymer.Length; i++) {
        string pair = polymer.Substring(i-1, 2);
        if(insertionRules.ContainsKey(pair)) {
            if(activePairs.ContainsKey(pair)) activePairs[pair]++;
            else activePairs.Add(pair, 1);
        }
    }

    return (elements, insertionRules, activePairs);
}

 Dictionary<string, long> NextReaction(Dictionary<char, long> elements, Dictionary<string, long> activePairs, Dictionary<string, char> insertionRules) {
    Dictionary<string, long> nextActivePairs = new Dictionary<string, long>();

    foreach(string pair in activePairs.Keys) {
        char newElement = insertionRules[pair];

        string newPair1 = string.Concat(pair[0], newElement.ToString());
        if(insertionRules.ContainsKey(newPair1)) {
            if(nextActivePairs.ContainsKey(newPair1)) nextActivePairs[newPair1] += activePairs[pair];
            else nextActivePairs.Add(newPair1, activePairs[pair]);
        }

        string newPair2 = string.Concat(newElement.ToString(), pair[1]);
        if(insertionRules.ContainsKey(newPair2)) {
            if(nextActivePairs.ContainsKey(newPair2)) nextActivePairs[newPair2] += activePairs[pair];
            else nextActivePairs.Add(newPair2, activePairs[pair]);
        }

        if(elements.ContainsKey(newElement)) elements[newElement] += activePairs[pair];
        else elements.Add(newElement, activePairs[pair]);
    }

    return nextActivePairs;
}

 void PrintElementCount(Dictionary<char, long> elements) {
    foreach(char element in elements.Keys) {
        Console.WriteLine($"{element} occurs {elements[element]} times");
    }
}

 long ScorePolymer(Dictionary<char, long> elements) {
    long pairMax = long.MinValue;
    long pairMin = long.MaxValue;

    foreach(long value in elements.Values) {
        pairMax = Math.Max(pairMax, value);
        pairMin = Math.Min(pairMin, value);
    }

    return pairMax - pairMin;
}