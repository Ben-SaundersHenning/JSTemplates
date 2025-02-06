using Microsoft.AspNetCore.Mvc;

namespace generationapi.Controllers;

public class DocumentRequestController : Controller
{
    // GET
    public IActionResult Index()
    {
        return View();
    }
}