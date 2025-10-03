const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const imageLoader = document.getElementById('imageLoader');
const saveButton = document.getElementById('saveButton');
const resetButton = document.getElementById('resetButton');
const fileNameDisplay = document.getElementById('fileName');
const themeToggle = document.getElementById('theme-toggle');

// Filter inputs
const brightnessInput = document.getElementById('brightness');
const contrastInput = document.getElementById('contrast');
const grayscaleInput = document.getElementById('grayscale');
const blurInput = document.getElementById('blur');
const saturateInput = document.getElementById('saturate');
const hueRotateInput = document.getElementById('hue-rotate');
const invertInput = document.getElementById('invert');
const sepiaInput = document.getElementById('sepia');
const opacityInput = document.getElementById('opacity');

// Transformation buttons
const rotateLeftButton = document.getElementById('rotateLeft');
const rotateRightButton = document.getElementById('rotateRight');
const flipHorizontalButton = document.getElementById('flipHorizontal');
const flipVerticalButton = document.getElementById('flipVertical');

const vintageButton = document.getElementById('vintage');
const lomoButton = document.getElementById('lomo');
const clarityButton = document.getElementById('clarity');
const embossButton = document.getElementById('emboss');
const pixelateButton = document.getElementById('pixelate');

let image = new Image();
let rotation = 0;
let flipHorizontal = 1;
let flipVertical = 1;

imageLoader.addEventListener('change', handleImage, false);
themeToggle.addEventListener('change', () => {
    document.body.classList.toggle('dark-mode');
});

function handleImage(e) {
    const reader = new FileReader();
    reader.onload = function(event) {
        image.onload = function() {
            resetAll();
            canvas.width = image.width;
            canvas.height = image.height;
            drawImage();
        }
        image.src = event.target.result;
    }
    reader.readAsDataURL(e.target.files[0]);
    fileNameDisplay.textContent = e.target.files[0].name;
}

function applyFilters() {
    const brightness = brightnessInput.value;
    const contrast = contrastInput.value;
    const grayscale = grayscaleInput.value;
    const blur = blurInput.value;
    const saturate = saturateInput.value;
    const hueRotate = hueRotateInput.value;
    const invert = invertInput.value;
    const sepia = sepiaInput.value;
    const opacity = opacityInput.value;

    return `
        brightness(${100 + parseInt(brightness)}%)
        contrast(${100 + parseInt(contrast)}%)
        grayscale(${grayscale}%)
        blur(${blur}px)
        saturate(${saturate}%)
        hue-rotate(${hueRotate}deg)
        invert(${invert}%)
        sepia(${sepia}%)
        opacity(${opacity}%)
    `;
}

function drawImage() {
    if (!image.src) return;

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.save();

    ctx.filter = applyFilters();

    // Apply transformations
    ctx.translate(canvas.width / 2, canvas.height / 2);
    ctx.rotate(rotation * Math.PI / 180);
    ctx.scale(flipHorizontal, flipVertical);

    ctx.drawImage(image, -image.width / 2, -image.height / 2);
    ctx.restore();
}

function resetAll() {
    resetFilters();
    rotation = 0;
    flipHorizontal = 1;
    flipVertical = 1;
    if (image.src) {
        drawImage();
    }
}

function resetFilters() {
    brightnessInput.value = 0;
    contrastInput.value = 0;
    grayscaleInput.value = 0;
    blurInput.value = 0;
    saturateInput.value = 100;
    hueRotateInput.value = 0;
    invertInput.value = 0;
    sepiaInput.value = 0;
    opacityInput.value = 100;
    drawImage();
}

[brightnessInput, contrastInput, grayscaleInput, blurInput, saturateInput, hueRotateInput, invertInput, sepiaInput, opacityInput].forEach(input => {
    input.addEventListener('input', drawImage);
});

resetButton.addEventListener('click', resetAll);

rotateLeftButton.addEventListener('click', () => {
    rotation -= 90;
    drawImage();
});

rotateRightButton.addEventListener('click', () => {
    rotation += 90;
    drawImage();
});

flipHorizontalButton.addEventListener('click', () => {
    flipHorizontal *= -1;
    drawImage();
});

flipVerticalButton.addEventListener('click', () => {
    flipVertical *= -1;
    drawImage();
});

saveButton.addEventListener('click', () => {
    const link = document.createElement('a');
    link.download = 'edited-image.png';
    link.href = canvas.toDataURL();
    link.click();
});

const loadingOverlay = document.getElementById('loading-overlay');
const worker = new Worker('worker.js');

// Custom Filters

vintageButton.addEventListener('click', () => {
    resetFilters();
    sepiaInput.value = 100;
    brightnessInput.value = -10;
    contrastInput.value = 10;
    drawImage();
});

lomoButton.addEventListener('click', () => {
    resetFilters();
    ctx.filter = 'none'; // Clear existing filters
    drawImage(); // Redraw the base image

    const gradient = ctx.createRadialGradient(
        canvas.width / 2, canvas.height / 2, canvas.width / 3,
        canvas.width / 2, canvas.height / 2, canvas.width / 1.5
    );
    gradient.addColorStop(0, 'rgba(0,0,0,0)');
    gradient.addColorStop(1, 'rgba(0,0,0,0.6)');
    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, canvas.width, canvas.height);
});

clarityButton.addEventListener('click', () => {
    const kernel = [
        0, -1, 0,
        -1, 5, -1,
        0, -1, 0
    ];
    applyConvolution(kernel);
});

embossButton.addEventListener('click', () => {
    const kernel = [
        -2, -1, 0,
        -1, 1, 1,
        0, 1, 2
    ];
    applyConvolution(kernel);
});

pixelateButton.addEventListener('click', () => {
    const size = 10;
    const w = canvas.width / size;
    const h = canvas.height / size;

    ctx.drawImage(canvas, 0, 0, w, h);
    ctx.imageSmoothingEnabled = false;
    ctx.drawImage(canvas, 0, 0, w, h, 0, 0, canvas.width, canvas.height);
});

function applyConvolution(kernel) {
    showLoading();
    const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    worker.postMessage({ imageData, kernel });
}

worker.onmessage = function(e) {
    const imageData = e.data;
    ctx.putImageData(imageData, 0, 0);
    hideLoading();
};

function showLoading() {
    loadingOverlay.style.display = 'flex';
}

function hideLoading() {
    loadingOverlay.style.display = 'none';
}