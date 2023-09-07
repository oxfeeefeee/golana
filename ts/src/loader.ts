export type Loader = {
  "version": "0.1.1",
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
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "handle",
          "type": "string"
        },
        {
          "name": "newSize",
          "type": "u64"
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
      "args": [
        {
          "name": "stepNum",
          "type": "u32"
        }
      ]
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
  "accounts": []
};

export const IDL: Loader = {
  "version": "0.1.1",
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
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "handle",
          "type": "string"
        },
        {
          "name": "newSize",
          "type": "u64"
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
      "args": [
        {
          "name": "stepNum",
          "type": "u32"
        }
      ]
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
  "accounts": []
};