using System.Net.Mime;
using DocProcessor;
using DocumentFormat.OpenXml;
using DocumentFormat.OpenXml.Packaging;
using DocumentFormat.OpenXml.Wordprocessing;
using Document = DocProcessor.Document;
using DocumentType = DocProcessor.DocumentType;

string testPath = "/home/ben/Documents/docs/jsot/2023/October/Josh/sdf sdf/AC_blah.docx";
using (Document doc = new Document(testPath, DocumentType.ExistingDocument))
{ 
    //doc.CombineAllRuns();
    doc.SearchAndReplaceTextByRegex(@"<[\w _-]{3,}>", "Blah");
    //doc.SearchAndReplaceTest(@"<[\w _-]{3,}>", "Blah");
}

//CreateWordprocessingDocument(testPath);

void CreateWordprocessingDocument(string filepath)
{
    // Create a document by supplying the filepath. 
    using WordprocessingDocument wordDocument =
        WordprocessingDocument.Create(filepath, WordprocessingDocumentType.Document);
    // Add a main document part. 
    MainDocumentPart mainPart = wordDocument.AddMainDocumentPart();

    // Create the document structure and add some text.
    mainPart.Document = new DocumentFormat.OpenXml.Wordprocessing.Document();
    Body body = mainPart.Document.AppendChild(new Body());
    Paragraph para = body.AppendChild(new Paragraph());
    Run run = para.AppendChild(new Run());
    run.AppendChild(new Text("Create text in body - CreateWordprocessingDocument"));
} 
