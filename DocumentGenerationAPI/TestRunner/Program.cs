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
using (Document doc = new Document(testPath, DocumentType.ExistingDocument))
{ 
    //doc.CombineAllRuns();
    //doc.SearchAndReplaceTest(@"<[\w _-]{3,}>", "Blah");
    doc.FindTagsAndPushToNewRun(@"<[\w _-]{3,}>");
    doc.SearchAndReplaceTextByRegex(@"<[\w _-]{3,}>", "Blah");
}
/*
//need 0, 1, 16, 22

string[] parts = new string[] {"<AAA><BBB><CCC><DDD>", "ASSESSOR>sd<SECOND MATCH>lkfj", "sd<ONE>lfkj", "sdlkfjs<BEN>", "<TEST>", "sdlkfjs"};
List<int> startIndices = new List<int>();
//startIndices.Add(0);

for (int i = 0; i < parts.Length; i++)
{
    int count = 0;
    for (int j = i - 1; j >= 0; j--)
    {
        count += parts[j].Length;
    }
    startIndices.Add(count);
}

string regexPattern = @"<[\w _-]{3,}>";

Regex regexMatcher = new Regex(regexPattern);

//build the paragraphs inner text
StringBuilder builder = new StringBuilder("");
foreach (string part in parts)
{
    builder.Append(part);
}

//matches will have 0 enumerable elements if there are no matches. var matches = regexMatcher.Matches(builder.ToString());
foreach (Match match in matches)
{
    Console.WriteLine($"MATCH: {match.ToString()}");
    Console.WriteLine($"    It was found starting at index {match.Index} of the full string");

    int partsLength = 0;
    for(int i = 0; i < parts.Length; i++)
    {
        var part = parts[i];
        
       partsLength += part.Length;

       if (match.Index <= partsLength)
       {
           //Console.WriteLine("    Match started in the {0} part. Match index = {1}, part len = {2}", i, match.Index, part.Length);
           Console.WriteLine("    Match started in part {0} and ended in part {1} (indices {2} and {3})", FindWhatPartIndexIsIn(match.Index), FindWhatPartIndexIsIn(match.Index + match.Length - 1), match.Index, match.Index + match.Length - 1);
           
           break;
       }

    }
    
    
}

int? FindWhatPartIndexIsIn(int index)
{
    int j = 0;
    foreach (int i in startIndices)
    {
        if (index < i)
        {
            
            //Console.WriteLine($"Index {index} is in part {j}");
            return j;
            break;
        }

        j++;
    }

    return startIndices.Count;
}
*/