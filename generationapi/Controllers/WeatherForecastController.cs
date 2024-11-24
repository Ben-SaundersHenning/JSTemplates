using Microsoft.AspNetCore.Mvc;

namespace generationapi.Controllers;

[ApiController]
[Route("[controller]")]
public class DocumentController : ControllerBase
{
    private static readonly string[] Summaries = new[]
    {
        "Freezing", "Bracing", "Chilly", "Cool", "Mild", "Warm", "Balmy", "Hot", "Sweltering", "Scorching"
    };

    private readonly ILogger<DocumentController> _logger;

    public DocumentController(ILogger<DocumentController> logger)
    {
        _logger = logger;
    }

    [HttpGet(Name = "GetDocument")]
    public String Get()
    {
        return "Hello, World!";
    }
    
}