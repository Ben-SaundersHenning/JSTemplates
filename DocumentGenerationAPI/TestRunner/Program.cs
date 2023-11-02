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
Document doc = new Document(testPath, DocumentType.ExistingDocument);

doc.SearchAndReplaceTextByRegex(@"<([\w _-]{3,})>", GetHelloStr);
doc.SaveAs("/home/ben/projects/JSTG/DocumentGenerationAPI/TESTING_FILES/HELLO.docx");
doc.SearchAndReplaceText("Hello!", "Bye!");
doc.SaveAs("/home/ben/projects/JSTG/DocumentGenerationAPI/TESTING_FILES/BYE.docx");
doc.Dispose();

string GetHelloStr(string match)
{
    return "Hello!";
}

using (MemoryStream stream = new MemoryStream())
{

    Document document = new Document(testPath, DocumentType.ExistingDocument);
    
    document.SearchAndReplaceTextByRegex(@"<([\w _-]{3,})>", ReplaceFunction);
    document.SaveAsStream(stream);
    document.Dispose();
    byte[] test = stream.ToArray();
    int i = 0;
}

// string ReplaceFunction(string str)
// {
//     return "Ben";
// }
string ReplaceFunction(string key)
{
    string test = key;
    return "NULL: " + key;
}
