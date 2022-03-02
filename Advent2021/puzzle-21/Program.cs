
using System.Diagnostics;

using var sr = new StreamReader("../../../input.txt");

int player1Start = int.Parse(sr.ReadLine()![^1..]);
int player2Start = int.Parse(sr.ReadLine()![^1..]);

Stopwatch stopwatch = new Stopwatch();
stopwatch.Start();
Console.WriteLine(Fibonacci(42));
Console.WriteLine($"Time Taken: {stopwatch.ElapsedMilliseconds}ms");

int Fibonacci(int n) {
    return n is 0 or 1 ? 1 : Fibonacci(n - 1) + Fibonacci(n - 2);
}

/*Player player1 = new Player {
    Place = int.Parse(player1Start[^1..])
};

Player player2 = new Player {
    Place = int.Parse(player2Start[^1..])
};

int[] vals = new int[3];
for (int currentDiceSpot = 1;;) {
    for (int i = 0; i < 3; i++) {
        vals[i] = currentDiceSpot;
        currentDiceSpot = currentDiceSpot % 100 + 1;
    }

    player1.DieRolled += 3;
    player1.Place = (player1.Place + vals[0] + vals[1] + vals[2] - 1) % 10 + 1;
    player1.Score += player1.Place;
    Console.WriteLine($"Player 1 rolls {vals[0]}+{vals[1]}+{vals[2]} and lands on {player1.Place} for a total score of {player1.Score}.");
    if (player1.Score >= 21) {
        int amount = player1.DieRolled + player2.DieRolled;
        int value = amount * player2.Score;
        Console.WriteLine(value);
        break;
    }
    for (int i = 0; i < 3; i++) {
        vals[i] = currentDiceSpot;
        currentDiceSpot = currentDiceSpot % 100 + 1;
    }

    player2.DieRolled += 3;
    player2.Place = (player2.Place + vals[0] + vals[1] + vals[2] - 1) % 10 + 1;
    player2.Score += player2.Place;
    Console.WriteLine($"Player 2 rolls {vals[0]}+{vals[1]}+{vals[2]} and lands on {player2.Place} for a total score of {player2.Score}.");
    if (player2.Score >= 21) {
        int amount = player1.DieRolled + player2.DieRolled;
        int value = amount * player1.Score;
        Console.WriteLine(value);
        break;
    }
    
}


internal class Player {
    public int Place { get; set; }
    public int Score { get; set; }
    public int DieRolled { get; set; }
}*/