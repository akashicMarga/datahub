# datahub
Learning to build a framework to receive all types of data on cloud or on device in single, batch and streaming mode.

## Running the Client and Server

### Server
To run the server:

1. Navigate to the server directory:
   ```
   cd datahub/server
   ```

2. Run the server:
   ```
   cargo run
   ```

The server will start and listen on `[::1]:8080`.

### Client
To run the client:

1. Navigate to the client directory:
   ```
   cd datahub/client
   ```

2. Run the client:
   ```
   cargo run
   ```

The client will connect to the server and send requests.

## Grafana Logging

This project uses Grafana for log visualization. To set up and use Grafana:

1. Start the Grafana stack (Grafana, Loki, and Promtail) using Docker Compose:
   ```
   docker-compose up -d
   ```

2. Access Grafana at `http://localhost:3000`.

3. Add Loki as a data source:
   - URL: `http://loki:3100`
   - Click "Save & Test"

4. Create a new dashboard and add a panel:
   - Choose Loki as the data source
   - Use the query: `{job="rustserver"}`

5. Explore logs:
   - Use label filters like `{level="error"}` to narrow down results
   - Use text search with `|=` operator, e.g., `{job="rustserver"} |= "error"`

Logs from your Rust application will be collected by Promtail, sent to Loki, and can be visualized in Grafana.
