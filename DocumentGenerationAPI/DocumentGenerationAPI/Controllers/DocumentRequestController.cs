using Microsoft.AspNetCore.Mvc;
using DocProcessor;
using DocumentFormat.OpenXml.Office2013.PowerPoint.Roaming;
using Newtonsoft.Json.Linq;
using NuGet.Protocol;
using Document = DocProcessor.Document;

namespace DocumentGenerationAPI.Controllers
{
    [Route("api/DocumentRequest")]
    [ApiController]
    public class DocumentRequestController : ControllerBase
    {
        
        private Dictionary<string, string> _outputs = new Dictionary<String,String>();
        
        private JObject Obj { get; set; }
        
        private IConfiguration Config { get; set; }

        public DocumentRequestController(IConfiguration config)
        {
            Config = config;
            Obj = default!;
        }
        
        [HttpPost("DocRequest")]
        public IActionResult DocRequest([FromBody] string data)
        {

            Obj = JObject.Parse(data);
            
            Console.WriteLine($"OBJ = {Obj.ToString()}");

            using (MemoryStream stream = new MemoryStream())
            {

                string docPath = Config["Templates"];
                string? type = (string)Obj.SelectToken("asmtType");
                if (type != null)
                {
                    docPath = docPath + type;
                }

                // Document document = new Document(docPath,
                //         DocumentType.ExistingDocument);

                string imgPath = Config["Images"];

                string? last = (string)Obj.SelectToken("assessor.lastName");
                // if (first != null && last != null)
                // {
                //     imgPath = imgPath + first[0] + last[0] + ".png";
                // }
                
                Console.WriteLine("IMGPATH = {0}", imgPath);

            //image replace has to be done first, since the tag matches the text replacement tags.
            // Image image = new Image(imgPath); //TEMP
            //     document.ReplaceTextWithImage("<PICTURE>", image);
            //     
            //     document.SearchAndReplaceTextByRegex(@"<([\w _-]{3,})>", ReplaceFunction);
            //
            //     document.SaveAsStream(stream);
            //     document.Dispose();
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
                
                document.SearchAndReplaceTextByRegex(@"<([\w _-]{3,})>", ReplaceFunctionF1);

                document.SaveAsStream(stream);
                document.Dispose();
                byte[] test = stream.ToArray();
                _outputs.Clear();
                
                return new FileContentResult(test, "application/octet-stream");
                
            }
        }

        private string ReplaceFunctionJ(string path)
        {
            JToken? value = Obj.SelectToken(path);

            if (value != null)
            {
                String val = value.ToString();
                if (val.StartsWith("DO")) //date of
                {
                    bool success = DateTime.TryParse(val, out DateTime result);
                    if (success)
                    {
                        return $"{result:MMMM dd, yyyy}";
                    }
                }
                return val;
            }

            return "NULL: " + path;

        }
       
        private string ReplaceFunction(string key)
        {
            if(_outputs.TryGetValue(key, out var val))
            {
                if (key.StartsWith("DO")) //date of
                {
                    bool success = DateTime.TryParse(val, out DateTime result);
                    if (success)
                    {
                        return $"{result:MMMM dd, yyyy}";
                    }
                }
                return val;
            }

            return "NULL: " + key;
        }
        
        private string ReplaceFunctionF1(string key)
        {
            if(_outputs.TryGetValue(key, out var val))
            {
                if (key.StartsWith("DO")) //date of
                {
                    bool success = DateTime.TryParse(val, out DateTime result);
                    if (success)
                    {
                        return $"{result:yyyy-MM-dd}";
                    } 
                }
                return val;
            }

            return "NULL: " + key;
        }
        
    }
}
