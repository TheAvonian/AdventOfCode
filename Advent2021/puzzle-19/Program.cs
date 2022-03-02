// See https://aka.ms/new-console-template for more information

Console.WriteLine("Hello, World!");

using var srs = new StreamReader("../../../input.txt");
//Part1(srs);
Part2(srs);

void Part1(StreamReader sr) {
    List<Scanner> scanners = ReadScannersFromInput(sr);

    MapScanners(scanners);

    //Console.WriteLine("Beacons: ");
    //scanners[0].beacons.ForEach(x=>Console.WriteLine($"{x.x}, {x.y}, {x.z}"));

    Console.WriteLine(scanners[0].beacons.Count);
}

void Part2(StreamReader sr) {
    List<Scanner> scanners = ReadScannersFromInput(sr);

    MapScanners(scanners);

    int largestManhattanDistance = int.MinValue;

    for(int i = 0; i < scanners.Count - 1; i++) {
        for(int j = i; j < scanners.Count; j++) {
            int manhattanDistance = CalculateManhattanDistance(scanners[i], scanners[j]);
            largestManhattanDistance = Math.Max(largestManhattanDistance, manhattanDistance);
        }
    }

    Console.WriteLine(largestManhattanDistance);
}

List<Scanner> ReadScannersFromInput(StreamReader sr) {
    string line = sr.ReadLine();
    List<Scanner> scanners = new List<Scanner>();
    while(line != null) {
        int scannerIndex = int.Parse(line.Split(" ")[2]);
        Scanner scanner = new Scanner();

        line = sr.ReadLine();
        while(line != "" && line != null) {
            string[] beaconLine = line.Split(",");
            Beacon beacon = new Beacon();

            beacon.x = int.Parse(beaconLine[0]);
            beacon.y = int.Parse(beaconLine[1]);
            beacon.z = int.Parse(beaconLine[2]);

            scanner.beacons.Add(beacon);

            line = sr.ReadLine();
        }

        scanners.Add(scanner);

        line = sr.ReadLine();
    }

    return scanners;
}

(int, int ,int) RotateBeacon((int, int, int) position, char dimension, double angle) {
    var (x, y, z) = position;
    int i, j, k;

    switch (dimension) {
        case 'x':
            i = x;
            j = (int)Math.Round((Math.Cos(angle) * y + -Math.Sin(angle) * z));
            k = (int)Math.Round((Math.Sin(angle) * y + Math.Cos(angle) * z));
            break;
        case 'y':
            i = (int)Math.Round((Math.Cos(angle) * x + Math.Sin(angle) * z));
            j = y;
            k = (int)Math.Round((-Math.Sin(angle) * x + Math.Cos(angle) * z));
            break;
        case 'z':
            i = (int)Math.Round((Math.Cos(angle) * x + -Math.Sin(angle) * y));
            j = (int)Math.Round((Math.Sin(angle) * x + Math.Cos(angle) * y));
            k = z;
            break;
        default:
            i = 0;
            j = 0; 
            k = 0;
            break;
    }
    return (i, j, k);
}


Beacon ReflectBeacon(Beacon beacon, double angle, int rotateIndex) {
    (int, int, int) currPos = (beacon.x, beacon.y, beacon.z);
    (int, int, int) newPos = (-1, -1, -1);
        switch (rotateIndex) {
            case 0:
                newPos = RotateBeacon(currPos, 'z', angle);
                break;
            case 1:
                newPos = RotateBeacon(RotateBeacon(currPos, 'x', Math.PI/2), 'z', angle);
                break;
            case 2:
                newPos = RotateBeacon(RotateBeacon(currPos, 'x', Math.PI), 'z', angle);
                break;
            case 3:
                newPos = RotateBeacon(RotateBeacon(currPos, 'x', -Math.PI/2), 'z', angle);
                break;
            case 4:
                newPos = RotateBeacon(RotateBeacon(currPos, 'y', Math.PI/2), 'z', angle);
                break;
            case 5:
                newPos = RotateBeacon(RotateBeacon(currPos, 'y', -Math.PI/2), 'z', angle);
                break;
        }

    return new Beacon(newPos.Item1, newPos.Item2, newPos.Item3);
}

void MapScanners(List<Scanner> scanners) {
    List<Beacon> totalBeacons = new List<Beacon>();
    scanners[0].beacons.ForEach(x=>totalBeacons.Add(x));

    scanners[0].x = 0;
    scanners[0].y = 0;
    scanners[0].z = 0;

    List<Scanner> unmappedScanners = new List<Scanner>();
    scanners.ForEach(x=>unmappedScanners.Add(x));
    unmappedScanners.Remove(scanners[0]);

    while(unmappedScanners.Count > 0) {
        int scannerIndex;
        (double, int) matchRotation = (-1, -1);
        List<(Beacon, Beacon)> sameBeacons = new List<(Beacon, Beacon)>();
        for(scannerIndex = 0; scannerIndex < unmappedScanners.Count; scannerIndex++) {
            foreach(Beacon beacon in scanners[0].beacons) {
                if(scanners[0].beacons.Count - scanners[0].beacons.IndexOf(beacon) < 12) break;

                List<Beacon> sisterBeacons = new List<Beacon>();
                sameBeacons.Clear();

                for (double angle = 0; angle < Math.Tau; angle += Math.PI / 2) {
                    for(int i = 0; i < 6; i++) {
                        sisterBeacons.Clear();
                        unmappedScanners[scannerIndex].beacons.ForEach(x=>sisterBeacons.Add(ReflectBeacon(x, angle, i)));
                        matchRotation = (angle, i);

                        Scanner sisterScanner = new Scanner();
                        sisterScanner.beacons = sisterBeacons;

                        List<(int, int, int)> distances1 = scanners[0].EvaluateDistances(beacon.x, beacon.y, beacon.z);

                        foreach(Beacon sisterBeacon in sisterBeacons) {
                            if(sisterBeacons.Count - sisterBeacons.IndexOf(sisterBeacon) < 12) break;
                            List<(int, int, int)> distances2 = sisterScanner.EvaluateDistances(sisterBeacon.x, sisterBeacon.y, sisterBeacon.z);

                            int matches = 0;

                            for(int j = 0; j < distances1.Count; j++) {
                                if(distances2.Contains(distances1[j])) {
                                    matches++;
                                    (Beacon, Beacon) match = (scanners[0].beacons[j], unmappedScanners[scannerIndex].beacons[distances2.IndexOf(distances1[j])]);
                                    sameBeacons.Add(match);
                                }
                            }

                            if(matches >= 12) {
                                goto endLoop;
                            } else {
                                matches = 0;
                                sameBeacons.Clear();
                            }
                        }
                    }
                }
            }
        }
        endLoop:

        if(sameBeacons.Count >= 12) {
            Beacon beaconMatch = ReflectBeacon(sameBeacons[0].Item2, matchRotation.Item1, matchRotation.Item2);

            Scanner currScanner = unmappedScanners[scannerIndex];

            currScanner.x = sameBeacons[0].Item1.x - beaconMatch.x;
            currScanner.y = sameBeacons[0].Item1.y - beaconMatch.y;
            currScanner.z = sameBeacons[0].Item1.z - beaconMatch.z;

            //Console.WriteLine($"{currScanner.x}, {currScanner.y}, {currScanner.z}");

            foreach(Beacon beacon in currScanner.beacons) {
                Beacon tempBeacon = ReflectBeacon(beacon, matchRotation.Item1, matchRotation.Item2);
                tempBeacon = new Beacon(currScanner.x + tempBeacon.x, currScanner.y + tempBeacon.y, currScanner.z + tempBeacon.z);
                if(!totalBeacons.Contains(tempBeacon)) totalBeacons.Add(tempBeacon);
            }

            scanners[0].beacons = totalBeacons.ToList();

            unmappedScanners.Remove(unmappedScanners[scannerIndex]);
        } else {
            Console.WriteLine("No scanner found. Code broke :(");
        }
    }
}

int CalculateManhattanDistance(Scanner scanner1, Scanner scanner2) {
    return Math.Abs(scanner1.x - scanner2.x) + Math.Abs(scanner1.y - scanner2.y) + Math.Abs(scanner1.z - scanner2.z);
}

class Scanner {
    public int x, y, z;
    public List<Beacon> beacons;

    public Scanner() {
        beacons = new List<Beacon>();
    }

    public List<(int, int , int)> EvaluateDistances(int x, int y, int z) {
        List<(int, int, int)> distances = new List<(int, int, int)>();
        
        foreach(Beacon beacon in beacons) {
            distances.Add((x - beacon.x, y - beacon.y, z - beacon.z));
        }

        return distances;
    }
}

class Beacon : IComparable {
    public int x, y, z;

    public Beacon(int x, int y, int z) {
        this.x = x;
        this.y = y;
        this.z = z;
    }

    public Beacon() {
        this.x = 0;
        this.y = 0;
        this.z = 0;
    }

    public int CompareTo(object obj) {
        if(this.Equals(obj)) return 0;

        return this.x.CompareTo(((Beacon)obj).x);
    }

    public override bool Equals(object obj)
    {
        return this.x == ((Beacon)obj).x && this.y == ((Beacon)obj).y && this.z == ((Beacon)obj).z;
    }

    public override int GetHashCode()
    {
        return (x * y - z).GetHashCode();
    }
}