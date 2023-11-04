using DocProcessor;
using Document = DocProcessor.Document;
using DocumentType = DocProcessor.DocumentType;

string testPath = "/home/ben/projects/JSTG/DocumentGenerationAPI/TESTING_FILES/AC_blah.docx";
string imageJS = "/home/ben/projects/JSTG/DocumentGenerationAPI/TESTING_FILES/images/JS.png";
string imageJM = "/home/ben/projects/JSTG/DocumentGenerationAPI/TESTING_FILES/images/JM.png";
string imageMM = "/home/ben/projects/JSTG/DocumentGenerationAPI/TESTING_FILES/images/MM.png";
string imageAS = "/home/ben/projects/JSTG/DocumentGenerationAPI/TESTING_FILES/images/AS.png";
Document doc = new Document(testPath, DocumentType.ExistingDocument);

Image Image = new Image(imageJS);
doc.ReplaceTextWithImage("<PICTURE>", Image);
Image ImageJM = new Image(imageJM);
doc.ReplaceTextWithImage("<JM>", ImageJM);
Image ImageJS = new Image(imageJS);
doc.ReplaceTextWithImage("<JS>", ImageJS);
Image ImageMM = new Image(imageMM);
doc.ReplaceTextWithImage("<MM>", ImageMM);
Image ImageAS = new Image(imageAS);
doc.ReplaceTextWithImage("<AS>", ImageAS);

doc.SearchAndReplaceTextByRegex(@"<([\w _-]{3,})>", GetHelloStr);
doc.SaveAs("/home/ben/projects/JSTG/DocumentGenerationAPI/TESTING_FILES/HELLO.docx");

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
// }
string ReplaceFunction(string key)
{
    string test = key;
    return "NULL: " + key;
}
