# Terminal SSR

Server side rendered terminal dashboards

## Usage

```bash
# Start server
cargo run --quiet

# Render output
 curl localhost:8910/sse -s -N > /tmp/output.txt | watch tail -n 8 /tmp/output.txt 
```