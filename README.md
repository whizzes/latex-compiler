<div>
  <h4 align="center">
    LaTeX Compiler Server
  </h4>
</div>

## Run in Docker

1. Pull and Run the Image

```bash
docker pull ghcr.io/whizzes/latex_compiler:latest
docker run -p 9000:9000 latex_compiler
```

2. The service should be available in your local network by addressing `http://localhost:9000`
