<template>
  <h1>{{ message }}</h1>
</template>

<script>
import { NodeClient, ContractClient } from "mazzaroth-js";
// This abiJSON file is output by building the contract
import abiJSON from "../ExampleContract.json";

export default {
  data: () => ({
    message: "Not Executed"
  }),
  created() {
    // Hard coding the channel, privKey and node address.
    const accountPrivKey = "0".repeat(64);
    const mazzNodeAddr = "http://localhost:8081";
    const channelID = "0".repeat(64);

    // Construct a node client.
    const nodeClient = new NodeClient(mazzNodeAddr, accountPrivKey);
    const contractClient = new ContractClient(
      abiJSON,
      nodeClient,
      null,
      channelID
    );
    contractClient.simple().then(res => {
      this.message = res;
    });
  }
};
</script>
