FROM ubuntu:latest AS build

# Setup yarn and NodeSource repos, then install everything we need.
# We also remove cmdtest if installed as per the Yarn install docs.
RUN apt-get update && apt-get install -y curl apt-transport-https && \
    curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - && \
    echo "deb https://dl.yarnpkg.com/debian/ stable main" > /etc/apt/sources.list.d/yarn.list && \
    curl -sL https://deb.nodesource.com/setup_9.x | bash - && \
    apt-get update && \
    apt-get install -y cmdtest- build-essential nodejs yarn

# Copy our project in and build.
COPY . /opt/hpas_build
WORKDIR /opt/hpas_build
RUN yarn install && yarn run build

FROM nginx:stable-alpine
COPY --from=build /opt/hpas_build/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 80/tcp
