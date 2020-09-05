using System;
using System.Threading.Tasks;
using Portal.Abstract;
using Portal.Health;

namespace Portal.Services
{
  public class HealthService : IHealthService
  {
    private Check.CheckClient client;

    public HealthService(Health.Check.CheckClient client)
    {
      this.client = client;
    }

    public async Task<bool> Check()
    {
      var response = await this.client.MessageAsync(new HealthRequest {Payload = "ping"});
      return response.Payload == "ping pong";
    }
  }
}
