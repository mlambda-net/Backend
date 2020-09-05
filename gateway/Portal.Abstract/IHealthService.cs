using System.Threading.Tasks;

namespace Portal.Abstract
{
  public interface IHealthService
  {
    public Task<bool> Check();
  }
}
