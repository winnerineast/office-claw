const ContainerManager = require('../src/container-manager');
const path = require('path');
const { execSync } = require('child_process');

async function runCheckpoint2Test() {
  const baseDir = path.join(__dirname, '..', '..', '..', 'office-claw-data');
  const manager = new ContainerManager(baseDir);

  console.log("=== Checkpoint 2: Container Isolation Test ===");

  const users = ['user1', 'user2', 'user3'];
  const containerIds = [];

  try {
    // 1. Start 3 Users
    for (const userId of users) {
      console.log(`Starting container for ${userId}...`);
      const id = await manager.startContainer(userId);
      containerIds.push(id);
      
      const status = await manager.getStatus(userId);
      console.log(`User ${userId} status: ${status}`);
    }

    // 2. Verify Isolation: List containers and their mounts
    console.log("\n--- Verifying Isolation ---");
    for (const userId of users) {
      const inspect = execSync(`docker inspect office-claw-user-${userId} --format '{{json .Mounts}}'`).toString();
      const mounts = JSON.parse(inspect);
      console.log(`User ${userId} mounts:`, mounts.map(m => m.Source));
      
      // Ensure User 1 cannot see User 2's mount
      if (userId === 'user1') {
        const u2Memory = path.join(baseDir, 'users', 'user2', 'memory');
        if (mounts.some(m => m.Source === u2Memory)) {
          throw new Error("ISOLATION BREACH: User 1 has access to User 2's memory!");
        }
      }
    }

    // 3. Verify Resource Limits
    console.log("\n--- Verifying Resource Limits ---");
    const limits = execSync(`docker inspect office-claw-user-user1 --format '{{.HostConfig.Memory}} {{.HostConfig.NanoCpus}}'`).toString();
    const [memory, cpus] = limits.trim().split(' ');
    console.log(`User user1 - Memory Limit: ${memory} bytes, CPU Limit: ${cpus / 1e9} vCPU`);
    
    if (memory !== '2147483648') { // 2GB
       throw new Error(`Resource limit mismatch: Expected 2GB, got ${memory}`);
    }

    console.log("\n=== Checkpoint 2: SUCCESS ===");
  } catch (error) {
    console.error("\n=== Checkpoint 2: FAILED ===");
    console.error(error);
  } finally {
    // Cleanup for POC testing (optional, keeping them alive for now as per plan)
    // for (const userId of users) {
    //   await manager.stopContainer(userId);
    // }
  }
}

runCheckpoint2Test();
