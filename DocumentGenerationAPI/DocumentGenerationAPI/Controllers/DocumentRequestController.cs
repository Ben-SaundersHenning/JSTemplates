using Microsoft.AspNetCore.Mvc;
using DocProcessor;
using Document = DocProcessor.Document;

namespace DocumentGenerationAPI.Controllers
{
    [Route("api/DocumentRequest")]
    [ApiController]
    public class DocumentRequestController : ControllerBase
    {
        
        private Dictionary<string, string> _outputs = new Dictionary<String,String>();

        public DocumentRequestController()
        {
        }

        [HttpPost("DocRequest")]
        public IActionResult DocRequest([FromBody] Dictionary<string, string> data)
        {

            foreach(KeyValuePair<string, string> entry in data)
            {
               
                _outputs[entry.Key] = entry.Value;
                
            }
            
            using (MemoryStream stream = new MemoryStream())
            {

                Document document = new Document($"{_outputs["TEMPLATE PATH"]}{_outputs["TEMPLATE"]}",
                    DocumentType.ExistingDocument);
               
                //image replace has to be done first, since the tag matches the text replacement tags.
                Image image = new Image($"{_outputs["IMAGE PATH"]}{_outputs["IMAGE"]}");
                document.ReplaceTextWithImage("<PICTURE>", image);
                
                document.SearchAndReplaceTextByRegex(@"<([\w _-]{3,})>", ReplaceFunction);

                document.SaveAsStream(stream);
                document.Dispose();
                byte[] test = stream.ToArray();
                _outputs.Clear();
                return new FileContentResult(test, "application/octet-stream");
                
            }
        }
        
       [HttpPost("F1Request")]
        public IActionResult Form1([FromBody] Dictionary<string, string> data)
        {

            foreach(KeyValuePair<string, string> entry in data)
            {
               
                _outputs[entry.Key] = entry.Value;
                
            }
            
            using (MemoryStream stream = new MemoryStream())
            {

                Document document = new Document($"{_outputs["TEMPLATE PATH"]}/F1.docx",
                    DocumentType.ExistingDocument);
                
                //image replace has to be done first, since the tag matches the text replacement tags.
                Image image = new Image($"{_outputs["IMAGE PATH"]}{_outputs["IMAGE"]}");
                document.ReplaceTextWithImage("<PICTURE>", image);
                
                document.SearchAndReplaceTextByRegex(@"<([\w _-]{3,})>", ReplaceFunction);

                document.SaveAsStream(stream);
                document.Dispose();
                byte[] test = stream.ToArray();
                _outputs.Clear();
                return new FileContentResult(test, "application/octet-stream");
                
            }
        } 
        
        private string ReplaceFunction(string key)
        {
            if(_outputs.TryGetValue(key, out var val))
            {
                return val;
            }

            return "NULL: " + key;
        }
        
    }
}
