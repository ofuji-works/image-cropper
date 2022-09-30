import("image-cropper").then(async (wasm) => {
    const url = "./images/sample.png";
    const res = await fetch(url);
    const blob = await res.blob();
    const resized = await cropImageWasm(blob, 100, 100, 200, 200, 'png', wasm);

    const a = document.getElementById('download');
    a.href = window.URL.createObjectURL(resized);
});

async function cropImageWasm(file, start_x, start_y, end_x, end_y, format, wasm) {
    console.log(`Original: ${file.size} Bytes`);
    const arr = new Uint8Array(await file.arrayBuffer());
  
    const result = wasm.crop_image(arr, start_x, start_y, end_x, end_y, format);

    const blob = new Blob([result]);
    console.log(`Resized: ${blob.size} Bytes`);
  
    return blob
}
