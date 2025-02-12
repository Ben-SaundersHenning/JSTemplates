using System.Runtime.InteropServices.JavaScript;
using DocProcessor;
using Microsoft.AspNetCore.Mvc;
using Newtonsoft.Json.Linq;

namespace generationapi.Controllers;

public class DocumentRequestController : Controller
{
    
    private JObject Obj { get; set; }

    public DocumentRequestController()
    {
        Obj = new JObject();
    }
    
    /*
    public IActionResult DocRequest([FromBody] string data)
    {

        Obj = JObject.Parse(data);
            
        // byte[] result = GenerateDocument(Obj);
            
        return new FileContentResult(result, "application/octet-stream");
            
    }*/

    private byte[] GenerateDocument(JObject data, Func<string, string> replacementFunc)
    {
        
        // 1. Open stream
        
        using MemoryStream stream = new();
        
        // 2. Find doc path, open document
        
        string docPath = (string)data.SelectToken("document.path");

        Document doc = new Document(docPath, DocumentType.ExistingDocument);

        // 3. Resolve conditional statements
        
        // TODO
        
        // 3. Find image path
        
        string imgPath = (string)data.SelectToken("signature_path");
        
        // 4. Insert image into document
        
        //image replace has to be done first, since the tag matches the text replacement tags.
        Image image = new(imgPath);
        doc.ReplaceTextWithImage("<assessor.signature>", image); 
        
        // 5. Replace all tags
        
        //replace the tags
        doc.SearchAndReplaceTextByRegex(@"<([\w \[\]._-]{3,})>", replacementFunc); 
        
        // 6. Save doc into byte array
        
        doc.SaveAsStream(stream);
        doc.Dispose();
        
        return stream.ToArray();

    }
    
    // Given a JSON path and returns the value that is to be inserted 
    // at the position of the path
    private string TagReplace(string objPath)
    {
        
        // 1. Try to get the token from Obj
        // 2. if the value has formatting (like a date), format it and then replace it
        // 3. otherwise just return the value
        // 4. if the value does not exist, return {ERR: key}

        JToken token;

        if (Obj.TryGetValue(objPath, out token))
        {

            string val = token.ToString();

            DateOnly date;

            if (DateOnly.TryParse(val, out date))
            {
                // this is a date, need to check formatting.
                // ex: <@ [key.date] -f YYYY-mm-dd >>
                // TODO
            }

            if (val.Contains("pronoun"))
            {
                // ex: <@ [claimant.gender] -t p0 --upper >>
                // TODO
            }

            return val;

        }

        return $"{{ERR: {objPath}}}";

    }

}