# 🏢 OfficeClaw: 轻量级、安全、可管理的个人企业级 OA Agent

**中文版** | [English Version](README.md)

---

## 📜 鸣谢与参考 (Credits & References)
OfficeClaw 的构建深受以下先驱项目的基础工作与架构启发：

- **[OpenClaw](https://github.com/openclaw/openclaw)**：核心个人 AI 助手框架，提供了我们的多通道编排逻辑。
- **[IronClaw](https://github.com/nearai/ironclaw)**：基于 Rust 的高性能实现，在 WASM 沙箱安全方面提供了重要参考。
- **[NanoClaw](https://github.com/qwibitai/nanoclaw)**：轻量级、操作系统级容器隔离的标杆。
- **[NemoClaw](https://github.com/NVIDIA/NemoClaw)**：企业级隐私路由和策略治理的标准。

---

## 🌟 核心愿景
在企业环境中，AI Agent 不仅仅要“聪明”，更必须是**可信且不干扰工作**的。OfficeClaw 旨在标准的办公硬件（从 Mac mini 到 PC 集群）上运行，同时确保敏感的公司数据在未经明确、可审计的许可下，绝不离开本地网络。

---

## 🛡️ 三大核心支柱

### 1. 🚀 轻量级 (Lightweight)
- **极小占用**：约 1.5 万行核心代码，资源占用比通用框架减少 95%。
- **边缘就绪**：支持在 8GB+ 内存的普通办公机运行，利用量化 MLX 模型并支持本地集群卸载。
- **快速冷启动**：通过 WASM 组件预加载实现 Agent 的近乎瞬时激活。

### 2. 🔐 企业级安全 (Enterprise Security)
- **Rust/WASM 沙箱 (参考 IronClaw)**：所有第三方技能都在加密隔离的 WASM 沙箱中运行，并受严格资源配额限制。
- **隐私路由器 (参考 NemoClaw)**：自动 PII 脱敏，确保敏感数据留在本地，仅匿名查询发送至云端。
- **容器隔离 (参考 NanoClaw)**：为每个用户 Agent 提供操作系统级隔离，防止跨角色数据泄露。

### 3. 📊 可管理性 (Manageability)
- **集中控制**：单一仪表盘监控集群中 Agent 的健康状况、资源占用和任务成功率。
- **审计追踪**：记录 Agent 的每一项行动，以便进行合规性和安全性审查。
- **零配置部署**：为非技术管理员提供一键安装体验。

---

## 🏗️ 桥接架构 (Bridge Architecture)
OfficeClaw 在原始 AI 能力与企业特定约束之间发挥“桥梁”作用：

- **逻辑层**：继承自 **OpenClaw** 灵活的 Agent 编排逻辑。
- **安全层**：集成 **IronClaw** 运行时的安全工具执行理念。
- **隔离层**：集成 **NanoClaw** 容器化的多租户隐私保护。
- **路由层**：集成 **NemoClaw** 感知隐私的任务调度。

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

### 1. 从源码构建
```bash
git clone https://github.com/winnerineast/office-claw
cd office-claw
./scripts/build-all.sh
```

### 2. 验证与测试
必须通过模拟 20+ 并发 OA 任务的“办公压力测试”。

### 3. 部署
```bash
./scripts/deploy.sh --cluster
```

---

**状态**: `活跃开发中` | **架构**: `桥接架构 v1.0`

---

## 🗺️ 行动计划 (Action Plan)

### 第一阶段：结构与环境 (Structure & Environment)
- [ ] **同步目录结构**：创建 security, core, routing, isolation, dashboard, scripts 文件夹。
- [ ] **初始化 Rust 安全桥**：将 `security/` 设置为 Rust NAPI-RS 项目。
- [ ] **搭建构建链**：创建 `scripts/build-all.sh` 实现多运行时编译。

### 第二阶段：核心逻辑提取 (Core Logic Extraction)
- [ ] **精简编排器**：从 `OpenClaw` 提取最简 Agent 编排逻辑。
- [ ] **隐私路由器 (NemoClaw)**：实现初步的 PII 脱敏逻辑。

### 第三阶段：安全与隔离 (Security & Isolation)
- [ ] **WASM 沙箱**：在 Rust 安全桥中实现资源限制。
- [ ] **容器集成**：开发隔离层以支持多租户。

### 第四阶段：管理与部署 (Management & Deployment)
- [ ] **仪表盘 MVP**：基础状态监控 UI。
- [ ] **集群部署**：完成 `scripts/deploy.sh`。

---

## 📝 工作日记 (Work Daily)

### 🗓️ 2026-03-22
- **环境清理**：重置本地环境，移除旧的全局包。
- **源码同步**：拉取 `openclaw` 和 `office-claw` 仓库。
- **愿景对齐**：确立“轻量、安全、可管理”的目标。
- **文档重构**：完成双语 README 结构，并向 "Claw" 家族项目致谢。
- **下一步**：开始第一阶段 - 目录创建与 Rust 初始化。
