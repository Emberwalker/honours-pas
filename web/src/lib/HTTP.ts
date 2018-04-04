import Axios, {AxiosError, AxiosResponse} from "axios";
import Mutations from "../lib/Mutations";
import store from "../stores";
import { getErrorCommit } from "../stores";

class HttpProvider {
  private base: string;

  constructor(base: string) {
    this.base = base;
  }

  public get(endpoint: string, allowErrors: boolean = false): Promise<AxiosResponse> {
    const promise = Axios.get(this.base + endpoint);
    if (!allowErrors) {
      promise.catch((err: AxiosError) => {
        let errServer = err;
        if (err.response) {
          const rawRes = err.response.data;
          try {
            const res = JSON.parse(err.response.data);
            if (res.message) { errServer = res.message; }
          } catch (ex) {
            // Pass.
          }
        }
        store.commit(getErrorCommit("An error occurred talking to the backend server.", errServer));
      });
    }
    return promise;
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

let instance = null as HttpProvider | null;

function getInstance(): HttpProvider {
  if (instance) { return instance; }
  instance = new HttpProvider("/api/v1");
  return instance;
}

// export default new HttpProvider("/api/v1");
export default getInstance;
