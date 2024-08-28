FROM mcr.microsoft.com/devcontainers/javascript-node:1-22-bookworm

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive
#     && apt-get -y install --no-install-recommends <your-package-list-here>

RUN apt-get update && apt-get install postgresql-client \
    && chmod +x ./db/init_db.sh && ./db/init_db.sh

# [Optional] Uncomment if you want to install an additional version of node using nvm
# ARG EXTRA_NODE_VERSION=10
# RUN su node -c "source /usr/local/share/nvm/nvm.sh && nvm install ${EXTRA_NODE_VERSION}"

# [Optional] Uncomment if you want to install more global node modules
# RUN su node -c "npm install -g <your-package-list-here>"
