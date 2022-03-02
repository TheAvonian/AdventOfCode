// See https://aka.ms/new-console-template for more information

Console.WriteLine("Hello, World!");

using var sr = new StreamReader("../../../input.txt");

string line = sr.ReadLine()!;

string xVal = line[(line.IndexOf("x=", StringComparison.Ordinal) + 2)..line.IndexOf(",", StringComparison.Ordinal)];
(int xStart, int xEnd) = (int.Parse(xVal[..xVal.IndexOf("..", StringComparison.Ordinal)]),
    int.Parse(xVal[(xVal.IndexOf("..", StringComparison.Ordinal) + 2)..]));
string yVal = line[(line.IndexOf("y=", StringComparison.Ordinal) + 2)..];
(int yStart, int yEnd) = (int.Parse(yVal[..yVal.IndexOf("..", StringComparison.Ordinal)]),
    int.Parse(yVal[(yVal.IndexOf("..", StringComparison.Ordinal) + 2)..]));

// keep going until x is 0 and if past point then lower x velocity
// while xVel * 2 >= xStart && xVel * 2 <= xEnd

Map map = new Map((xStart, xEnd), (yStart, yEnd));

Console.WriteLine(map.Simulate());
internal class Ball {
    public (int, int) Position { get; set; }
    public (int, int) Velocity { get; set; }
    public int HeighestPoint { get; set; }

    public Ball(int xVel, int yVel) {
        HeighestPoint = int.MinValue;
        Velocity = (xVel, yVel);
    }

    public void Step() {
        Position = (Position.Item1 + Velocity.Item1, Position.Item2 + Velocity.Item2);
        Velocity = (Velocity.Item1 - Math.Sign(Velocity.Item1), Velocity.Item2 - 1);
        if (Position.Item2 > HeighestPoint) {
            HeighestPoint = Position.Item2;
        }
    }
}

internal class EndZone {
    public (int, int) XBounds { get; set; }
    public (int, int) YBounds { get; set; }

    public EndZone((int, int) x, (int, int) y) {
        XBounds = x;
        YBounds = y;
    }

    public bool IsInside((int, int) position) {
        (int x, int y) = position;
        return x <= XBounds.Item2 && x >= XBounds.Item1 && 
               y <= YBounds.Item2 && y >= YBounds.Item1;
    }
}

internal class Map {
    public Ball Ball { get; set; }
    public EndZone EndZone { get; set; }
    public Map((int, int) x, (int, int) y, int xVel = 0, int yVel = 0) {
        Ball = new Ball(xVel,yVel);
        EndZone = new EndZone(x, y);
    }

    public int Simulate() {
        int heighestPoint = int.MinValue;
        int sum = Summation(1);
        int n = 1;
        while (sum < EndZone.XBounds.Item1) {
            sum = Summation(++n);
        }

        int total = 0;
        for (int x = n; x <= EndZone.XBounds.Item2; x++) {
            for (int y = 5000; y >= EndZone.YBounds.Item1; y--) {
                
                Ball = new Ball(x, y);
                bool isIn = false;
                while (Ball.Position.Item1 <= EndZone.XBounds.Item2 && Ball.Position.Item2 >= EndZone.YBounds.Item1) {
                    Ball.Step();
                    if (EndZone.IsInside(Ball.Position)) {
                        isIn = true;
                    }
                }

                if (isIn) total++;
                if (isIn && Ball.HeighestPoint > heighestPoint) {
                    heighestPoint = Ball.HeighestPoint;
                    Console.WriteLine(heighestPoint);
                }
            }
        }
        Console.WriteLine(total);
        return heighestPoint;
    }

    int Summation(int num) {
        int sum = 0;
        for (int i = num; i > 0; i--) {
            sum += i;
        }

        return sum;
    }

    public override string ToString() {
        string endString = "";
        return endString;
    }
}