# 🏢 OfficeClaw: 轻量级、安全、可管理的个人企业级 OA Agent

/* Generated-by: [20260322-01-project-init] */

**中文版** | [English Version](README.md)

**OfficeClaw** 是一款专门为现代办公自动化 (OA) 环境设计的生产级 AI Agent 框架。它继承了 **OpenClaw** 强大的编排能力，同时引入了“桥接架构 (Bridge Architecture)”，以满足企业级部署的严苛需求：**安全性 (Security)、隐私性 (Privacy) 和资源效率 (Resource Efficiency)。**

---

## 🛡️ 三大核心支柱
1. **🚀 轻量级 (参考 ZeroClaw/PicoClaw)**：约 1.5 万行核心代码，资源占用减少 95%。
2. **🔐 企业级安全 (参考 IronClaw/NanoClaw)**：基于 WASM 的沙箱执行与操作系统级容器隔离。
3. **📊 集群治理 (参考 NemoClaw)**：远程部署、账单配额控制、合规审计。

---

## 🏗️ 桥接架构 v1.2
OfficeClaw 在原始 AI 能力与企业特定约束之间发挥“桥梁”作用：
- **逻辑层**：继承自 **OpenClaw** 灵活的编排逻辑。
- **安全层**：集成 **IronClaw/ZeroClaw** 的安全与极速执行理念。
- **隔离层**：集成 **NanoClaw** 的容器化多租户隐私保护。
- **治理层**：集成 **NemoClaw** 与自定义集群管理系统，实现集中管控。

---

## 🛠️ 编码规范：提示词优先 (Prompt-First)
OfficeClaw 的每一行代码都源自经过版本追踪的提示词。我们坚持“优先修复提示词，而非直接修改代码”的原则。
- **任务型提示词**：保存在 `prompts/tasks/` 中。
- **原子提交**：提示词文件与生成的代码必须在同一个 Git Commit 中提交。

---

## 📂 项目结构
```text
office-claw/
├── security/           # 基于 Rust 的 WASM 安全桥
├── core/               # 企业级编排器
├── routing/            # 隐私与模型路由器
├── isolation/          # 容器管理层
├── fleet/              # 集群管理与控制平面
├── dashboard/          # 管理 UI 与审计日志
├── prompts/tasks/      # 任务型生成提示词
└── scripts/            # 部署与维护工具
```

---
**状态**: `活跃开发中` | **架构**: `桥接架构 v1.2`
