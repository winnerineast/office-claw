const path = require('path');
const fs = require('fs');
const http = require('http');

console.log("OfficeClaw Agent starting...");
const USER_ID = process.env.USER_ID || 'unknown';
console.log(`User ID: ${USER_ID}`);

const bridgePath = path.join(__dirname, '..', 'lib', 'security-bridge.node');
const security = require(bridgePath);

let runtimeInitialized = false;

security.initRuntime().then(() => {
  console.log("WASM Runtime initialized.");
  runtimeInitialized = true;
}).catch(err => {
  console.error("WASM Init Failed:", err);
});

// Simple HTTP server for POC testing
const server = http.createServer(async (req, res) => {
  if (req.url === '/summarize' && req.method === 'POST') {
    let body = '';
    req.on('data', chunk => { body += chunk.toString(); });
    req.on('end', async () => {
      try {
        if (!runtimeInitialized) throw new Error("Runtime not ready");
        
        const skillsDir = path.join(__dirname, '..', 'skills');
        const skillFiles = fs.readdirSync(skillsDir).filter(f => f.endsWith('.wasm'));
        if (skillFiles.length === 0) throw new Error(`No skills found in ${skillsDir}`);
        
        const wasmPath = path.join(skillsDir, skillFiles[0]); // Load the first available skill
        const wasmBuffer = fs.readFileSync(wasmPath);
        
        console.log(`Executing skill: ${wasmPath}`);
        const start = Date.now();
        const result = await security.executeSkill("summarizer", new Uint8Array(wasmBuffer), JSON.stringify(body));
        const duration = Date.now() - start;

        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ user: USER_ID, result: JSON.parse(result), durationMs: duration }));
      } catch (err) {
        res.writeHead(500);
        res.end(err.message);
      }
    });
  } else {
    res.writeHead(404);
    res.end();
  }
});

server.listen(3000, () => {
  console.log(`Agent for ${USER_ID} listening on port 3000`);
});
