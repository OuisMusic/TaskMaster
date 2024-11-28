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

  const tasksRaw = process.env.TASKS || "";
  const tasks = tasksRaw.split(",").map(task => task.trim()).filter(task => task);

  if (tasks.length === 0) {
    console.error('Please define at least one task in your .env file using the TASKS variable, separated by commas for multiple tasks.');
    return;
  }

  tasks.forEach(task => {
    const result = productivityFunction(task);
    console.log(`Result from Rust for task "${task}":`, result);
  });
};

runWasmApp().catch(console.error);