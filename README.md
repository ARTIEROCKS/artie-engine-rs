# Artie Engine gRPC Service

## Description

This gRPC service is part of the Artie Engine system and is designed to calculate distances and make adjustments in a workspace based on a set of blocks. It uses the `tonic` framework for gRPC handling and is written in Rust.

## Endpoints

### `DistanceCalculation`

Calculates the distance between two workspaces and suggests necessary adjustments to minimize the distance.

#### Request

```json
{
  "workspace": {
    "id": "string",
    "name": "string",
    "blocks": [
      {
        "id": "string",
        "name": "string",
        "family": "string",
        "inputs": [
          {
            "name": "string",
            "code": "string",
            "fields": [
              {"name": "string", "value": "string"}
            ]
          }
        ],
        "next": null,
        "nested": []
      }
    ]
  },
  "solution": {
    "id": "string",
    "name": "string",
    "blocks": [
      {
        "id": "string",
        "name": "string",
        "family": "string",
        "inputs": [
          {
            "name": "string",
            "code": "string",
            "fields": [
              {"name": "string", "value": "string"}
            ]
          }
        ],
        "next": null,
        "nested": []
      }
    ]
  }
}
