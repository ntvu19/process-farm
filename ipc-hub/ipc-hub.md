# Service Discovery - IPC Hub
It has a routing table with this structure:
> Map<ServiceName, ConnectionObject>

- Key: service name
- Value: object which manage the socket connect to this process

# Workflow
## 1. Registration
- Worker process A starts
- Connect to UDS/named pipe of this IPC Hub
- Worker send IpcPacket
    - Header: `type = REGISTER`, `source_service = 'worker_A'`
    - Payload: Empty or contains metadata
- Hub receive packet:
    - Store to routing map: `{"worker_A": "SocketConn_A"}`
    - Respond `IpcPacket`: `type = ACK`, payload `RegisterResponse{success: true}`

## 2. Routing
- Process A wants to send data to process B
- "Worker_A" package data into IpcPacket:
    - Header: `source="worker_A"`, `dest="worker_B"`, `type=DATA`
    - Payload: `<data>`
- IPC Hub receives packet:
    - Read `dest_service` -> `worker_B`
    - Search in map
        - Case 1: Found (exist)
            - Hub forward entire of the payload into `SocketConn_B` of `worker_B`
        - Case 2: Not found (process die, or not run yet)
            - Hub creates a packet `IpcPacket`, send back to `worker_A`
                - Header: `type=ERROR`, `dest="worker_A"`
                - Payload: `ErrorPayload{code: 404, message: "Destination service is not found"}`

## 3. Liveness
- The connection is **stateful** (due to using UDS/TCP)
- If process die (crash) -> OS will close socket
-> Hub will receive event `EOF` or `SocketException`
- Hub immediately remove process from the routing map

=> Service Discovery updated real-time without heartbeat continuously (but can still implement heartbeat message in case want to check application hang)