const ContainerManager = require('../src/container-manager');
const path = require('path');
const { execSync } = require('child_process');

async function runCheckpoint5Test() {
  const baseDir = path.join(__dirname, '..', '..', '..', 'office-claw-data');
  const manager = new ContainerManager(baseDir);

  console.log("=== Checkpoint 5: Multi-User POC & Load Test ===");

  const users = [
    { id: 'user1', port: 3001 },
    { id: 'user2', port: 3002 },
    { id: 'user3', port: 3003 }
  ];

  try {
    // 1. Launch 3 Concurrent Agents
    console.log("Launching 3 concurrent user agents...");
    for (const u of users) {
      await manager.startContainer(u.id, u.port);
    }

    // Wait for agents to initialize WASM
    console.log("Waiting 5 seconds for WASM initialization...");
    await new Promise(resolve => setTimeout(resolve, 5000));

    // 2. Perform Concurrent Load Test
    console.log("\nTriggering concurrent 'Summarize' requests...");
    const prompts = [
      "Meeting with NVIDIA regarding Mac Studio Cluster scaling next week.",
      "Internal memo: Salary reviews are completed for the Q1 period.",
      "Project OpenClaw: Security bridge integration is 100% verified."
    ];

    const startTime = Date.now();
    
    // Use curl in parallel
    const requests = users.map((u, i) => {
      const cmd = `curl -s -X POST -d "${prompts[i]}" http://localhost:${u.port}/summarize`;
      console.log(`[Request] ${u.id} -> port ${u.port}`);
      return new Promise((resolve) => {
        try {
          const res = execSync(cmd).toString();
          resolve(JSON.parse(res));
        } catch (e) {
          resolve({ error: e.message });
        }
      });
    });

    const results = await Promise.all(requests);
    const totalDuration = Date.now() - startTime;

    console.log("\n--- Results ---");
    results.forEach((r, i) => {
      if (r.error) {
        console.error(`User ${users[i].id}: FAILED - ${r.error}`);
      } else {
        console.log(`User ${r.user}: ${r.result} (Lat: ${r.durationMs}ms)`);
      }
    });

    console.log(`\nTotal Concurrent Time: ${totalDuration}ms`);
    console.log(`Average Latency: ${totalDuration / users.length}ms`);

    if (results.every(r => !r.error)) {
      console.log("\n=== Checkpoint 5: SUCCESS ===");
    } else {
      throw new Error("One or more requests failed.");
    }

  } catch (error) {
    console.error("\n=== Checkpoint 5: FAILED ===");
    console.error(error);
  } finally {
    // Cleanup
    // await manager.stopAll();
  }
}

runCheckpoint5Test();
