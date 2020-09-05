using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
using Portal.Abstract;

namespace Portal.Gateway.Controllers
{
  [ApiController]
  [Route("[controller]")]
  public class HealthController : ControllerBase
  {

    private readonly ILogger<HealthController> logger;
    private IHealthService service;


    public HealthController(IHealthService service, ILogger<HealthController> logger)
    {
      this.service = service;
      this.logger = logger;
    }

    [HttpGet]
    public async Task<bool> Get()
    {
      return await this.service.Check();
    }
  }
}
