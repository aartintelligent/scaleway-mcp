# Scaleway MCP

Test MCP

```shell
npx @modelcontextprotocol/inspector cargo run
```

Dans la section "Environments" ajouter : SCALEWAY_SECRET_KEY + valeur

Puis Cliquer sur connection

---

Build docker image

```shell
docker build . -t mcp/scaleway
```

---

Usage

```json
{
  "mcpServers": {
    "scaleway": {
      "command": "docker",
      "args": [
        "run",
        "-i",
        "--rm",
        "-e",
        "SCALEWAY_SECRET_KEY",
        "mcp/scaleway"
      ],
      "env": {
        "SCALEWAY_SECRET_KEY": "<<!CHANGE-IT!>>"
      }
    }
  }
}
```
