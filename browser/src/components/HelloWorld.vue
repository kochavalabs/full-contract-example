<template>
  <div>
    <h1> Full Contract Example </h1>
    <p>This button should submit a transaction to a Mazzaroth Node running at {{ address }}</p>
    <p>If it does not work, check that you have started a node and deployed the contract.</p>
    <button v-on:click="execute">Execute simple</button>
  </div>
</template>

<script>
import { NodeClient, ContractClient } from "mazzaroth-js";
// This abiJSON file is output by building the contract
import abiJSON from "../ExampleContract.json";

export default {
  data: () => ({
    address: "http://localhost:8081"
  }),

  methods: {
    execute() {
      // Hard coding the channel, privKey
      const accountPrivKey = "0".repeat(64);
      const channelID = "0".repeat(64);

      // Construct a node client.
      const nodeClient = new NodeClient(this.address, accountPrivKey);
      const contractClient = new ContractClient(
        abiJSON,
        nodeClient,
        null,
        channelID
      );

      // Execute simple function and return response
      contractClient.simple().then(res => {
        alert("Executed simple function, returned result: " + res);
      })
    }
  }
};
</script>
