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

        public DocumentRequestController(ILogger<WeatherForecastController> logger)
        {
            _logger = logger;
        }

        [HttpPost]
        public IActionResult Cat([FromBody] DocumentRequest request)
        {

            byte[] file = System.IO.File.ReadAllBytes(request.baseTemplate);
           
            using (MemoryStream stream = new MemoryStream())
            {
                DocX doc = DocX.Load(@"/run/media/ben/Windows/Users/Ben Saunders-Henning/AppData/Roaming/JSTemplates/templates/AC.docx");
                doc.InsertParagraph("Blah blah blah blah blah blob lbol lb jsad;lfkj as;lf jk");
                doc.SaveAs(stream);
                byte[] test = stream.ToArray();

                return new FileContentResult(test, "application/octet-stream");
            } 
        }

        [HttpGet("Test")]
        public int TestGet()
        {
            return 1;
        }
        
    }
}
