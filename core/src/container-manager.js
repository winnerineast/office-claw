const { exec } = require('child_process');
const fs = require('fs');
const path = require('path');
const util = require('util');
const execPromise = util.promisify(exec);

class ContainerManager {
  constructor(baseDataDir) {
    this.baseDataDir = baseDataDir;
    this.imageName = 'office-claw-agent:latest';
  }

  async setupUserDirs(userId) {
    const userDir = path.join(this.baseDataDir, 'users', userId);
    const memoryDir = path.join(userDir, 'memory');
    const soulDir = path.join(userDir, 'soul');

    if (!fs.existsSync(memoryDir)) fs.mkdirSync(memoryDir, { recursive: true });
    if (!fs.existsSync(soulDir)) fs.mkdirSync(soulDir, { recursive: true });

    return { memoryDir, soulDir };
  }

  async startContainer(userId, hostPort, limits = { cpu: 1, memory: '2g' }) {
    const { memoryDir, soulDir } = await this.setupUserDirs(userId);
    const containerName = `office-claw-user-${userId}`;

    const dockerCmd = [
      'docker run -d',
      `--name ${containerName}`,
      `-p ${hostPort}:3000`,
      `-e USER_ID=${userId}`,
      `--memory ${limits.memory}`,
      `--cpus ${limits.cpu}`,
      `-v "${memoryDir}:/app/data/memory"`,
      `-v "${soulDir}:/app/data/soul"`,
      '--restart on-failure',
      this.imageName
    ].join(' ');

    try {
      const { stdout } = await execPromise(dockerCmd);
      return stdout.trim();
    } catch (error) {
      if (error.message.includes('already in use')) {
        await execPromise(`docker rm -f ${containerName}`);
        return this.startContainer(userId, hostPort, limits);
      }
      throw error;
    }
  }

  async stopAll() {
    try {
      await execPromise('docker ps -a --filter "name=office-claw-user-" -q | xargs docker rm -f');
    } catch (e) {}
  }
}

module.exports = ContainerManager;
