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

    //for testing
    public Paragraph? GetParagraph(string str)
    {
        foreach (Paragraph para in Body.Descendants<Paragraph>())
        {
            if (para.InnerText.Contains(str))
            {
                return para;
            }
        }

        return null;
    }

    public void SearchAndReplaceText(string pattern, string replacementStr)
    {
        
        Regex matcher = new Regex(pattern);
                
        foreach (Paragraph para in Body!.Descendants<Paragraph>())
        {
            
            if (!matcher.IsMatch(para.InnerText))
            {
                continue;
            }

            if (para.Descendants<Text>().Count() > 1)
            {
                IsolatePatternInParagraph(para, pattern);
            }
            
            foreach (Text text in para.Descendants<Text>())
            {
                if (matcher.IsMatch(text.Text))
                {
                    text.Text = matcher.Replace(text.Text, replacementStr);
                    text.Space = SpaceProcessingModeValues.Preserve;
                    
                }
                
            }

        } 
        
        Doc.Save();
        
    }

    private void IsolatePatternInParagraph(Paragraph para, string pattern)
    {

        /*
        //to catch debugger.
        if (para.InnerText.Contains("<ASSESSOR QUALIFICATIONS>"))
        {
            int i = 0;
        } */
        
        List<Text> textElements = para.Descendants<Text>().ToList();

        List<string> textTexts = new List<string>();

        foreach (Text text in textElements)
        {
            textTexts.Add(text.Text);
        }

        List<int> indices = IndexPositionsInStrList(textTexts);
        
        Regex matcher = new Regex(pattern);
        MatchCollection matches = matcher.Matches(para.InnerText);

        for (int i = 0; i < matches.Count; i++)
        {
            Match match = matches.ElementAt(i);

            int matchStartsInText = WhatPositionIsIndexIn(indices, match.Index);
            Run matchStartsInRun = (Run)textElements.ElementAt(matchStartsInText).Parent;
            int matchEndsInText = WhatPositionIsIndexIn(indices, match.Index + match.Value.Length - 1);

            if (matchStartsInText == matchEndsInText)
            { 
                continue; //match is over a single text element already.
            }

            #region CreateRunWithMatch
            
            Run run = new Run();
            RunProperties propertiesToMatch = matchStartsInRun.RunProperties;
            if (propertiesToMatch != null)
            {
                run.AppendChild((RunProperties)propertiesToMatch.CloneNode(true));
            }
            run.AppendChild(new Text(match.Value));
            
            #endregion

            #region RemoveMatchFromOriginalParapgraph

            int matchStartsAtIndex = match.Index - indices.ElementAt(matchStartsInText);
            int matchEndsAtIndex = match.Index + match.Length - 1 - indices.ElementAt(matchEndsInText);
            // int matchEndsAtIndex = matchStartsAtIndex + match.Length - indices.ElementAt(matchEndsInText);

            for (int j = matchStartsInText; j <= matchEndsInText; j++)
            {

                Text text = textElements.ElementAt(j);

                if (j == matchStartsInText)
                {
                    text.Text = text.Text.Remove(matchStartsAtIndex);
                    continue;
                }
                
                if (j == matchEndsInText)
                {
                    text.Text = text.Text.Remove(0, matchEndsAtIndex + 1);
                    continue;
                }

                //if the text contained only the tag and absolutely nothing else..
                text.Remove();
                    
            }
            
            #endregion
            
            #region InsertNewRunInParagraph

            matchStartsInRun.InsertAfterSelf(run);

            #endregion

        }

    }

  
    /*
    //almost works, but discards elements that can be between text elements. Redo needed.
    public void FindTagsAndPushToNewRun(string pattern)
    {

        Regex matcher = new Regex(pattern);
        
        foreach (var para in Body!.Descendants<Paragraph>())
        {

            var matches = matcher.Matches(para.InnerText);
            foreach (Match match in matches) 
            {
                // just to catch debugger
                  if (match.Value.Contains("DOL")) //CHECKOUT TABCHARS
                  {
                      int i = 0;
                  }

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
        
    } */

    //this shouldn't be used! It discards elements that can go between
    //text elements, such as a tab.
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

    public List<int> IndexPositionsInStrList(List<string> texts)
    {
        
        List<int> indices = new List<int> { 0 };

        for (int i = 0; i < texts.Count - 1; i++)
        {
            indices.Add(indices[i] + texts[i].Length);
        }
        
        return indices;
        
    }

    //This func assumes index IS in a valid index of indices.
    //It wont work properly if it isnt.
    public int WhatPositionIsIndexIn(List<int> indices, int index)
    {

        for (int i = 0; i < indices.Count; i++)
        {
            if (index < indices.ElementAt(i))
            {
                return i - 1;
            }
        }
        
        return indices.Count - 1;
        
    }

    public void Dispose()
    {
        Doc.Dispose();
    }
    
    
}
