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

    public void SearchAndReplaceText(string pattern, Func<string, string> getReplacementString)
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

                foreach (Match match in matcher.Matches(text.Text))
                {
                    text.Text = matcher.Replace(text.Text, getReplacementString(match.Value));
                    text.Space = SpaceProcessingModeValues.Preserve;
                }
                
                // if (matcher.IsMatch(text.Text))
                // {
                //     text.Text = matcher.Replace(text.Text, replacementStr);
                //     text.Space = SpaceProcessingModeValues.Preserve;
                //     
                // }
                
            }

        } 
        
        Doc.Save();
        
    }

    private void IsolatePatternInParagraph(Paragraph para, string pattern)
    {

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

    private List<int> IndexPositionsInStrList(List<string> texts)
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
    private int WhatPositionIsIndexIn(List<int> indices, int index)
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
