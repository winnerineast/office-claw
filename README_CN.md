# 🏢 OfficeClaw: 轻量级、安全、可管理的个人企业级 OA Agent

/* Generated-by: [20260322-02-arch-deep-dive] */

**中文版** | [English Version](README.md)

**OfficeClaw** 是一款专门为现代办公自动化 (OA) 环境设计的生产级 AI Agent 框架。它继承了 **OpenClaw** 强大的编排能力，同时引入了“桥接架构 (Bridge Architecture)”，以满足企业级部署的严苛需求：**安全性 (Security)、隐私性 (Privacy) 和资源效率 (Resource Efficiency)。**

---

## 🛡️ 三大核心支柱
1. **🚀 轻量级 (参考 ZeroClaw/PicoClaw)**：约 1.5 万行核心代码，<10MB 待机内存。
2. **🔐 企业级安全 (参考 IronClaw/NanoClaw)**：基于 WASM 的沙箱执行、OS 级容器隔离以及 **mTLS** 加密通信。
3. **📊 集群治理 (参考 NemoClaw)**：**拉取式 (Pull-based)** 集中管控，实现远程部署、账单配额控制、合规审计。

---

## 🏗️ 桥接架构 v1.2 (深化版)
OfficeClaw 在原始 AI 能力与企业特定约束之间发挥“桥梁”作用：
- **洋葱中间件流水线 (Onion Middleware Pipeline)**：ACP 消息流经 6 层处理（入站 -> 集群钩子 -> PII 脱敏 -> 推理 -> WASM 验证 -> 出站）。
- **安全层**：集成 **IronClaw/ZeroClaw** 的高性能 FFI (NAPI-RS) 安全执行理念。
- **隔离层**：集成 **NanoClaw** 的微虚拟机/容器化多租户隐私保护。
- **治理层**：集成 **NemoClaw** 与自定义**拉取式集群管理系统**，实现远程编排与加密签名审计。

---

## 📂 项目结构
```text
office-claw/
├── security/           # 基于 Rust 的 WASM 安全桥 (IronClaw/ZeroClaw)
├── core/               # 企业级编排器 (基于 OpenClaw 逻辑)
├── routing/            # 隐私与模型路由器 (NemoClaw)
├── isolation/          # 容器管理层 (NanoClaw)
├── fleet/              # 集群管理与控制平面
├── dashboard/          # 管理 UI 与审计日志
├── prompts/tasks/      # 任务型生成提示词 (提示词优先规范)
└── scripts/            # 部署与维护工具
```

---
**状态**: `活跃开发中` | **架构**: `桥接架构 v1.2 (深化版)`
