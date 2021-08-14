import init, {
    collatz,
    collatz_next_number
  } from "./pkg/collatz_wasm.js";
  
  init().then(() => {
    const list = collatz(4);
    alert(list);

  });