
Console.WriteLine("Hello, World!");

using var sr = new StreamReader("../../../input.txt");

string input = sr.ReadLine()!;
string binarystring = string.Join(string.Empty,
    input.Select(
        c => Convert.ToString(Convert.ToInt32(c.ToString(), 16), 2).PadLeft(4, '0')
    )
);
Console.WriteLine(binarystring);

string version = binarystring[..3];
string type = binarystring[3..6];
string 