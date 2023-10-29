using System.Net.Mime;
using System.Text;
using System.Text.RegularExpressions;
using DocProcessor;
using DocumentFormat.OpenXml;
using DocumentFormat.OpenXml.Packaging;
using DocumentFormat.OpenXml.Wordprocessing;
using Document = DocProcessor.Document;
using DocumentType = DocProcessor.DocumentType;

string testPath = "/home/ben/projects/JSTG/DocumentGenerationAPI/TESTING_FILES/CAT CAT GOSE.docx";
using (Document doc = new Document(testPath, DocumentType.ExistingDocument))
{

    //TODO: instead of a replacementstr, pass in a callback function
    //that returns the replacement string. The callback should take 
    //in the matched tag (unedited).
    doc.SearchAndReplaceText(@"<[\w _-]{3,}>", "Blah");

}