# 🏢 OfficeClaw: 轻量级、安全、可管理的个人企业级 OA Agent

**中文版** | [English Version](README.md)

---

## 📜 鸣谢与参考 (Credits & References)
OfficeClaw 的构建深受以下先驱项目的基础工作与架构启发：

- **[OpenClaw](https://github.com/openclaw/openclaw)**：核心个人 AI 助手框架，提供了我们的多通道编排逻辑。
- **[IronClaw](https://github.com/nearai/ironclaw)**：基于 Rust 的高性能实现，在 WASM 沙箱安全方面提供了重要参考。
- **[NanoClaw](https://github.com/qwibitai/nanoclaw)**：轻量级、操作系统级容器隔离的标杆。
- **[NemoClaw](https://github.com/NVIDIA/NemoClaw)**：企业级隐私路由和策略治理的标准。
- **[ZeroClaw](https://github.com/winnerineast/openclaw)**：超快启动 (<10ms) 和极小静态二进制体积的基准。
- **[PicoClaw](https://github.com/winnerineast/openclaw)**：边缘硬件低内存效率 (~1MB 占用) 的金标准。

---

## 🌟 核心愿景
在企业环境中，AI Agent 不仅仅要“聪明”，更必须是**可信且不干扰工作**的。OfficeClaw 旨在标准的办公硬件上运行，同时确保敏感的公司数据在未经明确、可审计的许可下，绝不离开本地网络。

---

## 🛡️ 三大核心支柱

### 1. 🚀 轻量级 (参考 ZeroClaw/PicoClaw)
- **极小占用**：约 1.5 万行核心代码，资源占用比通用框架减少 95%。
- **快速冷启动**：通过 WASM 组件预加载实现近乎瞬时激活。

### 2. 🔐 企业级安全 (参考 IronClaw/NanoClaw)
- **WASM 沙箱**：所有技能都在加密隔离的沙箱中运行。
- **零信任认证**：强制执行 Token 认证，杜绝“ClawJacked”类漏洞攻击。

### 3. 📊 集群管理与治理 (参考 NemoClaw)
- **集中控制平面 (Control Plane)**：远程部署、更新和监控办公网络中的所有节点。
- **账单与配额控制**：统一管理 LLM 成本，实现部门级的预算追踪。
- **合规审计**：为每一项 Agent 行动提供加密签名的审计日志，满足 SOC2/GDPR 要求。

---

## 🏗️ 桥接架构 (Bridge Architecture)
OfficeClaw 在原始 AI 能力与企业特定约束之间发挥“桥梁”作用：

- **逻辑层**：继承自 **OpenClaw** 灵活的 Agent 编排逻辑。
- **安全层**：集成 **IronClaw/ZeroClaw** 运行时的安全与极速执行理念。
- **隔离层**：集成 **NanoClaw** 容器化的多租户隐私保护。
- **路由与治理层**：集成 **NemoClaw** 与自定义集群管理系统，实现隐私保护、模型路由和集中化管控。

---

## 📂 项目结构
```text
office-claw/
├── security/           # 基于 Rust 的 WASM 安全桥 (IronClaw/ZeroClaw)
├── core/               # 企业级编排器 (基于 OpenClaw/Nanoclaw)
├── routing/            # 隐私与模型路由器 (NemoClaw)
├── isolation/          # 容器管理层 (NanoClaw)
├── fleet/              # 集群管理与控制平面
├── dashboard/          # 管理 UI 与审计日志
└── scripts/            # 部署与维护工具
```

---

**状态**: `活跃开发中` | **架构**: `桥接架构 v1.2`

---

## 📝 工作日记 (Work Daily)

### 🗓️ 2026-03-22
- **环境清理**：重置本地环境，移除旧的全局包。
- **源码同步**：拉取 `openclaw` 和 `office-claw` 仓库。
- **愿景对齐**：确立“轻量、安全、可管理”的目标。
- **架构同步更新**：在所有文档 (README, README_CN, Design) 中同步加入了**集群管理与治理 (Fleet Management & Governance)** 层，明确了远程部署、账单控制和合规审计的设计要求。
- **下一步**：开始第一阶段 - 目录创建与 Rust 初始化。
