export type Loader = {
  "version": "0.1.0",
  "name": "loader",
  "instructions": [
    {
      "name": "golInitialize",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "bytecode",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "handle",
          "type": "string"
        }
      ]
    },
    {
      "name": "golClear",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "bytecode",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "handle",
          "type": "string"
        }
      ]
    },
    {
      "name": "golWrite",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "bytecode",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "data",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "golFinalize",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "bytecode",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "golExecute",
      "accounts": [
        {
          "name": "bytecode",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "id",
          "type": "string"
        },
        {
          "name": "args",
          "type": "bytes"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "golBytecode",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "handle",
            "type": "string"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "finalized",
            "type": "bool"
          },
          {
            "name": "meta",
            "type": "bytes"
          },
          {
            "name": "content",
            "type": "bytes"
          }
        ]
      }
    }
  ]
};

export const IDL: Loader = {
  "version": "0.1.0",
  "name": "loader",
  "instructions": [
    {
      "name": "golInitialize",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "bytecode",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "handle",
          "type": "string"
        }
      ]
    },
    {
      "name": "golClear",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "bytecode",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "handle",
          "type": "string"
        }
      ]
    },
    {
      "name": "golWrite",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "bytecode",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "data",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "golFinalize",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "bytecode",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "golExecute",
      "accounts": [
        {
          "name": "bytecode",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "id",
          "type": "string"
        },
        {
          "name": "args",
          "type": "bytes"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "golBytecode",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "handle",
            "type": "string"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "finalized",
            "type": "bool"
          },
          {
            "name": "meta",
            "type": "bytes"
          },
          {
            "name": "content",
            "type": "bytes"
          }
        ]
      }
    }
  ]
};
