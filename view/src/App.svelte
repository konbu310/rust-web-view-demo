<script lang="ts">
  import { rpc } from "./rpc";

  let text: string;

  const handleClick = () => {
    rpc.log(text);
  };
</script>

<main>
  <label>
    <input type="text" bind:value={text} />
  </label>
  <button on:click={handleClick}>Logger</button>
</main>

<style>
</style>
