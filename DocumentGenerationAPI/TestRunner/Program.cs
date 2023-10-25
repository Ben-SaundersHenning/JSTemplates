using System.Net.Mime;
using System.Text;
using System.Text.RegularExpressions;
using DocProcessor;
using DocumentFormat.OpenXml;
using DocumentFormat.OpenXml.Packaging;
using DocumentFormat.OpenXml.Wordprocessing;
using Document = DocProcessor.Document;
using DocumentType = DocProcessor.DocumentType;

string testPath = "/home/ben/projects/JSTG/DocumentGenerationAPI/TESTING_FILES/AC_blah.docx";
/*using (Document doc = new Document(testPath, DocumentType.ExistingDocument))
{ 
    //doc.CombineAllRuns();
    doc.SearchAndReplaceTextByRegex(@"<[\w _-]{3,}>", "Blah");
    //doc.SearchAndReplaceTest(@"<[\w _-]{3,}>", "Blah");
}*/


string[] parts = new string[] {"<", "ASSESSOR>sdlkfj", " sdlfkj ", "sdlkfjs<BEN>"};
List<int> startIndices = new List<int>();
startIndices.Add(0);

//TODO: need a list of the indices of each part in parts.
//It will be references in order to determine which part the 
//match(es) end in.
foreach(var part in parts)
{
    startIndices.Add(part.Length + startIndices[startIndices.Count - 1]);
}



string regexPattern = @"<[\w _-]{3,}>";

Regex regexMatcher = new Regex(regexPattern);

//build the paragraphs inner text
StringBuilder builder = new StringBuilder("");
foreach (string part in parts)
{
    builder.Append(part);
}

//matches will have 0 enumerable elements if there are no matches.
var matches = regexMatcher.Matches(builder.ToString());
foreach (Match match in matches)
{
    Console.WriteLine($"MATCH: {match.ToString()}");
    Console.WriteLine($"It was found starting at index {match.Index} of the string {builder.ToString()}");

    int partsLength = 0;
    for(int i = 0; i < parts.Length; i++)
    {
        var part = parts[i];
        
       partsLength += part.Length;

       if (match.Index <= partsLength)
       {
           Console.WriteLine("    Match started in the {0} part. Match index = {1}, part len = {2}", i, match.Index, part.Length);
           
           break;
       }

    }
    
    
}