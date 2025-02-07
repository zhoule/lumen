
# Lumen CLI 扩展版

这是一个基于 [Lumen](https://github.com/jnsahaj/lumen) 的扩展项目。Lumen 是一个使用 AI 来生成 git commit 信息、解释代码变更等功能的命令行工具。本项目在原有基础上进行了一些功能扩展和优化。

## 主要扩展功能

- 支持自定义 OpenAI API 代理地址，可以使用第三方中转 API 服务
- 完整支持多个 AI 提供商配置

## 安装方法

1. 首先需要安装原版 Lumen:

```bash
cargo install lumen
```

2. 然后通过源码安装本扩展版:

```bash
git clone <repository-url>
cd lumen
cargo install --path .
```

## 使用方法

### 基础用法

在 Mac 系统中,可以在 `.zshrc` 或 `.bashrc` 中添加以下别名来简化使用:

```bash
alias aicommit='lumen draft | git commit -F -'
```

这样可以直接使用 `aicommit` 命令来生成 commit 信息并提交。

### OpenAI 代理配置

可以通过环境变量配置使用第三方 OpenAI API 代理服务：

```bash
# 在 .zshrc 或 .bashrc 中添加
export LUMEN_AI_PROVIDER=openai
export LUMEN_API_KEY=YOUR_API_KEY
export LUMEN_API_BASE_URL=YOUR_PROXY_URL  # 例如: https://your-proxy-domain/v1
```

或者在项目的 `lumen.config.json` 中配置：

```json
{
  "provider": "openai",
  "api_key": "YOUR_API_KEY",
  "api_base_url": "YOUR_PROXY_URL"
}
```

这样可以使用淘宝等平台购买的 OpenAI API 中转服务，降低使用成本。

## 其他功能

- 智能生成 commit 信息
- 解释代码变更
- 交互式搜索 commit 历史
- 支持自定义配置

## 配置

可以通过以下方式配置:

1. 命令行参数
2. 配置文件 (`lumen.config.json`)
3. 环境变量

具体配置选项请参考原版 Lumen 的文档。

## License

MIT License

