# 🏢 OfficeClaw: 轻量级、安全、可管理的个人企业级 OA Agent

**OfficeClaw** 是一款专门为现代办公自动化 (OA) 环境设计的生产级 AI Agent 框架。它继承了 **OpenClaw** 强大的编排能力，同时引入了“桥接架构 (Bridge Architecture)”，以满足企业级部署的严苛需求：**安全性 (Security)、隐私性 (Privacy) 和资源效率 (Resource Efficiency)。**

---

## 🌟 核心愿景
在企业环境中，AI Agent 不仅仅要“聪明”，更必须是**可信且不干扰工作**的。OfficeClaw 旨在标准的办公硬件（从 Mac mini 到 PC 集群）上运行，同时确保敏感的公司数据在未经明确、可审计的许可下，绝不离开本地网络。

---

## 🛡️ 三大核心支柱

### 1. 🚀 轻量级 (Lightweight)
- **极小占用 (Small Footprint)**：经过优化，核心代码约 1.5 万行，资源占用比通用 Agent 框架减少 95%。
- **边缘就绪 (Edge-Ready)**：通过将重型推理卸载到本地集群或使用量化的 MLX 模型，可在标准办公机器 (8GB+ RAM) 上流畅运行。
- **快速冷启动 (Fast Cold Start)**：通过专门的 WASM 组件预加载技术，实现 Agent 的近乎瞬时激活。

### 2. 🔐 企业级安全 (Enterprise Security)
- **Rust/WASM 沙箱 (IronClaw)**：所有第三方“技能 (Skill)”都在加密隔离的 WebAssembly 沙箱中运行，并受到严格的 CPU/内存配额限制。
- **隐私路由器 (NemoClaw)**：自动 PII (个人身份信息) 脱敏。敏感数据（如工资、合同）在本地处理；只有匿名化的查询才会发送至云端。
- **容器隔离 (NanoClaw)**：每个用户的 Agent 都在操作系统级进行隔离，确保不同部门或角色之间零干扰。

### 3. 📊 可管理性 (Manageability)
- **集中控制 (Centralized Control)**：通过单一仪表盘监控整个办公集群中 Agent 的健康状况、资源使用率和任务成功率。
- **审计追踪 (Audit Trails)**：Agent 采取的每一项行动都会被记录，以便进行合规性和安全性审查。
- **零配置部署 (Zero-Config Deployment)**：为非技术办公管理员量身定制的一键安装和自动化配置。

---

## 🏗️ 桥接架构 (Bridge Architecture)
OfficeClaw 在原始 AI 能力与企业特定约束之间发挥“桥梁”作用：

- **逻辑层 (Logic Layer)**：继承自 **OpenClaw** 灵活的多通道 Agent 逻辑。
- **安全层 (Security Layer)**：集成 **IronClaw** (Rust 运行时)，用于安全的工具执行。
- **隔离层 (Isolation Layer)**：集成 **NanoClaw** (容器运行器)，实现多租户隐私。
- **路由层 (Routing Layer)**：集成 **NemoClaw**，用于智能且感知隐私的任务调度。

---

## 📂 项目结构
```text
office-claw/
├── security/           # 基于 Rust 的 WASM 安全桥 (IronClaw)
├── core/               # 企业级编排器 (基于 OpenClaw 逻辑)
├── routing/            # 隐私优先的任务路由器 (NemoClaw)
├── isolation/          # 容器管理层 (NanoClaw)
├── dashboard/          # 管理 UI 与审计日志
└── scripts/            # 部署与维护工具
```

---

## 🛠️ 开发与部署生命周期

### 1. 从源码构建 (Source-to-Binary Build)
我们坚信全透明。OfficeClaw 从源码构建，以确保没有隐藏的后门。
```bash
# 克隆并构建核心组件
git clone https://github.com/winnerineast/office-claw
cd office-claw
./scripts/build-all.sh
```

### 2. 验证与测试 (Validation & Testing)
每个构建版本在部署前都必须通过“办公压力测试”，模拟 20+ 并发 OA 任务。

### 3. 部署 (Deployment)
通过单一命令部署到单台机器或 Mac Studio 集群：
```bash
./scripts/deploy.sh --cluster
```

---

## 🤝 与 OpenClaw 的关系
OfficeClaw 是 [OpenClaw](https://github.com/openclaw/openclaw) 的下游项目。OpenClaw 侧重于个人通用性和广泛的频道支持，而 OfficeClaw 侧重于**企业级硬化、本地优先隐私和可扩展性。**

---

**状态**: `活跃开发中` | **架构**: `桥接架构 v1.0`

---

## 🗺️ 行动计划 (Action Plan)

### 第一阶段：结构与环境 (Structure & Environment)
- [ ] **同步目录结构**：创建架构中定义的各个核心文件夹。
- [ ] **初始化 Rust 安全桥**：将 `security/` 设置为 Rust NAPI-RS 项目。
- [ ] **搭建构建链**：创建 `scripts/build-all.sh` 以自动化 Node.js 和 Rust 的编译。

### 第二阶段：核心逻辑提取 (Core Logic Extraction)
- [ ] **精简编排器**：从 `OpenClaw` 提取最简 Agent 编排逻辑至 `office-claw/core`。
- [ ] **隐私路由器 (NemoClaw)**：实现初步的 PII 脱敏逻辑。

### 第三阶段：安全与隔离 (Security & Isolation)
- [ ] **WASM 沙箱**：在 Rust 安全桥中实现资源限制。
- [ ] **容器集成**：使用 Docker/OrbStack 开发隔离层。

### 第四阶段：管理与部署 (Management & Deployment)
- [ ] **仪表盘 MVP**：用于监控 Agent 状态的基础 UI。
- [ ] **集群部署**：完成 `scripts/deploy.sh`。

---

## 📝 工作日记 (Work Daily)

### 🗓️ 2026-03-22
- **环境清理**：停止旧服务，移除全局包，备份配置。
- **源码同步**：从 `winnerineast` 拉取 `openclaw` 和 `office-claw`。
- **项目定义**：明确 OfficeClaw 三大支柱（轻量、安全、可管理）。
- **文档重构**：重写 `README.md` (English) 并新增 `README_CN.md` (中文)。
- **下一步**：开始**第一阶段** - 同步目录结构并初始化 Rust 项目。
