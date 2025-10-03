self.onmessage = function(e) {
    const { imageData, kernel } = e.data;
    const convolutedData = applyConvolution(imageData, kernel);
    self.postMessage(convolutedData);
};

function applyConvolution(imageData, kernel) {
    const data = imageData.data;
    const side = Math.round(Math.sqrt(kernel.length));
    const halfSide = Math.floor(side / 2);
    const src = data.slice();
    const w = imageData.width;
    const h = imageData.height;

    for (let y = 0; y < h; y++) {
        for (let x = 0; x < w; x++) {
            const dstOff = (y * w + x) * 4;
            let r = 0, g = 0, b = 0;

            for (let cy = 0; cy < side; cy++) {
                for (let cx = 0; cx < side; cx++) {
                    const scy = y + cy - halfSide;
                    const scx = x + cx - halfSide;

                    if (scy >= 0 && scy < h && scx >= 0 && scx < w) {
                        const srcOff = (scy * w + scx) * 4;
                        const wt = kernel[cy * side + cx];
                        r += src[srcOff] * wt;
                        g += src[srcOff + 1] * wt;
                        b += src[srcOff + 2] * wt;
                    }
                }
            }
            data[dstOff] = r;
            data[dstOff + 1] = g;
            data[dstOff + 2] = b;
        }
    }
    return imageData;
}