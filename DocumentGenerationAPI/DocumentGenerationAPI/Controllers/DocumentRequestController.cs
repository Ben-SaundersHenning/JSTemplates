using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Net.Http.Headers;
using System.Runtime.InteropServices;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using TemplateGenerationAPI.Models;
using DocProcessor;
using Document = DocProcessor.Document;

namespace TemplateGenerationAPI.Controllers
{
    [Route("api/DocumentRequest")]
    [ApiController]
    public class DocumentRequestController : ControllerBase
    {
        private readonly ILogger<WeatherForecastController> _logger;
        private Dictionary<string, string> outputs = new Dictionary<String,String>();

        public DocumentRequestController(ILogger<WeatherForecastController> logger)
        {
            _logger = logger;
        }

        [HttpPost("DocRequest")]
        public IActionResult DocRequest([FromBody] Dictionary<string, string> data)
        {

            foreach(KeyValuePair<string, string> entry in data)
            {
               
                outputs[entry.Key] = entry.Value;
                
            }
            
            using (MemoryStream stream = new MemoryStream())
            {

                Document document = new Document($"{outputs["TEMPLATE PATH"]}{outputs["TEMPLATE"]}",
                    DocumentType.ExistingDocument);
                
                document.SearchAndReplaceText(@"<[\w _-]{3,}>", ReplaceFunction);
                
                /* not implemented yet
                if(RuntimeInformation.IsOSPlatform(OSPlatform.Windows)) {
                    var image = doc.AddImage(
                        $"{outputs["IMAGE PATH"]}{outputs["IMAGE"]}");
                    var picture = image.CreatePicture();
                    ObjectReplaceTextOptions options = new ObjectReplaceTextOptions();
                    options.RegExOptions = RegexOptions.IgnoreCase;
                    options.NewObject = picture;
                    options.SearchValue = "<PICTURE>";
                    options.TrackChanges = false;
                    doc.ReplaceTextWithObject(options);
                } */

                document.SaveAsStream(stream);
                byte[] test = stream.ToArray();
                outputs.Clear();
                return new FileContentResult(test, "application/octet-stream");
                
            }
        }
        
       [HttpPost("F1Request")]
        public IActionResult Form1([FromBody] Dictionary<string, string> data)
        {

            foreach(KeyValuePair<string, string> entry in data)
            {
               
                outputs[entry.Key] = entry.Value;
                
            }
            
            using (MemoryStream stream = new MemoryStream())
            {

                Document document = new Document($"{outputs["TEMPLATE PATH"]}/F1.docx",
                    DocumentType.ExistingDocument);
                
                document.SearchAndReplaceText(@"<[\w _-]{3,}>", ReplaceFunction);
                
                /* not implemented yet
                if(RuntimeInformation.IsOSPlatform(OSPlatform.Windows)) {
                    var image = doc.AddImage(
                        $"{outputs["IMAGE PATH"]}{outputs["IMAGE"]}");
                    var picture = image.CreatePicture();
                    ObjectReplaceTextOptions options = new ObjectReplaceTextOptions();
                    options.RegExOptions = RegexOptions.IgnoreCase;
                    options.NewObject = picture;
                    options.SearchValue = "<PICTURE>";
                    options.TrackChanges = false;
                    doc.ReplaceTextWithObject(options);
                } */

                document.SaveAsStream(stream);
                byte[] test = stream.ToArray();
                outputs.Clear();
                return new FileContentResult(test, "application/octet-stream");
                
            }
        } 
        
        private string ReplaceFunction(string findStr)
        {

            Regex matcher = new Regex("<(.*?)>");
            string key = matcher.Match(findStr).Groups[0].Value;
            
            if(outputs.ContainsKey(key))
            {
                return outputs[findStr];
            }

            return "NULL: " + findStr;
        }
        
    }
}
