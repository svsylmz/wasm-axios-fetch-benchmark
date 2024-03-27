<template>
  <div>
    <button @click="runBenchmark">Run Benchmark</button>
    <div v-if="results.length > 0">
      <h2>Results</h2>
      <ul>
        <li v-for="(result, index) in results" :key="index">
          <h3>{{ result.type }}</h3>
          <p>Total Requests: {{ result.totalRequests }}</p>
          <p>Average Time (ms): {{ result.avgTime }}</p>
          <p>Min Time (ms): {{ result.minTime }}</p>
          <p>Max Time (ms): {{ result.maxTime }}</p>
        </li>
      </ul>
    </div>
  </div>
</template>

<script>
import * as wasm from "hello-wasm";
import axios from "axios";
export default {
  data() {
    return {
      results: [],
      numRequests: 1, // Number of requests to make for each method
      requestUrl: "http://localhost:3000/allData",
    };
  },
  methods: {
    async runBenchmark() {
      this.results = [];
      await this.benchmarkAxios();
      await this.benchmarkWasmRest();
      await this.benchmarkFetchAPI();
    },
    async benchmarkAxios() {
      const axiosTimes = [];
      for (let i = 0; i < this.numRequests; i++) {
        const start = performance.now();
        try {
          await axios.get(this.requestUrl);
          const end = performance.now();
          axiosTimes.push(end - start);
        } catch (error) {
          console.error("Axios Error:", error);
        }
      }
      this.recordResults("Axios", axiosTimes);
    },
    async benchmarkWasmRest() {
      const wasmTimes = [];
      for (let i = 0; i < this.numRequests; i++) {
        const start = performance.now();
        try {
          await wasm.make_get_request(this.requestUrl);
          const end = performance.now();
          wasmTimes.push(end - start);
        } catch (error) {
          console.error("Wasm-Rest Error:", error);
        }
      }
      this.recordResults("Wasm-Rest", wasmTimes);
    },
    async benchmarkFetchAPI() {
      const fetchTimes = [];
      for (let i = 0; i < this.numRequests; i++) {
        const start = performance.now();
        try {
          const response = await fetch(this.requestUrl);
          await response.json();
          const end = performance.now();
          fetchTimes.push(end - start);
        } catch (error) {
          console.error("Fetch API Error:", error);
        }
      }
      this.recordResults("Fetch API", fetchTimes);
    },
    recordResults(type, times) {
      const totalRequests = times.length;
      const sum = times.reduce((acc, cur) => acc + cur, 0);
      const avgTime = sum / totalRequests;
      const minTime = Math.min(...times);
      const maxTime = Math.max(...times);
      this.results.push({ type, totalRequests, avgTime, minTime, maxTime });
    },
  },
};
</script>

<style scoped>
button {
  margin-bottom: 10px;
}
</style>
