using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Net.Http.Headers;
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
               
                //Console.WriteLine(entry.Key + " : " + entry.Value);
                outputs[entry.Key] = entry.Value;
                
            }

             using (MemoryStream stream = new MemoryStream())
             {
                 DocX doc;
                 doc = DocX.Load(
                     @$"/run/media/ben/Windows/Users/Ben Saunders-Henning/AppData/Roaming/JSTemplates/templates/{outputs["Template"]}");
                 if (doc.FindUniqueByPattern(@"<[\w _-]{3,}>", System.Text.RegularExpressions.RegexOptions.IgnoreCase).Count > 0)
                 {
                     var replaceTextOptions = new FunctionReplaceTextOptions()
                     {
                         FindPattern = "<(.*?)>",
                         RegexMatchHandler = ReplaceFunc,
                         RegExOptions = System.Text.RegularExpressions.RegexOptions.IgnoreCase,
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
        
        [HttpGet("Test")]
        public int TestGet()
        {
            return 1;
        }
        
    }
}
