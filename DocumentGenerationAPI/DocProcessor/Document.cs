using System.IO;
using System.Text;
using System.Text.RegularExpressions;
using DocumentFormat.OpenXml;
using DocumentFormat.OpenXml.Packaging;
using DocumentFormat.OpenXml.Wordprocessing;

namespace DocProcessor;

public enum DocumentType
{
    NewDocument,
    ExistingDocument
}

public class Document: IDisposable
{
    
    private WordprocessingDocument Doc { get; set; }
    
    private MainDocumentPart? MainPart { get; set; }
    
    private Body? Body { get; set; }
    
    
    public Document(string path, DocumentType type)
    {
        if (type == DocumentType.ExistingDocument)
        {
            Doc = WordprocessingDocument.Open(path, true);
            MainPart = Doc.MainDocumentPart;
            Body = MainPart.Document.Body;
        }
        else 
        {
            Doc = WordprocessingDocument.Create(path, WordprocessingDocumentType.Document);
            CreateDocument();
        }

    }
    
    public Document(string path)
    {
        Doc = WordprocessingDocument.Open(path, true);
        Body = Doc.MainDocumentPart.Document.Body;
    }
    
    private void CreateDocument()
    {
        // Add a main document part. 
        MainPart = Doc.AddMainDocumentPart();

        // Create the document structure and add some text.
        MainPart.Document = new DocumentFormat.OpenXml.Wordprocessing.Document();
        Body = MainPart.Document.AppendChild(new Body());
        
    }
    
    
    public void InsertText(string newText)
    {
        Paragraph paragraph = Body.AppendChild(new Paragraph());
        Run run = paragraph.AppendChild(new Run());
        run.AppendChild(new Text(newText));
        Doc.Save();
    }

    public void CombineAllRuns()
    {
        foreach (var para in Body.Descendants<Paragraph>())
        {
            StringBuilder content = new StringBuilder("");
            foreach (var run in para.Elements<Run>())
            {
                foreach (var text in run.Elements<Text>())
                {
                    Console.WriteLine(text.Text);
                    if (text.Text.Contains("QUALIFICATIONS"))
                    {
                        Console.WriteLine("Here");
                    }
                    content.Append(text.Text);
                    text.Remove();
                }
                run.RemoveAllChildren<Text>();
            }
            para.RemoveAllChildren<Run>();

            Run newRun = new Run();
            newRun.AppendChild(new Text(content.ToString()));
            para.AppendChild<Run>(newRun);
        }

            Doc.Save();

    }

    public void SearchAndReplaceTextByRegex(string pattern, string newText)
    {

        foreach (var para in Body.Descendants<Paragraph>())
        {

            foreach (var run in para.Elements<Run>())
            {
                foreach (var text in run.Elements<Text>())
                {
                    if (Regex.IsMatch(text.Text, pattern, RegexOptions.IgnoreCase))
                    {
                        text.Text = Regex.Replace(text.Text, pattern, newText);
                    }
                }
            }

        }

        Doc.Save();

    }

    public void SearchAndReplaceTest(string pattern, string newText)
    {
        foreach (var para in Body.Descendants<Paragraph>())
        {
            //the run, or potential runs, have a matching tag.
            if (Regex.IsMatch(para.InnerText, pattern, RegexOptions.IgnoreCase))
            {
                if (para.Elements<Run>().Count() == 1)
                {
                    continue; //tag has to be contained in the 1 run.
                }
                else
                {
                    /*
                     * TODO: Have to find out which of the runs contain
                     *       the split tag. For the ones that do, remove only
                     *       their runs, and then combine them into a single run.
                     *       the run should have the default formatting for all the
                     *      replacements.
                     */
                    //tag is contained over multiple runs
                    StringBuilder content = new StringBuilder("");
                    foreach (var run in para.Elements<Run>())
                    {
                        foreach (var text in run.Elements<Text>())
                        {
                            content.Append(text.Text);
                        }
                        
                    }
                    para.RemoveAllChildren<Run>();
                    Run newRun = new Run();
                    newRun.AppendChild(new Text(content.ToString()));
                    para.AppendChild(newRun);
                }
                
            }

        }

        Doc.Save();
        
    }

    public void Dispose()
    {
        Doc.Dispose();
    }
    
    
}
