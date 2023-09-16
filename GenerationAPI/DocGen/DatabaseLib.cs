using System.Data.SQLite;

namespace JSOT;

internal static class DatabaseLib {

    private static readonly string AppPath = Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData) + @"\JSTemplates\";
    private static readonly string ConnectionPath = Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData) + @"\JSTemplates\JSOTdb";
    private static readonly string ConnectionString = "Data Source=" + Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData) + @"\JSTemplates\JSOTdb";
    private static SQLiteConnection db = new SQLiteConnection(ConnectionString);


    static DatabaseLib() {

        if (!System.IO.Directory.Exists(AppPath))
        {
            System.IO.Directory.CreateDirectory(AppPath);
        }

        if (!System.IO.File.Exists(ConnectionPath))
        {

            SQLiteConnection.CreateFile(ConnectionPath);

        }

    }

    // Retrieves the data from a table,
    // not formatted in any way
    internal static SQLiteDataReader getData(Tables table) {

        db.Open();

        var command = db.CreateCommand();

        command.CommandText = $@"
            SELECT *
            FROM [{table}];";

        return command.ExecuteReader(System.Data.CommandBehavior.CloseConnection);

    }

    // Given some unformatted table data, it prints the data
    // in a formatted table
    internal static void printTable(Tables table, SQLiteDataReader reader) {

        if(table == Tables.Assessors) {

            printAssessors(reader);

        } else if(table == Tables.Assessments) {

            Console.WriteLine("Printing assessment table");

        } else if(table == Tables.Clients) {

            Console.WriteLine("Printing client table");

        }

        reader.Close();

    }

    private static void printAssessors(SQLiteDataReader reader) {

        Console.WriteLine("| {0, -12} | {1, -12} | {2, -15} |", "First Name", "Last Name", "Registration ID");
        Console.WriteLine("|{0}|", new String('*', 47));
        
        while(reader.Read()) {

            Console.WriteLine("| {0, -12} | {1, -12} | {2, -15} |",
                    reader["FirstName"], reader["LastName"], reader["RegistrationID"]);

            Console.WriteLine("|{0}|", new String('-', 47));

        }

    }

}
