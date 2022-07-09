<script lang="ts">
  import { readDir, BaseDirectory } from "@tauri-apps/api/fs";

  async function handleClick() {
    const entries = await readDir("users", {
      dir: BaseDirectory.App,
      recursive: true,
    });
    function processEntries(entries) {
      for (const entry of entries) {
        console.log(`Entry: ${entry.path}`);
        if (entry.children) {
          processEntries(entry.children);
        }
      }
    }
  }
</script>

<div>
  <button class="btn" on:click={handleClick}>list dir</button>
</div>
