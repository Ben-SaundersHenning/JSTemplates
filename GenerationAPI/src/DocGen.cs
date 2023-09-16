using System.Data.SQLite;
using System.Drawing.Text;
using Xceed.Document.NET;
using Xceed.Words.NET;

namespace JSOT;

internal class DocGen {

    private List<string> inputs = new List<string>();
    private Dictionary<string, string> outputs = new Dictionary<string, string>();

    internal DocGen() {

        inputs.Add("ADJUSTER");
        inputs.Add("INSURANCE COMPANY");
        inputs.Add("CLIENT SALUTATION");
        inputs.Add("CLIENT FIRST");
        inputs.Add("CLIENT LAST");
        inputs.Add("DOB");
        inputs.Add("DOA");
        inputs.Add("DOL");
        inputs.Add("CLAIM NO");
        inputs.Add("AGE");
        inputs.Add("ADDRESS");

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

        using (var document = DocX.Load(@"B:\projects\JSTemplates\templates\jsottemplate.docx"))
        {
            foreach (string str in document.FindUniqueByPattern(@"<[\w _-]{3,}>", System.Text.RegularExpressions.RegexOptions.IgnoreCase))
            {
                Console.WriteLine(str);
            }
        }

    }

    internal void GenerateDocument()
    {
        using (var document = DocX.Load(@"B:\projects\JSTemplates\templates\jsottemplate.docx"))
        {
            if(document.FindUniqueByPattern(@"<[\w _-]{3,}>", System.Text.RegularExpressions.RegexOptions.IgnoreCase).Count > 0)
            {
                var replaceTextOptions = new FunctionReplaceTextOptions()
                {
                    FindPattern = "<(.*?)>",
                    RegexMatchHandler = ReplaceFunc,
                    RegExOptions = System.Text.RegularExpressions.RegexOptions.IgnoreCase,
                    NewFormatting = new Formatting() { Bold = false, FontColor = System.Drawing.Color.Black, Size = 12, FontFamily = new Font("Times New Roman") }
                };

                document.ReplaceText(replaceTextOptions);
                document.SaveAs(@"B:\projects\JSTemplates\templates\replaced.docx");
            }
        }
    }

    private string ReplaceFunc(string findStr)
    {
        if(outputs.ContainsKey(findStr))
        {
            return outputs[findStr];
        }
        return "NULL: NO DATA";
    }

}
