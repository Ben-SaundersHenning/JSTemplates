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
using Xceed.Document.NET;
using Xceed.Words.NET;

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

        [HttpPost]
        public IActionResult Cat([FromBody] Dictionary<string, string> data)
        {

            foreach(KeyValuePair<string, string> entry in data)
            {
               
                outputs[entry.Key] = entry.Value;
                
            }
            
            using (MemoryStream stream = new MemoryStream())
            {
                DocX doc = DocX.Load($"{outputs["TEMPLATE PATH"]}{outputs["TEMPLATE"]}");

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
                }

                if (doc.FindUniqueByPattern(@"<[\w _-]{3,}>", RegexOptions.IgnoreCase).Count > 0)
                {
                    var replaceTextOptions = new FunctionReplaceTextOptions()
                    {
                        FindPattern = "<(.*?)>",
                        RegexMatchHandler = ReplaceFunc,
                        RegExOptions = RegexOptions.IgnoreCase,
                        NewFormatting = new Formatting() { FontColor = System.Drawing.Color.Black, Size = 12, FontFamily = new Font("Times New Roman") }
                    };
            
                    doc.ReplaceText(replaceTextOptions);
                    doc.SaveAs(stream);
                    byte[] test = stream.ToArray();
                    outputs = null;
                    return new FileContentResult(test, "application/octet-stream");
                }
                 
                return null;
                
            }
        }
        
        private string ReplaceFunc(string findStr)
        {
            if(outputs.ContainsKey(findStr))
            {
                return outputs[findStr];
            }

            return "NULL: " + findStr;
        }
        
    }
}
