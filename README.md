# Process Farm
Orchestrate the processes within the same machine with highest performance

## Components
- Service Discovery
    - This is where all agents are registered
- Configuration Server
    - Store the configuration for the whole application
- Distributed Tracing
    - Monitor service
- Message Queue
    - Structure to store the messages, and transfer them to each agent
- Communication Method
    - Contains communication payload
    - Use "protobuf" format
- Agent Interface
    - Interface for each agent
    - All agent must be implemented follow to this template
