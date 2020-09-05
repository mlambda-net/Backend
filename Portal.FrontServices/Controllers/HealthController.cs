using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace Portal.Controllers
{
  [ApiController]
  [Route("[controller]")]
  public class HealthController: ControllerBase
  {
    private readonly ILogger<HealthController> logger;

    public HealthController(ILogger<HealthController> logger)
    {
      this.logger = logger;
    }

    [HttpGet]
    public string Get()
    {
      return "ok";
    }
  }
}
