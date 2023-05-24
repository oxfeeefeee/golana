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
        },
        {
          "name": "memDump",
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
        },
        {
          "name": "memDump",
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
        },
        {
          "name": "memDump",
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
          "name": "memDump",
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
            "name": "content",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "golMemoryDump",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bytecode",
            "type": "publicKey"
          },
          {
            "name": "metaPtr",
            "type": "u64"
          },
          {
            "name": "bcPtr",
            "type": "u64"
          },
          {
            "name": "dump",
            "type": {
              "array": [
                "u8",
                256
              ]
            }
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
        },
        {
          "name": "memDump",
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
        },
        {
          "name": "memDump",
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
        },
        {
          "name": "memDump",
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
          "name": "memDump",
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
            "name": "content",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "golMemoryDump",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bytecode",
            "type": "publicKey"
          },
          {
            "name": "metaPtr",
            "type": "u64"
          },
          {
            "name": "bcPtr",
            "type": "u64"
          },
          {
            "name": "dump",
            "type": {
              "array": [
                "u8",
                256
              ]
            }
          }
        ]
      }
    }
  ]
};
