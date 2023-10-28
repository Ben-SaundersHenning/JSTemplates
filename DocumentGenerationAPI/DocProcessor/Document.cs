using System.IO;
using System.Text;
using System.Text.RegularExpressions;
using System.Xml.Serialization;
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
        
        foreach (var para in Body!.Descendants<Paragraph>())
        {

            var matches = matcher.Matches(para.InnerText);
            foreach (Match match in matches) 
            {
                // just to catch debugger
                 // if (match.Value.Contains("ASSESSOR QUALIFICATIONS"))
                 // {
                 //     int i = 0;
                 // }

                IEnumerable<Run> runs = para.Elements<Run>().ToList();
                
                if (runs.Count() == 1) //Match exists over the 1 run.
                {
                    Run run = runs.ElementAt(0);
                    Run newRun = CreateCopyOfRunWithSingleTextElement(run);
                    run.InsertAfterSelf(newRun);
                    run.Remove();
                }
                else //tag is contained over more than 1 run
                {

                    List<string> paragraphsRuns = new List<string>();

                    foreach (var run in runs)
                    {
                        paragraphsRuns.Add(run.InnerText);
                    }
                    
                    List<int> paragraphRunIndices = IndexRunsOfParagraphText(paragraphsRuns);

                    int startRun = DetermineWhatRunIndexIsIn(paragraphRunIndices, match.Index) - 1;
                    int endRun = DetermineWhatRunIndexIsIn(paragraphRunIndices, match.Index + match.Length - 1) - 1;

                    var runWhereMatchStarted = para.Elements<Run>().ElementAt(startRun);
                    var runWhereMatchEnded = para.Elements<Run>().ElementAt(endRun);

                    /*
                    
                    //Create the new run, with the properties
                    //of the (atleast) first char.
                    Run newRun = new Run();
                    
                    //try and copy the properties
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
                    
                    */
                    
                    /*
                     * -- Scenario 1, V1 -> Ideal situation, the tag is prepped. ---
                     * <r>
                     *  <t>
                     *      TAG HERE
                     *  </t>
                     * </r>
                     *
                     * -- Scenario 1, V2 ---
                     * <r>
                     *  <t>
                     *      TAG
                     *  </t>
                     *  <t>
                     *      HERE 
                     *  </t>
                     * </r>
                     *     
                     * -- Scenario 2, V1 ---
                     * <r>
                     *  <t>
                     *      TAG
                     *  </t>
                     * </r>
                     * <r>
                     *  <t>
                     *      HERE 
                     *  </t>
                     * </r>
                     * 
                     * -- Scenario 2, V2 ---
                     * <r>
                     *  <t>
                     *      T
                     *  </t>
                     * </r>
                     * <r>
                     *  <t>
                     *      AG
                     *  </t>
                     * </r>
                     * <r>
                     *  <t>
                     *      HERE 
                     *  </t>
                     * </r>
                     * 
                     */
                    
                    if (startRun == endRun) //HANDLE SCENARIO 1
                    {

                        var textElements = para.Elements<Run>().ElementAt(startRun).Descendants<Text>().ToList();

                        if (textElements.Count != 1) //HANDLE V2 
                        {
                            Run newRun = CreateCopyOfRunWithSingleTextElement(runWhereMatchEnded);
                            para.Elements<Run>().ElementAt(endRun).InsertAfterSelf(newRun);
                            para.Elements<Run>().ElementAt(endRun).Remove();
                        }
                        
                    }
                    else //HANDLE SCENARIO 2
                    {

                        //standardize runs to have 1 text element
                        int runIndex = startRun;
                        while (runIndex != endRun)
                        {
                            Run curRun = runs.ElementAt(runIndex);
                            Run newRun = CreateCopyOfRunWithSingleTextElement(curRun);
                            curRun.InsertAfterSelf(newRun);
                            curRun.Remove();
                            runIndex++;
                        }

                        int numCharsRemoved = 0;

                        Text startRunText = runWhereMatchStarted.Descendants<Text>().ToList().ElementAt(0);
                        
                        //If it started in this run and doesnt end in this run, the tag has to go
                        //to the end of the run.
                        numCharsRemoved = startRunText.Text.Length - match.Index;
                        startRunText.Text = startRunText.Text.Remove(match.Index, numCharsRemoved);
                        var newStartRun = runWhereMatchStarted.CloneNode(true);
                        para.Elements<Run>().ElementAt(startRun).InsertAfterSelf(newStartRun);
                        para.Elements<Run>().ElementAt(startRun).Remove();

                        while (numCharsRemoved < match.Length)
                        {
                            for (int rIndex = 1; rIndex <= endRun; rIndex++)
                            {
                                Run run = runs.ElementAt(rIndex);
                                Text text = run.Descendants<Text>().ToList().ElementAt(0);
                                int textLen = text.Text.Length;
                                int textRemovalIndex = 0;

                                for (int i = 0; i < textLen; i++)
                                {
                                    if (numCharsRemoved >= match.Length)
                                    {
                                        goto LoopEnd;
                                    }

                                    text.Text = text.Text.Remove(textRemovalIndex, 1);
                                    //textRemovalIndex++;
                                    numCharsRemoved++;
                                }

                                if (rIndex < endRun)
                                {
                                    //every char in the run has been removed
                                    run.Remove();
                                }
                                
                            }
                        }
LoopEnd:
                        Run newRunWithMatch = new Run();
                        //try and copy the properties
                        var runProperties = runWhereMatchStarted.Elements<RunProperties>().FirstOrDefault();
                        if (runProperties != null)
                        {
                            newRunWithMatch.AppendChild((RunProperties)runProperties.CloneNode(true));
                        }
                        
                        newRunWithMatch.AppendChild(new Text(match.Value));
                        para.Elements<Run>().ElementAt(endRun).InsertAfterSelf(newRunWithMatch);
                        para.Elements<Run>().ElementAt(endRun).Remove();
                    
                    }

                }
                
            }
                
        }

        Doc.Save();
        
    }

    private Run CreateCopyOfRunWithSingleTextElement(Run run)
    {
            string runText = run.InnerText;
            
            //Create the new run, with the properties
            //of the original run.
            Run newRun = new Run();
            var runProperties = run.Elements<RunProperties>().FirstOrDefault();

            if (runProperties != null)
            {
                newRun.AppendChild((RunProperties)runProperties.CloneNode(true));
            }
            
            //move all text into a single text element. There is no tPr, so this
            //wont affect formatting.
            newRun.AppendChild(new Text(runText));

            return newRun;
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
