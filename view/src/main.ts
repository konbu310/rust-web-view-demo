import App from "./App.svelte";
import { rpc } from "./rpc";

rpc.init();

const app = new App({
  target: document.body,
  props: {},
});

export default app;
