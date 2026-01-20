# AI Models & API Keys Configuration

## Overview

Hardware Tool provides **native, first-party AI integration** — no hacky MCP servers or tacked-on attempts at controlling hardware. AI models have direct access to the tooling, enabling seamless design assistance, optimization, and natural language interaction.

## Supported AI Providers

### Provider Matrix

| Provider | Models | Features | Status |
|----------|--------|----------|--------|
| **OpenAI** | GPT-4, GPT-4 Turbo, GPT-4o | Full feature set | ✅ Supported |
| **Anthropic** | Claude 3 Opus, Sonnet, Haiku | Full feature set | ✅ Supported |
| **Google** | Gemini Pro, Gemini Ultra | Full feature set | ✅ Supported |
| **Local** | Llama, Mistral, CodeLlama | Core features | ✅ Supported |
| **Custom** | Any OpenAI-compatible API | Configurable | ✅ Supported |

## API Key Configuration

### Secure Storage Options

```rust
ApiKeyStorage {
    // Priority order for key lookup
    sources: vec![
        // 1. System keychain (most secure)
        KeySource::SystemKeychain,
        
        // 2. Environment variables
        KeySource::Environment,
        
        // 3. Encrypted config file
        KeySource::EncryptedConfig,
    ],
    
    // Never store in plain text
    plain_text_allowed: false,
}
```

### Environment Variables

```bash
# OpenAI
export HWTOOLS_OPENAI_API_KEY="sk-..."

# Anthropic
export HWTOOLS_ANTHROPIC_API_KEY="sk-ant-..."

# Google
export HWTOOLS_GOOGLE_API_KEY="..."

# Custom endpoint
export HWTOOLS_AI_ENDPOINT="https://your-api.com/v1"
export HWTOOLS_AI_API_KEY="..."
```

### Configuration File

```toml
# ~/.config/hardware-tool/ai.toml (encrypted)

[ai]
enabled = true
default_provider = "openai"

[providers.openai]
# Key stored in system keychain, referenced by ID
key_ref = "keychain:hardware-tool/openai"
model = "gpt-4-turbo"
max_tokens = 4096
temperature = 0.7

[providers.anthropic]
key_ref = "keychain:hardware-tool/anthropic"
model = "claude-3-opus"
max_tokens = 4096

[providers.local]
endpoint = "http://localhost:11434"
model = "codellama:34b"

[providers.custom]
endpoint = "https://your-company-api.com/v1"
key_ref = "env:COMPANY_AI_KEY"
model = "internal-model-v2"
```

### GUI Configuration

```
┌─────────────────────────────────────────────────────────────────┐
│ AI Configuration                                        [?]     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ ☑ Enable AI Features                                            │
│                                                                 │
│ Default Provider: [OpenAI          ▼]                           │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ OpenAI                                              [Edit]  │ │
│ │ API Key: ••••••••••••••••sk-...abc    [🔑 Change]          │ │
│ │ Model: gpt-4-turbo                                          │ │
│ │ Status: ✅ Connected                                        │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Anthropic                                           [Edit]  │ │
│ │ API Key: Not configured                   [🔑 Add Key]      │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Local (Ollama)                                      [Edit]  │ │
│ │ Endpoint: http://localhost:11434                            │ │
│ │ Status: ⚠️ Not running                                      │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [+ Add Custom Provider]                                         │
│                                                                 │
│ ─────────────────────────────────────────────────────────────── │
│                                                                 │
│ Usage This Month:                                               │
│   Tokens: 145,230 / 1,000,000                                   │
│   Cost: $4.52 (estimated)                                       │
│                                                                 │
│ [Test Connection]  [View Usage Details]                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Security Best Practices

### Key Management

```rust
SecurityConfig {
    // Encryption
    encryption: Encryption::Aes256Gcm,
    key_derivation: KeyDerivation::Argon2id,
    
    // Access control
    require_unlock: true,
    unlock_timeout: Duration::minutes(30),
    
    // Audit
    log_api_calls: true,
    log_tokens_used: true,
    never_log_keys: true,
    never_log_prompts: false,  // Optional for debugging
}
```

### Network Security

```rust
NetworkSecurity {
    // TLS requirements
    min_tls_version: TlsVersion::V1_3,
    verify_certificates: true,
    
    // Proxy support
    respect_system_proxy: true,
    allow_proxy_override: true,
    
    // Rate limiting
    rate_limit_requests: true,
    max_requests_per_minute: 60,
}
```

## Provider-Specific Setup

### OpenAI Setup

1. Visit [platform.openai.com](https://platform.openai.com)
2. Create API key with appropriate permissions
3. Add to Hardware Tool:

```bash
# Via CLI
hwt config ai set-key openai

# Or via environment
export HWTOOLS_OPENAI_API_KEY="sk-..."
```

### Anthropic Setup

1. Visit [console.anthropic.com](https://console.anthropic.com)
2. Generate API key
3. Configure:

```bash
hwt config ai set-key anthropic
```

### Local Model Setup (Ollama)

```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Pull a model
ollama pull codellama:34b

# Configure Hardware Tool
hwt config ai add-provider local \
  --endpoint http://localhost:11434 \
  --model codellama:34b
```

### Custom/Enterprise Setup

```bash
# Add custom provider
hwt config ai add-provider mycompany \
  --endpoint https://ai.mycompany.com/v1 \
  --model internal-v2 \
  --key-env MYCOMPANY_AI_KEY
```

## Feature Permissions

### Granular AI Controls

```toml
[ai.features]
# Design assistance
auto_placement = true
auto_routing = true
design_review = true
optimization = true

# Natural language
natural_language_commands = true
chat_interface = true

# Code generation
generate_rust_code = true
generate_documentation = true

# Data sharing
send_design_context = true
send_component_data = true
send_net_names = true  # May contain sensitive info

# Limits
max_context_size = 32000  # tokens
max_response_size = 8000  # tokens
```

### Privacy Controls

```rust
PrivacyConfig {
    // What can be sent to AI
    allow_schematic_data: true,
    allow_pcb_data: true,
    allow_component_values: true,
    allow_net_names: true,
    allow_project_name: false,  // Anonymize
    
    // Data retention
    allow_training: false,      // Opt-out of training
    request_deletion: true,     // Request data deletion
}
```

## Troubleshooting

### Connection Issues

```bash
# Test API connection
hwt ai test-connection

# Output:
# Testing OpenAI connection...
# ✅ API key valid
# ✅ Model access confirmed (gpt-4-turbo)
# ✅ Latency: 145ms
# 
# Testing Anthropic connection...
# ❌ API key invalid or expired
```

### Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| `401 Unauthorized` | Invalid API key | Check/regenerate key |
| `429 Rate Limited` | Too many requests | Wait or upgrade plan |
| `403 Forbidden` | Model access denied | Check API permissions |
| `Connection refused` | Local model not running | Start Ollama/server |

## Related Topics

- [Native AI Design Assistant](./native-ai-assistant.md)
- [AI-Powered Routing & Optimization](./ai-routing-optimization.md)
- [Benchmarking Simulator](./benchmarking-simulator.md)
