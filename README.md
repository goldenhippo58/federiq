# FederIQ Documentation

## Quick Start

### Docker Setup and Commands

1. Build the Docker images:
```bash
# Build all services
docker-compose build

# Start the entire stack
docker-compose up -d

# Check running services
docker-compose ps
```

2. Individual container management:
```bash
# Start specific services
docker-compose up -d mysql mongodb postgres

# View logs
docker-compose logs -f federiq

# Stop services
docker-compose down
```

### Database Setup

#### MySQL Setup
1. Connect to MySQL:
```bash
docker exec -it federiq-mysql mysql -uroot -p
```

2. Create the database structure:
```sql
CREATE DATABASE federiq;
USE federiq;

CREATE TABLE users (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(100),
    email VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE transactions (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT,
    amount DECIMAL(10,2),
    transaction_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

#### MongoDB Setup
1. Connect to MongoDB:
```bash
docker exec -it federiq-mongodb mongosh
```

2. Create collections:
```javascript
use federiq

db.createCollection('user_profiles')
db.createCollection('analytics')

// Create indexes
db.user_profiles.createIndex({ "user_id": 1 })
db.analytics.createIndex({ "timestamp": 1 })
```

### API Documentation

Base URL: `http://localhost:3000`

#### Available Endpoints

1. **Health Check**
   - Endpoint: `/health`
   - Method: GET
   - Description: Checks if the service is running
   - Example: `curl http://localhost:3000/health`

2. **Federated Query**
   - Endpoint: `/federated_query`
   - Method: GET
   - Description: Query data across multiple data sources
   
   Parameters:
   - `page`: Page number (default: 1)
   - `size`: Items per page (default: 10)
   - `filter`: Filter criteria
   - `source`: Comma-separated list of data sources

   Examples:
   ```bash
   # Basic query
   curl http://localhost:3000/federated_query

   # With pagination
   curl http://localhost:3000/federated_query?page=1&size=2

   # With filtering
   curl http://localhost:3000/federated_query?filter=name=Alice

   # Query specific sources
   curl http://localhost:3000/federated_query?source=postgres,mysql
   ```

### Testing the APIs

#### Using Postman

Import the following collection:

```json
{
  "info": {
    "name": "FederIQ API",
    "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
  },
  "item": [
    {
      "name": "Health Check",
      "request": {
        "method": "GET",
        "url": "http://localhost:3000/health"
      }
    },
    {
      "name": "Federated Query",
      "request": {
        "method": "GET",
        "url": "http://localhost:3000/federated_query"
      }
    },
    {
      "name": "Paginated Query",
      "request": {
        "method": "GET",
        "url": {
          "raw": "http://localhost:3000/federated_query?page=1&size=2",
          "query": [
            {"key": "page", "value": "1"},
            {"key": "size", "value": "2"}
          ]
        }
      }
    },
    {
      "name": "Filtered Query",
      "request": {
        "method": "GET",
        "url": {
          "raw": "http://localhost:3000/federated_query?filter=name=Alice",
          "query": [
            {"key": "filter", "value": "name=Alice"}
          ]
        }
      }
    }
  ]
}
```

#### Using curl

Test all endpoints:
```bash
# Health check
curl http://localhost:3000/health

# Basic query
curl http://localhost:3000/federated_query

# Paginated query
curl "http://localhost:3000/federated_query?page=1&size=2"

# Filtered query
curl "http://localhost:3000/federated_query?filter=name=Alice"

# Source-specific query
curl "http://localhost:3000/federated_query?source=postgres,mysql"
```

### Error Handling

The API returns standard HTTP status codes:
- 200: Success
- 400: Bad Request
- 404: Not Found
- 500: Internal Server Error

Example error response:
```json
{
  "error": {
    "code": "BAD_REQUEST",
    "message": "Invalid filter format"
  }
}
```

### Monitoring and Logs

1. View service logs:
```bash
# All services
docker-compose logs

# Specific service
docker-compose logs federiq
docker-compose logs mysql
```

2. Monitor container status:
```bash
docker-compose ps
docker stats
```

For more detailed documentation, please refer to the individual documentation files in the `/docs` directory of the project.