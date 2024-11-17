import dotenv from 'dotenv';
dotenv.config();

import init, { productivityFunction } from './pkg/my_rust_lib.js';

const runWasmApp = async () => {
  await init();

  const API_KEY = process.env.API_KEY;
  if (!API_KEY) {
    console.error('Please define API_KEY in your .env file');
    return;
  }

  const result = productivityFunction();
  console.log('Result from Rust:', result);
};

runWasmApp().catch(console.error);