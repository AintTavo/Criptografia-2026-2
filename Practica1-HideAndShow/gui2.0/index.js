const logger = (id) => {
  const el = document.getElementById(id);
  return {
    clear: () => { el.innerHTML = ''; },
    log: (msg, cls = '') => {
      el.innerHTML += `<div class="log-line${cls ? ' log-' + cls : ''}">&gt; ${msg}</div>`;
      el.scrollTop = el.scrollHeight;
    },
  };
};

const logHide = logger('console-hide');
const logReveal = logger('console-reveal');

let wasmReady = false;
Module.onRuntimeInitialized = () => {
  wasmReady = true;
  logHide.log('WebAssembly loaded and ready.', 'ok');
  logReveal.log('WebAssembly loaded and ready.', 'ok');
};

function setupDropZone(zoneId, inputId, fnId, onFile) {
  const zone = document.getElementById(zoneId);
  const input = document.getElementById(inputId);
  const fnEl = document.getElementById(fnId);

  zone.addEventListener('click', () => input.click());

  input.addEventListener('change', () => {
    const f = input.files[0];
    if (!f) return;
    fnEl.textContent = f.name;
    fnEl.classList.add('show');
    zone.classList.add('has-file');
    if (onFile) onFile(f);
  });

  ['dragenter', 'dragover'].forEach(ev => {
    zone.addEventListener(ev, e => {
      e.preventDefault();
      e.stopPropagation();
      zone.classList.add('dragover');
    });
  });
  ['dragleave', 'drop'].forEach(ev => {
    zone.addEventListener(ev, e => {
      e.preventDefault();
      e.stopPropagation();
      zone.classList.remove('dragover');
    });
  });
  zone.addEventListener('drop', e => {
    const f = e.dataTransfer.files[0];
    if (!f) return;
    fnEl.textContent = f.name;
    fnEl.classList.add('show');
    zone.classList.add('has-file');
    const dt = new DataTransfer();
    dt.items.add(f);
    input.files = dt.files;
    input.dispatchEvent(new Event('change'));
    if (onFile) onFile(f);
  });
}

let bmpFile = null;
let txtFile = null;
let revealBmpFile = null;

setupDropZone('dz-bmp-hide', 'bmp-input-hide', 'fn-bmp-hide', f => { bmpFile = f; });
setupDropZone('dz-txt', 'txt-input', 'fn-txt', f => { txtFile = f; });
setupDropZone('dz-bmp-reveal', 'bmp-input-reveal', 'fn-bmp-reveal', f => { revealBmpFile = f; });

document.querySelectorAll('.src-btn').forEach(btn => {
  btn.addEventListener('click', () => {
    document.querySelectorAll('.src-btn').forEach(b => b.classList.remove('active'));
    btn.classList.add('active');
    const mode = btn.dataset.source;
    document.getElementById('src-file').classList.toggle('active', mode === 'file');
    document.getElementById('src-text').classList.toggle('active', mode === 'text');
  });
});

document.querySelectorAll('.mode-tab').forEach(tab => {
  tab.addEventListener('click', () => {
    document.querySelectorAll('.mode-tab').forEach(t => t.classList.remove('active'));
    tab.classList.add('active');
    const page = tab.dataset.page;
    document.querySelectorAll('.page').forEach(p => p.classList.remove('active'));
    document.getElementById('page-' + page).classList.add('active');
    document.body.classList.toggle('reveal', page === 'reveal');
  });
});

function readFileAsU8(file) {
  return new Promise((resolve, reject) => {
    const r = new FileReader();
    r.onload = () => resolve(new Uint8Array(r.result));
    r.onerror = reject;
    r.readAsArrayBuffer(file);
  });
}

document.getElementById('btn-hide').addEventListener('click', async () => {
  if (!wasmReady) return alert('WebAssembly has not finished loading.');

  let bmp, txtData;

  if (!bmpFile) return alert('Select a BMP image.');
  bmp = await readFileAsU8(bmpFile);

  const srcMode = document.querySelector('.src-btn.active').dataset.source;
  if (srcMode === 'file') {
    if (!txtFile) return alert('Select a .txt file or switch to "Text" mode.');
    txtData = await readFileAsU8(txtFile);
  } else {
    const text = document.getElementById('txt-textarea').value;
    if (!text.trim()) return alert('Write the message to hide.');
    const enc = new TextEncoder();
    txtData = enc.encode(text);
  }

  const baseName = bmpFile.name.replace(/\.bmp$/i, '');
  const outputName = baseName + '_h.bmp';

  logHide.clear();
  logHide.log('Writing files to virtual memory...');
  Module.FS.writeFile('original.bmp', bmp);
  Module.FS.writeFile('mensaje.txt', txtData);

  logHide.log('Running hide in C/WASM...');
  const status = Module.ccall('hide', 'number', ['string', 'string'], ['mensaje.txt', 'original.bmp']);

  if (status !== 0) {
    logHide.log('Error: code ' + status, 'err');
    return alert('Hide error (code ' + status + ')');
  }

  logHide.log('Hide successful.', 'ok');
  logHide.log('Reading result and downloading...');

  try {
    const resultData = Module.FS.readFile('output_secret.bmp');
    const blob = new Blob([resultData], { type: 'image/bmp' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = outputName;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
    logHide.log('Download: ' + outputName, 'ok');
  } catch (e) {
    logHide.log('Error reading output file.', 'err');
    console.error(e);
  }
});

document.getElementById('btn-reveal').addEventListener('click', async () => {
  if (!wasmReady) return alert('WebAssembly has not finished loading.');

  if (!revealBmpFile) return alert('Select a BMP image with a hidden message.');

  const bmp = await readFileAsU8(revealBmpFile);

  logReveal.clear();
  logReveal.log('Writing image to virtual memory...');
  Module.FS.writeFile('secret_to_read.bmp', bmp);

  logReveal.log('Extracting hidden message...');
  const secreto = Module.ccall('show', 'string', ['string'], ['secret_to_read.bmp']);

  const resultBox = document.getElementById('result-box');
  const resultText = document.getElementById('result-text');

  if (secreto && secreto.length > 0 && !secreto.startsWith('El archivo no') && !secreto.startsWith('Archivo vacio')) {
    resultText.textContent = secreto;
    resultBox.classList.add('show');
    logReveal.log('Message revealed successfully.', 'ok');
  } else {
    resultText.textContent = '(no hidden message found)';
    resultBox.classList.add('show');
    logReveal.log(secreto || 'No message found.', 'err');
  }
});
