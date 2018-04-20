# uod-doc-project-alloc

> Discipline of Computing Honours Project Allocation

## Deployment

When deploying this application, you can either install it on an existing reverse proxy or use the provided Dockerfile
to host a basic server for testing.

### With Docker

This guide assumes you've already deployed the backend image, and are using an overlay network called `hpas`. Adjust as
necessary for your setup.

1. Build the image: `docker build --tag hpas_frontend:latest .`
2. Deploy the image: `docker run -d --name hpas_frontend --network hpas -p 80:80/tcp hpas_frontend:latest`
    - This assumes there's no other web server on the Docker host. If this isn't the case, try the without Docker
      deployment option below.

### Without Docker

This guide will show how to add a new virtual server to an existing Nginx server. The same rough steps should apply to
any reverse proxy + web server. First however, you need to build the project, with or without Docker to help.

#### Using Docker as a Build Environment (Recommended)

Using this method saves installing the additional JavaScript tooling on your machine, but does require Docker to be
available.

1. Build the `build` image: `docker build --target build --tag hpas_frontend_build:latest .`
2. Create a container from the image: `docker create --name build_tmp hpas_frontend_build:latest`
3. Copy built files: `docker cp build_tmp:/opt/hpas_build/dist ./dist`
4. Destroy build container: `docker rm build_tmp`

#### Building Manually

Use this method if you don't have Docker available or already do Node.js development on your machine. Prefer the Docker
method for better reproducibility, though.

1. Install NodeJS 9.0 or higher along with the [Yarn package manager](https://yarnpkg.com).
    - 🍺 If you're on a macOS machine, you can use Homebrew to fetch both: `brew install node yarn`
2. Build the project: `yarn run build`

#### Installing on Your Web Server

1. Copy the contents of the `dist` folder to your web server (this example will use `/opt/hpas/web`, change as needed)
2. Configure Nginx (or your proxy/web server of choice) to serve the assets and redirect API requests.
    - Change the `proxy_pass` URL to wherever you have the backend deployed. If deploying on the same host (recommended)
      use `localhost` with the appropriate port that was exposed from the backend container.
    - Set other configuration as needed. An SSL configuration is highly recommended, as is setting `server_name` if the
      server hosts multiple domain names. If using SSL, add a redirect to HTTPS for HTTP connections!
    - If you're not using Nginx, you'll need to translate this to your proxy/web server of choice.
    - Change the `proxy_pass` host/port pair as needed.

```nginx
server {
  listen 80;

  location /api/ {
    proxy_pass http://localhost:8080;
    proxy_pass_header Server;
    proxy_http_version 1.1;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
  }

  location / {
    root /opt/hpas/web;
    try_files $uri $uri/ /index.html;
  }
}
```

## Build Setup

``` bash
# install dependencies
npm install

# serve with hot reload at localhost:8888
npm run dev

# build for production with minification
npm run build

# build for production and view the bundle analyzer report
npm run build --report
```

For a detailed explanation on how things work, check out the [guide](http://vuejs-templates.github.io/webpack/) and [docs for vue-loader](http://vuejs.github.io/vue-loader).
