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
        
        private IConfiguration Config { get; }

        public DocumentRequestController(IConfiguration config)
        {
            Config = config;
            Obj = default!;
        }
        
        [HttpPost("DocRequest")]
        public IActionResult DocRequest([FromBody] string data)
        {

            Obj = JObject.Parse(data);
            
            byte[] result = GenerateDocument(Obj, ReplaceFunction, false);
            
            return new FileContentResult(result, "application/octet-stream");
            
        }

       [HttpPost("F1Request")]
        public IActionResult Form1([FromBody] string data)
        {

            Obj = JObject.Parse(data);

            byte[] result = GenerateDocument(Obj, ReplaceFunctionF1, true);
            
            return new FileContentResult(result, "application/octet-stream");
            
        }
        
        private byte[] GenerateDocument(JObject data, Func<string, string> GetReplacement, bool isF1)
        {
            using MemoryStream stream = new MemoryStream();
            string docPath = Config["Templates"];
            string? type;
            if (isF1)
            {
                type = "F1.docx";
            }
            else
            {
                type = (string)Obj.SelectToken("asmtType");
            }
            if (type != null)
            {
                docPath = docPath + type;
            }

            Document document = new Document(docPath,
                    DocumentType.ExistingDocument);

            string imgPath = Config["Images"];

            string? last = (string)Obj.SelectToken("assessor.lastName");
            string? first = (string)Obj.SelectToken("assessor.firstName");
            if (first != null && last != null)
            {
                imgPath = imgPath + first[0] + last[0] + ".png";
            }
                
            //image replace has to be done first, since the tag matches the text replacement tags.
            Image image = new Image(imgPath); //TEMP
                document.ReplaceTextWithImage("<assessor.signature>", image);
                
                document.SearchAndReplaceTextByRegex(@"<([\w ._-]{3,})>", GetReplacement);
           
                document.SaveAsStream(stream);
                document.Dispose();
            _outputs.Clear();
            return stream.ToArray();
        }

        private string ReplaceFunction(string path)
        {
            JToken? value = Obj.SelectToken(path);

            if (value != null)
            {
                String val = value.ToString();
                if (path.Contains("date"))
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
       
        private string ReplaceFunctionF1(string path)
        {
            JToken? value = Obj.SelectToken(path);

            if (value != null)
            {
                String val = value.ToString();
                if (path.Contains("date"))
                {
                    bool success = DateTime.TryParse(val, out DateTime result);
                    if (success)
                    {
                        return $"{result:yyyy-MM-dd}";
                    } 
                }
                
                return val;
            }

            return "NULL: " + path;
        }
        
    }
}
