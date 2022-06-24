<script context="module">
  import init from "../../../pkg/wgpu_svelte_template";

  export async function getServerFile(path) {
    const response = await fetch(path);
    const data = await response.text();
    return data;
  }

  export async function getImageBytes(path) {
    const response = await fetch(path);
    const data = await response.arrayBuffer();
    const ints = new Int8Array(data);

    let image = new Image();
    // return { ints, dimensions: response.headers.get("x-dimensions") };
  }
  function webGPUTextureFromImageBitmapOrCanvas(source) {
    console.log(source);
  }
  async function webGPUTextureFromImageUrl(url) {
    // Note that this is an async function
    const response = await fetch(url);
    const blob = await response.blob();
    const int8Array = decode(await blob.arrayBuffer());
    const imgBitmap = await createImageBitmap(blob);

    return webGPUTextureFromImageBitmapOrCanvas(imgBitmap);
  }

  webGPUTextureFromImageUrl("images/wasm.png");
  init("wgpu_svelte_template_bg.wasm").then(() => {});
</script>

<script lang="ts">
  import { onMount } from "svelte";
  let canvas: HTMLCanvasElement;

  onMount(() => {
    window.addEventListener("resize", () => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;

      canvas.style.height = window.innerHeight + "px";
      canvas.style.width = window.innerWidth + "px";
    });
  });
</script>

<canvas bind:this={canvas} id="canvas" class="w-screen h-screen">
  <slot />
</canvas>
