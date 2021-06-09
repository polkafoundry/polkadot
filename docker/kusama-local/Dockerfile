FROM ubuntu:20.04
LABEL description="Testing Kusama"
WORKDIR /usr/src/polkafoundry

COPY . .

EXPOSE 30333 9933 9944

CMD ["./polkadot", "--chain", "kusama-local.json", "--validator", "--pruning", "archive", "--unsafe-ws-external", "--rpc-cors", "all"]