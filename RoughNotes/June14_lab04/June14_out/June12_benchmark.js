let wasmReady = false;
let _matMul, _allocMatrix, _freeMatrix;

Module.onRuntimeInitialized = () => {
  _matMul     = Module.cwrap('matrix_multiply', null,     ['number','number','number','number']);
  _allocMatrix= Module.cwrap('alloc_matrix',    'number', ['number']);
  _freeMatrix = Module.cwrap('free_matrix',     null,     ['number']);
  wasmReady   = true;
  console.log("✅ WASM Matrix module ready");
};

// Pure JavaScript matrix multiply
function jsMatMul(A, B, N) {
  const C = new Float32Array(N * N);
  for (let i = 0; i < N; i++)
    for (let j = 0; j < N; j++) {
      let sum = 0;
      for (let k = 0; k < N; k++)
        sum += A[i * N + k] * B[k * N + j];
      C[i * N + j] = sum;
    }
  return C;
}

function randomMatrix(N) {
  const m = new Float32Array(N * N);
  for (let i = 0; i < m.length; i++) m[i] = Math.random();
  return m;
}

async function runBenchmark() {
  if (!wasmReady) { alert("WASM module not ready yet!"); return; }

  const N = parseInt(document.getElementById('size').value);
  const A = randomMatrix(N);
  const B = randomMatrix(N);

  // ---- WASM Benchmark ----
  const pA = _allocMatrix(N);
  const pB = _allocMatrix(N);
  const pC = _allocMatrix(N);
  Module.HEAPF32.set(A, pA >> 2);
  Module.HEAPF32.set(B, pB >> 2);

  const t0 = performance.now();
  _matMul(pA, pB, pC, N);
  const wasmTime = performance.now() - t0;

  _freeMatrix(pA); _freeMatrix(pB); _freeMatrix(pC);

  // ---- JS Benchmark ----
  const t1 = performance.now();
  jsMatMul(A, B, N);
  const jsTime = performance.now() - t1;

  // ---- Display Results ----
  document.getElementById('results').style.display = 'block';
  document.getElementById('r-size').textContent  = `${N} × ${N}`;
  document.getElementById('r-wasm').textContent  = `${wasmTime.toFixed(2)} ms`;
  document.getElementById('r-js').textContent    = `${jsTime.toFixed(2)} ms`;
  const speedup = (jsTime / wasmTime).toFixed(2);
  const winner = wasmTime < jsTime
    ? `🏆 WASM is ${speedup}x faster than JavaScript!`
    : `JS was faster this time (JIT warmup may have helped).`;
  document.getElementById('r-winner').textContent = winner;
}
