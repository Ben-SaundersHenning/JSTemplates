using Xceed.Document.NET;
using Xceed.Words.NET;

namespace JSOT;

internal class DocGen {

    private List<string> inputs = new List<string>();
    private Dictionary<string, string> outputs = new Dictionary<string, string>();

    internal DocGen() {
    }

    internal void testMethod() {

        Console.WriteLine("This is in DocGen");

    }

    internal void getData()
    {
        foreach (string input in inputs)
        {
            Console.Write("{0} --> ", input);
            outputs.Add(input, Console.ReadLine());
        }
    }

    internal void FindTags()
    {

        //using (var document = DocX.Load(@""))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\NEB.docx"))
        using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\AC.docx"))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\MRB.docx"))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\CAT MRB.docx"))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\CAT.docx"))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\CAT AC.docx"))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\CAT GOSE.docx"))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\AC MRB.docx"))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\CAT AC MRB.docx"))
        {
            foreach (string str in document.FindUniqueByPattern(@"<[\w _-]{3,}>", System.Text.RegularExpressions.RegexOptions.IgnoreCase))
            {
                inputs.Add(str.Substring(1, str.Length - 2));
            }
        }

    }

    internal void GenerateDocument()
    {
        //using (var document = DocX.Load(@""))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\NEB.docx"))
        using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\AC.docx"))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\MRB.docx"))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\CAT MRB.docx"))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\CAT.docx"))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\CAT AC.docx"))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\CAT GOSE.docx"))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\AC MRB.docx"))
        //using (var document = DocX.Load(@"C:\Users\Ben Saunders-Henning\AppData\Roaming\JSTemplates\templates\CAT AC MRB.docx"))
        {
            if (document.FindUniqueByPattern(@"<[\w _-]{3,}>", System.Text.RegularExpressions.RegexOptions.IgnoreCase).Count > 0)
            {
                var replaceTextOptions = new FunctionReplaceTextOptions()
                {
                    FindPattern = "<(.*?)>",
                    RegexMatchHandler = ReplaceFunc,
                    RegExOptions = System.Text.RegularExpressions.RegexOptions.IgnoreCase,
                    NewFormatting = new Formatting() { FontColor = System.Drawing.Color.Black, Size = 12, FontFamily = new Font("Times New Roman") }
                };

                document.ReplaceText(replaceTextOptions);
                document.SaveAs(@"B:\docs\jsot\2023\replaced.docx");
            }
        }
    }

    private string ReplaceFunc(string findStr)
    {
        if(outputs.ContainsKey(findStr))
        {
            return outputs[findStr];
        }

        return "NULL: " + findStr;
    }

}
