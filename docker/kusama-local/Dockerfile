FROM haidtpublic/polka-kusama-local as builder
LABEL description="This is the build stage for Polkadot. Here we create the binary."

ARG PROFILE=release
WORKDIR /usr/src/polkafoundry

# ===== SECOND STAGE ======
FROM ubuntu:20.04
WORKDIR /usr/src/polkafoundry
LABEL description="This is the 2nd stage: a very small image where we copy the Polkadot binary."
COPY --from=builder /usr/src/polkafoundry/kusama-local.json /usr/local/.
COPY --from=builder /usr/src/polkafoundry/polkadot /usr/local/bin

EXPOSE 30333 9933 9944

CMD ["./polkadot", "--chain", "kusama-local.json", "--validator", "--pruning", "archive", "--unsafe-ws-external", "--rpc-cors", "all"]