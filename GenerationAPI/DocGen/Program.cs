namespace JSOT;

internal class Program {

    internal static void Main(string[] args) {

        CLIInterface CLI = new CLIInterface();

        do
        {

            CLI.ShowMenu();
            // Thread.Sleep(1500);

        } while (true);

    }
    
    // function to get the full month name
    static string getFullName(int month)
    {
        DateTime date = new DateTime(2020, month, 1);

        return date.ToString("MMMM");
    }

}

internal class CLIInterface {

    private DocGen docs;

    internal CLIInterface() {

        docs = new DocGen();

    }

    internal void ShowMenu() {

        // Console.Clear();
        Console.WriteLine("0 - Print Assessors");
        Console.WriteLine("1 - Test DocGen");
        Console.WriteLine("Other - Hello, World");
        Console.Write("--> ");

        switch(Convert.ToInt32(Console.ReadLine())) {

            case 0:

                DatabaseLib.printTable(Tables.Assessors, DatabaseLib.getData(Tables.Assessors));
                break;

            case 1:

                docs.FindTags();
                docs.getData();
                docs.GenerateDocument();
                break;

            default:

                Console.WriteLine("Hello, World!");
                string appPath = Environment.GetFolderPath(Environment.SpecialFolder.ProgramFiles) + @"\JSTemplates\";
                Console.WriteLine(Environment.GetFolderPath(Environment.SpecialFolder.ProgramFiles));
                Console.WriteLine(Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData));
                Console.WriteLine(appPath);
                break;

        }

    }

}
