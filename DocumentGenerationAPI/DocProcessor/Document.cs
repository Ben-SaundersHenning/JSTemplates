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
                else //tag is contained over more than 1 run
                {
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

    public void FindTagsAndPushToNewRun(string pattern)
    {

        Regex matcher = new Regex(pattern);
        
        foreach (var para in Body.Descendants<Paragraph>())
        {

            var matches = matcher.Matches(para.InnerText);
            foreach (Match match in matches)
            {
                if (match.Value.Contains("ASSESSOR QUALIFICATIONS"))
                {
                    int i = 0;
                }
                if (para.Elements<Run>().Count() == 1)
                {
                    continue; //tag(s) has/have to be contained in the 1 run.
                }
                else //tag is contained over more than 1 run
                {

                    List<string> paragraphsRuns = new List<string>();

                    foreach (var run in para.Elements<Run>())
                    {
                        paragraphsRuns.Add(run.InnerText);
                    }
                    
                    List<int> paragraphRunIndices = IndexRunsOfParagraphText(paragraphsRuns);

                    int startRun = DetermineWhatRunIndexIsIn(paragraphRunIndices, match.Index) - 1;
                    int endRun = DetermineWhatRunIndexIsIn(paragraphRunIndices, match.Index + match.Length - 1) - 1;

                    var runWhereMatchStarted = para.Elements<Run>().ElementAt(startRun);
                    var runWhereMatchEnded = para.Elements<Run>().ElementAt(endRun);

                    //Create the new run, with the properties
                    //of the (atleast) first char.
                    Run newRun = new Run();
                    
                    //try and copy the properties
                    RunProperties? properties = new RunProperties();
                    var orgProperties = runWhereMatchStarted.Elements<RunProperties>().FirstOrDefault().CloneNode(true);
                    if (orgProperties != null)
                    {
                        newRun.AppendChild((RunProperties)orgProperties);
                    }
                    
                    Text newText = new Text(match.Value);
                    newRun.AppendChild(newText);
                    runWhereMatchStarted.InsertAfterSelf(newRun);
                    
                    //NOTE: Right now, the above is not removing the original split text.
                    //So the replacement happens in the middle of the tag.
                    //match is contained in a single run
                    if (startRun == endRun)
                    {
                        //TODO: implement    
                    }
                    else //match is contained over atleast 2 runs
                    {
                        //TODO: implement
                    }

                }
                
            }
                
        }

        Doc.Save();
        
    }

    private List<int> IndexRunsOfParagraphText(List<string> runText)
    {
        List<int> indices = new List<int>();

        for (int i = 0; i < runText.Count; i++)
        {
            int count = 0;
            for (int j = i - 1; j >= 0; j--)
            {
                count += runText[j].Length;
            }
            indices.Add(count);
        } 

        return indices;
    }

    //This function assumes that index is in one of the runs 
    //(or index groups). When this has been called, a match 
    //has already been determined.
    private int DetermineWhatRunIndexIsIn(List<int> indices, int index)
    {
        int indexMatch = 0;
        foreach (int i in indices)
        {
            if (index < i)
            {
                return indexMatch;
                break;

            }

            indexMatch++;
        }

        return indices.Count;
        
    }

    public void Dispose()
    {
        Doc.Dispose();
    }
    
    
}
