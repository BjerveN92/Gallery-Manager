<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  // Temporary test page for the four backend commands.
  let folder = $state("gallery");
  let filePath = $state("");
  let order = $state(0);
  let publicId = $state("");
  let output = $state("");
  let busy = $state(false);

  async function run(command: string, args: Record<string, unknown>) {
    busy = true;
    output = `${command}…`;
    try {
      const result = await invoke(command, args);
      output = JSON.stringify(result, null, 2);
    } catch (e) {
      output = `ERROR: ${e}`;
    } finally {
      busy = false;
    }
  }
</script>

<main>
  <h1>Backend test</h1>

  <label>
    Folder
    <input bind:value={folder} />
  </label>

  <section>
    <button disabled={busy} onclick={() => run("list_images", { folder })}>
      list_images
    </button>
  </section>

  <section>
    <label>
      File path
      <input bind:value={filePath} placeholder="C:\Users\...\bild.jpg" />
    </label>
    <label>
      Order
      <input type="number" min="0" bind:value={order} />
    </label>
    <button
      disabled={busy || !filePath}
      onclick={() => run("upload_image", { filePath, folder, order })}
    >
      upload_image
    </button>
  </section>

  <section>
    <label>
      Public ID
      <input bind:value={publicId} placeholder="gallery/abc123" />
    </label>
    <button
      disabled={busy || !publicId}
      onclick={() => run("update_order", { publicId, order })}
    >
      update_order (uses Order above)
    </button>
    <button
      disabled={busy || !publicId}
      onclick={() => run("delete_image", { publicId })}
    >
      delete_image
    </button>
  </section>

  <pre>{output}</pre>
</main>

<style>
  main {
    max-width: 800px;
    margin: 0 auto;
    padding: 1.5rem;
    font-family: system-ui, sans-serif;
  }
  section {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: end;
    margin: 1rem 0;
  }
  label {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    font-size: 0.85rem;
  }
  input {
    padding: 0.4rem;
    min-width: 16rem;
  }
  input[type="number"] {
    min-width: 5rem;
  }
  button {
    padding: 0.5rem 0.9rem;
  }
  pre {
    background: #111;
    color: #ddd;
    padding: 1rem;
    border-radius: 6px;
    max-height: 50vh;
    overflow: auto;
    white-space: pre-wrap;
  }
</style>
