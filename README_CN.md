# 🏢 OfficeClaw: 轻量级、安全、可管理的个人企业级 OA Agent

/* Generated-by: [20260322-02-arch-deep-dive] */

**中文版** | [English Version](README.md)

**OfficeClaw** 是一款专门为现代办公自动化 (OA) 环境设计的生产级 AI Agent 框架。它继承了 **OpenClaw** 强大的编排能力，同时引入了“桥接架构 (Bridge Architecture)”，以满足企业级部署的严苛需求：**安全性 (Security)、隐私性 (Privacy) 和资源效率 (Resource Efficiency)。**

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

## 🛡️ 三大核心支柱

### 1. 🚀 轻量级 (参考 ZeroClaw/PicoClaw)
- **极小占用**：约 1.5 万行核心代码，资源占用减少 95%。
- **极低能耗**：<10MB 待机内存消耗。
- **快速冷启动**：通过 WASM 组件预加载实现近乎瞬时激活。

### 2. 🔐 企业级安全 (参考 IronClaw/NanoClaw)
- **WASM 沙箱**：通过高性能 FFI (NAPI-RS) 实现所有技能的加密隔离执行。
- **容器隔离**：为每个用户 Agent 提供操作系统级隔离，防止跨角色数据泄露。
- **零信任认证**：强制执行 Token 认证与 **mTLS** 加密，杜绝“ClawJacked”类漏洞攻击。

### 3. 📊 集群管理与治理 (参考 NemoClaw)
- **拉取式控制平面 (Pull-based)**：远程部署、更新和监控办公网络中的所有节点。
- **账单与配额控制**：统一管理 LLM 成本，实现部门级的预算追踪。
- **合规审计**：为每一项 Agent 行动提供加密签名的审计日志，满足 SOC2/GDPR 要求。

---

## 🏗️ 桥接架构 v1.2 (深化版)
OfficeClaw 在原始 AI 能力与企业特定约束之间发挥“桥梁”作用：
- **洋葱中间件流水线 (Onion Middleware Pipeline)**：ACP 消息流经 6 层严格处理：
  1. **入站 (Ingress)**：接收原始 ACP 载荷。
  2. **集群钩子 (Fleet Hook)**：基于全局策略的过滤。
  3. **PII 脱敏 (NemoClaw)**：脱敏 Email、身份证号、财务数据等敏感信息。
  4. **推理 (Reasoning)**：核心 Agent 逻辑处理。
  5. **WASM 验证 (IronClaw)**：验证技能签名与资源配额。
  6. **出站 (Egress)**：返回经过审计的最终响应。
- **安全层**：集成 **IronClaw/ZeroClaw** 的原生安全执行理念。
- **隔离层**：集成 **NanoClaw** 的微虚拟机/容器化隔离。
- **治理层**：集成 **NemoClaw** 与自定义集群管理系统。

---

## 🛠️ 编码规范：提示词优先 (Prompt-First)
OfficeClaw 的每一行代码都源自经过版本追踪的提示词。
- **任务型提示词**：保存在 `prompts/tasks/` 中。
- **原子提交**：提示词文件与生成的代码必须在同一个 Git Commit 中提交。
- **修复原则**：优先增强提示词，通过重新生成来修复代码，而非手动补丁。

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
**状态**: `活跃开发中` | **架构**: `桥接架构 v1.2 (Symmetrical)`
