# Stage 1: Build the Rust Security Bridge (NAPI-RS)
FROM rust:slim-bookworm AS rust-builder
WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl python3 make g++ tar && rm -rf /var/lib/apt/lists/*

# Install Node.js and NAPI-RS CLI
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && apt-get install -y nodejs
RUN npm install -g @napi-rs/cli

COPY security/ .
RUN napi build --release --platform

# Stage 2: Final Agent Image
FROM node:22-bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Create required directories
RUN mkdir -p /app/lib /app/skills

# Copy the compiled NAPI binary
COPY --from=rust-builder /build/office-claw-security.*.node /app/lib/security-bridge.node

# Copy pre-built skills from host (Standardized Path)
COPY skills/dist/*.wasm /app/skills/

# Copy Core Orchestrator code
COPY core/package*.json ./
RUN npm install --production
COPY core/ .

RUN groupadd -r officeagent && useradd -r -g officeagent officeagent && \
    chown -R officeagent:officeagent /app
USER officeagent

CMD ["node", "src/index.js"]
