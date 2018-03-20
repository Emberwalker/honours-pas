import Axios, {AxiosResponse} from "axios";
import store from "../stores";

class HttpProvider {
  private base: string;

  constructor(base: string) {
    this.base = base;
  }

  public get(endpoint: string): Promise<AxiosResponse> {
    return Axios.get(this.base + endpoint);
  }

  public post(endpoint: string, data: object): Promise<AxiosResponse> {
    return Axios.post(this.base + endpoint, data);
  }

  public put(endpoint: string, data: object): Promise<AxiosResponse> {
    return Axios.put(this.base + endpoint, data);
  }

  public delete(endpoint: string): Promise<AxiosResponse> {
    return Axios.delete(this.base + endpoint);
  }
}

export default new HttpProvider("/api/v1");
