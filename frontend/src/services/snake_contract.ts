/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/snake_contract.json`.
 */
export type SnakeContract = {
  "address": "3sXaMR5bCoP5ePizVUCXcWykZL3PdckHMUKoG7gZyRY6",
  "metadata": {
    "name": "snakeContract",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "acceptOtcSwap",
      "discriminator": [
        103,
        14,
        86,
        129,
        24,
        186,
        131,
        169
      ],
      "accounts": [
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "buyerClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "buyer"
              }
            ]
          }
        },
        {
          "name": "seller",
          "writable": true
        },
        {
          "name": "sellerClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              }
            ]
          }
        },
        {
          "name": "otcSwap",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  116,
                  99,
                  95,
                  115,
                  119,
                  97,
                  112
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              }
            ]
          }
        },
        {
          "name": "sellerTokenAccount",
          "writable": true
        },
        {
          "name": "buyerTokenAccount",
          "writable": true
        },
        {
          "name": "treasury",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  114,
                  101,
                  97,
                  115,
                  117,
                  114,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "tokenMint",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "acceptOtcSwapPatronToPatron",
      "discriminator": [
        231,
        125,
        194,
        212,
        119,
        193,
        183,
        53
      ],
      "accounts": [
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "buyerClaim",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "buyer"
              }
            ]
          }
        },
        {
          "name": "seller",
          "writable": true
        },
        {
          "name": "sellerClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              }
            ]
          }
        },
        {
          "name": "otcSwap",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  116,
                  99,
                  95,
                  115,
                  119,
                  97,
                  112
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              }
            ]
          }
        },
        {
          "name": "sellerTokenAccount",
          "writable": true
        },
        {
          "name": "buyerTokenAccount",
          "writable": true
        },
        {
          "name": "tokenMint",
          "writable": true
        },
        {
          "name": "treasury",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  114,
                  101,
                  97,
                  115,
                  117,
                  114,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "acceptTreasuryBuyback",
      "discriminator": [
        253,
        114,
        222,
        81,
        96,
        253,
        41,
        189
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "rewardPool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              }
            ]
          }
        },
        {
          "name": "seller",
          "writable": true
        },
        {
          "name": "otcSwap",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  116,
                  99,
                  95,
                  115,
                  119,
                  97,
                  112
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              }
            ]
          }
        },
        {
          "name": "sellerTokenAccount",
          "writable": true
        },
        {
          "name": "treasuryTokenAccount",
          "writable": true
        },
        {
          "name": "treasury",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  114,
                  101,
                  97,
                  115,
                  117,
                  114,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "allocateDaoSeat",
      "discriminator": [
        16,
        130,
        227,
        85,
        27,
        25,
        22,
        156
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "user"
        },
        {
          "name": "daoSeat",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  100,
                  97,
                  111,
                  95,
                  115,
                  101,
                  97,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "daoRegistry",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  100,
                  97,
                  111,
                  95,
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "currentBalance",
          "type": "u64"
        }
      ]
    },
    {
      "name": "applyForPatron",
      "discriminator": [
        222,
        230,
        200,
        168,
        244,
        6,
        161,
        183
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "walletAgeDays",
          "type": "u32"
        },
        {
          "name": "communityScore",
          "type": "u32"
        }
      ]
    },
    {
      "name": "approvePatronApplication",
      "discriminator": [
        54,
        232,
        175,
        114,
        171,
        249,
        231,
        203
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "applicant"
              }
            ]
          }
        },
        {
          "name": "applicant"
        }
      ],
      "args": [
        {
          "name": "minQualificationScore",
          "type": "u32"
        }
      ]
    },
    {
      "name": "cancelOtcOrder",
      "discriminator": [
        214,
        181,
        157,
        73,
        249,
        155,
        219,
        244
      ],
      "accounts": [
        {
          "name": "seller",
          "writable": true,
          "signer": true
        },
        {
          "name": "otcOrder",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  116,
                  99,
                  95,
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "cancelOtcSwap",
      "discriminator": [
        109,
        53,
        155,
        253,
        54,
        40,
        17,
        174
      ],
      "accounts": [
        {
          "name": "seller",
          "writable": true,
          "signer": true
        },
        {
          "name": "otcSwap",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  116,
                  99,
                  95,
                  115,
                  119,
                  97,
                  112
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              }
            ]
          }
        },
        {
          "name": "sellerTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "checkPatronEligibility",
      "discriminator": [
        230,
        90,
        214,
        241,
        71,
        125,
        96,
        133
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "minScore",
          "type": "u32"
        }
      ],
      "returns": "bool"
    },
    {
      "name": "claimReward",
      "discriminator": [
        149,
        95,
        181,
        242,
        94,
        90,
        158,
        162
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "relations": [
            "rewardPool"
          ]
        },
        {
          "name": "rewardPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              }
            ]
          }
        },
        {
          "name": "treasury",
          "writable": true,
          "relations": [
            "rewardPool"
          ]
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "userTokenAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "user"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "mint",
          "writable": true,
          "relations": [
            "rewardPool"
          ]
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "claimTokensWithRole",
      "discriminator": [
        93,
        237,
        253,
        107,
        168,
        122,
        52,
        151
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "claimReceipt",
          "writable": true
        },
        {
          "name": "userTokenAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "user"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "rewardPoolPda",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              }
            ]
          }
        },
        {
          "name": "treasuryTokenAccount",
          "writable": true
        },
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "role",
          "type": {
            "defined": {
              "name": "userRole"
            }
          }
        },
        {
          "name": "tweetId",
          "type": "string"
        }
      ]
    },
    {
      "name": "claimVestedTokens",
      "discriminator": [
        165,
        219,
        11,
        0,
        187,
        52,
        142,
        199
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "vestingSchedule",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  101,
                  115,
                  116,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "vestingEscrow",
          "writable": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "claimYield",
      "discriminator": [
        49,
        74,
        111,
        7,
        186,
        22,
        61,
        165
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "rewardPoolPda",
          "docs": [
            "Reward Pool PDA"
          ],
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              }
            ]
          }
        },
        {
          "name": "treasury",
          "docs": [
            "Treasury token account owned by reward pool PDA"
          ],
          "writable": true
        },
        {
          "name": "userStakingHistory",
          "docs": [
            "User staking history PDA"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103,
                  95,
                  104,
                  105,
                  115,
                  116,
                  111,
                  114,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "globalStakingStats",
          "docs": [
            "Global staking stats PDA"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103,
                  95,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "createOtcOrder",
      "discriminator": [
        122,
        165,
        2,
        74,
        145,
        77,
        94,
        236
      ],
      "accounts": [
        {
          "name": "seller",
          "writable": true,
          "signer": true
        },
        {
          "name": "sellerClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              }
            ]
          }
        },
        {
          "name": "sellerTokenAccount",
          "writable": true
        },
        {
          "name": "otcOrder",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  116,
                  99,
                  95,
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "orderId",
          "type": "u64"
        },
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "price",
          "type": "u64"
        },
        {
          "name": "buyerRestrictions",
          "type": {
            "defined": {
              "name": "buyerRestrictions"
            }
          }
        }
      ]
    },
    {
      "name": "createVestingSchedule",
      "discriminator": [
        195,
        30,
        184,
        253,
        77,
        154,
        187,
        66
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "vestingSchedule",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  101,
                  115,
                  116,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "vestingEscrow",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "vestingAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "executeOtcOrder",
      "discriminator": [
        0,
        204,
        228,
        196,
        134,
        168,
        107,
        25
      ],
      "accounts": [
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "buyerClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "buyer"
              }
            ]
          }
        },
        {
          "name": "otcOrder",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  116,
                  99,
                  95,
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "otc_order.seller",
                "account": "otcOrder"
              }
            ]
          }
        },
        {
          "name": "sellerTokenAccount",
          "writable": true
        },
        {
          "name": "buyerTokenAccount",
          "writable": true
        },
        {
          "name": "buyerPaymentAccount",
          "writable": true
        },
        {
          "name": "sellerPaymentAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "getGlobalStakingStats",
      "discriminator": [
        252,
        88,
        46,
        250,
        9,
        67,
        196,
        154
      ],
      "accounts": [
        {
          "name": "globalStakingStats",
          "docs": [
            "Global staking stats PDA"
          ],
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103,
                  95,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "getSwapStats",
      "discriminator": [
        186,
        157,
        111,
        217,
        75,
        59,
        10,
        86
      ],
      "accounts": [
        {
          "name": "user"
        },
        {
          "name": "otcTracker",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  116,
                  99,
                  95,
                  116,
                  114,
                  97,
                  99,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "getUserStakingSummary",
      "discriminator": [
        223,
        49,
        206,
        85,
        252,
        80,
        13,
        133
      ],
      "accounts": [
        {
          "name": "user",
          "signer": true
        },
        {
          "name": "userStakingHistory",
          "docs": [
            "User staking history PDA"
          ],
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103,
                  95,
                  104,
                  105,
                  115,
                  116,
                  111,
                  114,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "initializeDaoRegistry",
      "discriminator": [
        91,
        182,
        232,
        194,
        209,
        113,
        148,
        95
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "daoRegistry",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  100,
                  97,
                  111,
                  95,
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "maxSeats",
          "type": "u32"
        },
        {
          "name": "minDaoStake",
          "type": "u64"
        },
        {
          "name": "month6Timestamp",
          "type": "i64"
        }
      ]
    },
    {
      "name": "initializeGlobalStats",
      "discriminator": [
        57,
        82,
        52,
        126,
        182,
        236,
        5,
        131
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "globalStakingStats",
          "docs": [
            "Global staking stats PDA"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103,
                  95,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "initializeRewardPool",
      "discriminator": [
        139,
        189,
        60,
        130,
        44,
        211,
        218,
        99
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "ownerAta",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "rewardPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              }
            ]
          }
        },
        {
          "name": "treasury",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "rewardPool"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "mint"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "initializeRewardPoolParams"
            }
          }
        }
      ]
    },
    {
      "name": "initializeStakingHistory",
      "discriminator": [
        102,
        254,
        212,
        144,
        216,
        14,
        203,
        115
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userStakingHistory",
          "docs": [
            "User staking history PDA"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103,
                  95,
                  104,
                  105,
                  115,
                  116,
                  111,
                  114,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "initializeUserClaim",
      "discriminator": [
        14,
        77,
        182,
        76,
        223,
        97,
        222,
        18
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "initiateOtcSwapEnhanced",
      "discriminator": [
        138,
        92,
        118,
        67,
        176,
        72,
        253,
        143
      ],
      "accounts": [
        {
          "name": "seller",
          "writable": true,
          "signer": true
        },
        {
          "name": "sellerClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              }
            ]
          }
        },
        {
          "name": "otcSwap",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  116,
                  99,
                  95,
                  115,
                  119,
                  97,
                  112
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              }
            ]
          }
        },
        {
          "name": "sellerTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "tokenAmount",
          "type": "u64"
        },
        {
          "name": "solRate",
          "type": "u64"
        },
        {
          "name": "buyerRebate",
          "type": "u64"
        },
        {
          "name": "swapType",
          "type": {
            "defined": {
              "name": "snake_contract::instructions::otc_swap_enhanced::SwapType"
            }
          }
        }
      ]
    },
    {
      "name": "lockTokens",
      "discriminator": [
        136,
        11,
        32,
        232,
        161,
        117,
        54,
        211
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "rewardPoolPda",
          "docs": [
            "Reward Pool PDA that will hold the locked tokens"
          ],
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              }
            ]
          }
        },
        {
          "name": "treasuryTokenAccount",
          "docs": [
            "Treasury token account that will receive the locked tokens"
          ],
          "writable": true
        },
        {
          "name": "userStakingHistory",
          "docs": [
            "User staking history PDA"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103,
                  95,
                  104,
                  105,
                  115,
                  116,
                  111,
                  114,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "globalStakingStats",
          "docs": [
            "Global staking stats PDA"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103,
                  95,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "durationMonths",
          "type": "u8"
        }
      ]
    },
    {
      "name": "mockUiEvent",
      "discriminator": [
        157,
        141,
        132,
        128,
        34,
        124,
        104,
        211
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "otcTracker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  116,
                  99,
                  95,
                  116,
                  114,
                  97,
                  99,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "eventType",
          "type": "string"
        },
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "patronExit",
      "discriminator": [
        189,
        239,
        175,
        60,
        117,
        213,
        49,
        115
      ],
      "accounts": [
        {
          "name": "patron",
          "writable": true,
          "signer": true
        },
        {
          "name": "patronClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "patron"
              }
            ]
          }
        },
        {
          "name": "patronTokenAccount",
          "writable": true
        },
        {
          "name": "tokenMint",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "exitAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "patronOtcExit",
      "discriminator": [
        181,
        172,
        90,
        44,
        141,
        64,
        88,
        201
      ],
      "accounts": [
        {
          "name": "patron",
          "writable": true,
          "signer": true
        },
        {
          "name": "patronClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "patron"
              }
            ]
          }
        },
        {
          "name": "patronTokenAccount",
          "writable": true
        },
        {
          "name": "buyerTokenAccount",
          "writable": true
        },
        {
          "name": "patronPaymentAccount",
          "writable": true
        },
        {
          "name": "buyerPaymentAccount",
          "writable": true
        },
        {
          "name": "tokenMint",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "exitAmount",
          "type": "u64"
        },
        {
          "name": "salePrice",
          "type": "u64"
        }
      ]
    },
    {
      "name": "realBurnOnExit",
      "discriminator": [
        7,
        195,
        248,
        29,
        254,
        152,
        52,
        39
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "otcTracker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  116,
                  99,
                  95,
                  116,
                  114,
                  97,
                  99,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "tokenMint",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "exitAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "revokeDaoSeat",
      "discriminator": [
        147,
        191,
        115,
        176,
        49,
        83,
        215,
        34
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "user"
        },
        {
          "name": "daoSeat",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  100,
                  97,
                  111,
                  95,
                  115,
                  101,
                  97,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "daoRegistry",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  100,
                  97,
                  111,
                  95,
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  121
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "reason",
          "type": "string"
        }
      ]
    },
    {
      "name": "revokePatronStatus",
      "discriminator": [
        65,
        160,
        252,
        124,
        185,
        101,
        190,
        242
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "applicant"
              }
            ]
          }
        },
        {
          "name": "applicant"
        }
      ],
      "args": []
    },
    {
      "name": "selectRole",
      "discriminator": [
        228,
        204,
        240,
        243,
        152,
        194,
        179,
        203
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "role",
          "type": {
            "defined": {
              "name": "userRole"
            }
          }
        }
      ]
    },
    {
      "name": "simulateBurnOnExit",
      "discriminator": [
        125,
        68,
        240,
        96,
        40,
        201,
        91,
        103
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "otcTracker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  116,
                  99,
                  95,
                  116,
                  114,
                  97,
                  99,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "exitAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "simulateOtcSwap",
      "discriminator": [
        237,
        214,
        168,
        125,
        19,
        26,
        38,
        197
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "otcTracker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  116,
                  99,
                  95,
                  116,
                  114,
                  97,
                  99,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "isSale",
          "type": "bool"
        }
      ]
    },
    {
      "name": "unlockTokens",
      "discriminator": [
        233,
        35,
        95,
        159,
        37,
        185,
        47,
        88
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "rewardPoolPda",
          "docs": [
            "Reward Pool PDA that holds the locked tokens"
          ],
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              }
            ]
          }
        },
        {
          "name": "treasuryTokenAccount",
          "docs": [
            "Treasury token account that holds the locked tokens"
          ],
          "writable": true
        },
        {
          "name": "userStakingHistory",
          "docs": [
            "User staking history PDA"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103,
                  95,
                  104,
                  105,
                  115,
                  116,
                  111,
                  114,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "globalStakingStats",
          "docs": [
            "Global staking stats PDA"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  107,
                  105,
                  110,
                  103,
                  95,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "updateRewardPool",
      "discriminator": [
        24,
        85,
        178,
        189,
        138,
        149,
        80,
        72
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true,
          "relations": [
            "rewardPool"
          ]
        },
        {
          "name": "rewardPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "updateRewardPoolParams"
            }
          }
        }
      ]
    },
    {
      "name": "updateUserStats",
      "docs": [
        "Update user statistics for patron qualification"
      ],
      "discriminator": [
        182,
        105,
        179,
        64,
        228,
        226,
        150,
        182
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "rewardPool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              }
            ]
          }
        },
        {
          "name": "user"
        },
        {
          "name": "userClaim",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "updateUserStatsParams"
            }
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "claimReceipt",
      "discriminator": [
        223,
        233,
        11,
        229,
        124,
        165,
        207,
        28
      ]
    },
    {
      "name": "daoRegistry",
      "discriminator": [
        230,
        192,
        79,
        185,
        139,
        117,
        7,
        141
      ]
    },
    {
      "name": "daoSeat",
      "discriminator": [
        169,
        31,
        75,
        69,
        232,
        100,
        229,
        159
      ]
    },
    {
      "name": "globalStakingStats",
      "discriminator": [
        211,
        22,
        171,
        98,
        97,
        237,
        148,
        125
      ]
    },
    {
      "name": "otcOrder",
      "discriminator": [
        160,
        106,
        162,
        213,
        214,
        222,
        100,
        153
      ]
    },
    {
      "name": "otcSwap",
      "discriminator": [
        215,
        59,
        233,
        43,
        181,
        178,
        17,
        85
      ]
    },
    {
      "name": "otcSwapTracker",
      "discriminator": [
        184,
        93,
        2,
        82,
        9,
        60,
        188,
        21
      ]
    },
    {
      "name": "rewardPool",
      "discriminator": [
        134,
        121,
        197,
        211,
        133,
        154,
        82,
        32
      ]
    },
    {
      "name": "userClaim",
      "discriminator": [
        228,
        142,
        195,
        181,
        228,
        147,
        32,
        209
      ]
    },
    {
      "name": "userStakingHistory",
      "discriminator": [
        166,
        45,
        215,
        81,
        81,
        50,
        38,
        129
      ]
    },
    {
      "name": "vestingSchedule",
      "discriminator": [
        130,
        200,
        173,
        148,
        39,
        75,
        243,
        147
      ]
    }
  ],
  "events": [
    {
      "name": "claimedReward",
      "discriminator": [
        99,
        108,
        97,
        105,
        109
      ]
    },
    {
      "name": "daoEligibilityAcquired",
      "discriminator": [
        35,
        49,
        168,
        57,
        144,
        229,
        185,
        51
      ]
    },
    {
      "name": "daoEligibilityRevoked",
      "discriminator": [
        15,
        92,
        72,
        49,
        250,
        42,
        142,
        225
      ]
    },
    {
      "name": "daoRegistryInitialized",
      "discriminator": [
        129,
        80,
        248,
        144,
        227,
        107,
        229,
        78
      ]
    },
    {
      "name": "daoSeatAcquired",
      "discriminator": [
        198,
        131,
        19,
        248,
        33,
        143,
        198,
        204
      ]
    },
    {
      "name": "daoSeatAllocated",
      "discriminator": [
        183,
        9,
        101,
        214,
        20,
        93,
        7,
        55
      ]
    },
    {
      "name": "daoSeatReturned",
      "discriminator": [
        26,
        116,
        4,
        170,
        42,
        157,
        104,
        4
      ]
    },
    {
      "name": "daoSeatRevoked",
      "discriminator": [
        123,
        192,
        199,
        45,
        80,
        68,
        177,
        200
      ]
    },
    {
      "name": "daoSeatTransferred",
      "discriminator": [
        142,
        207,
        75,
        182,
        135,
        15,
        152,
        154
      ]
    },
    {
      "name": "enhancedSwapCompleted",
      "discriminator": [
        28,
        98,
        15,
        98,
        88,
        199,
        118,
        116
      ]
    },
    {
      "name": "enhancedSwapCreated",
      "discriminator": [
        12,
        176,
        93,
        149,
        119,
        37,
        244,
        203
      ]
    },
    {
      "name": "otcTradeExecuted",
      "discriminator": [
        12,
        232,
        218,
        85,
        153,
        192,
        87,
        170
      ]
    },
    {
      "name": "patronApplicationSubmitted",
      "discriminator": [
        233,
        4,
        189,
        110,
        194,
        234,
        193,
        168
      ]
    },
    {
      "name": "patronApproved",
      "discriminator": [
        152,
        146,
        172,
        65,
        142,
        160,
        51,
        7
      ]
    },
    {
      "name": "patronExitTracked",
      "discriminator": [
        32,
        92,
        131,
        169,
        164,
        223,
        9,
        154
      ]
    },
    {
      "name": "patronExited",
      "discriminator": [
        95,
        201,
        73,
        98,
        227,
        240,
        242,
        34
      ]
    },
    {
      "name": "patronExitedOld",
      "discriminator": [
        124,
        127,
        34,
        96,
        147,
        111,
        25,
        114
      ]
    },
    {
      "name": "patronRebateDistributed",
      "discriminator": [
        199,
        56,
        220,
        136,
        108,
        121,
        95,
        96
      ]
    },
    {
      "name": "patronRevoked",
      "discriminator": [
        59,
        234,
        71,
        232,
        87,
        121,
        166,
        185
      ]
    },
    {
      "name": "proposalCancelled",
      "discriminator": [
        253,
        59,
        104,
        46,
        129,
        78,
        9,
        14
      ]
    },
    {
      "name": "proposalCreated",
      "discriminator": [
        186,
        8,
        160,
        108,
        81,
        13,
        51,
        206
      ]
    },
    {
      "name": "proposalExecuted",
      "discriminator": [
        92,
        213,
        189,
        201,
        101,
        83,
        111,
        83
      ]
    },
    {
      "name": "proposalFinalized",
      "discriminator": [
        159,
        104,
        210,
        220,
        86,
        209,
        61,
        51
      ]
    },
    {
      "name": "rewardPoolInitialized",
      "discriminator": [
        112,
        111,
        111,
        108,
        105,
        110,
        105,
        116
      ]
    },
    {
      "name": "swapCancelled",
      "discriminator": [
        210,
        232,
        53,
        121,
        126,
        236,
        66,
        142
      ]
    },
    {
      "name": "swapCompleted",
      "discriminator": [
        118,
        93,
        218,
        77,
        215,
        165,
        112,
        76
      ]
    },
    {
      "name": "swapInitiated",
      "discriminator": [
        88,
        197,
        100,
        28,
        189,
        82,
        98,
        2
      ]
    },
    {
      "name": "tokensBurned",
      "discriminator": [
        230,
        255,
        34,
        113,
        226,
        53,
        227,
        9
      ]
    },
    {
      "name": "tokensLocked",
      "discriminator": [
        63,
        184,
        201,
        20,
        203,
        194,
        249,
        138
      ]
    },
    {
      "name": "tokensUnlocked",
      "discriminator": [
        32,
        143,
        250,
        162,
        63,
        131,
        83,
        163
      ]
    },
    {
      "name": "tokensVested",
      "discriminator": [
        243,
        70,
        43,
        247,
        190,
        45,
        221,
        10
      ]
    },
    {
      "name": "userClaimInitialized",
      "discriminator": [
        54,
        243,
        185,
        212,
        76,
        168,
        45,
        243
      ]
    },
    {
      "name": "vestingCreated",
      "discriminator": [
        181,
        223,
        229,
        220,
        204,
        6,
        169,
        125
      ]
    },
    {
      "name": "vestingScheduleCreated",
      "discriminator": [
        234,
        207,
        231,
        98,
        6,
        142,
        111,
        165
      ]
    },
    {
      "name": "vestingWithdrawn",
      "discriminator": [
        250,
        252,
        81,
        188,
        247,
        57,
        169,
        145
      ]
    },
    {
      "name": "voteCast",
      "discriminator": [
        39,
        53,
        195,
        104,
        188,
        17,
        225,
        213
      ]
    },
    {
      "name": "yieldClaimed",
      "discriminator": [
        177,
        201,
        94,
        68,
        19,
        200,
        227,
        27
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "unauthorized",
      "msg": "[SNAKE:6000] Unauthorized owner key"
    },
    {
      "code": 6001,
      "name": "cooldownNotPassed",
      "msg": "[SNAKE:6001] Cooldown period not passed"
    },
    {
      "code": 6002,
      "name": "yieldClaimCooldownNotPassed",
      "msg": "[SNAKE:6002] Yield claim cooldown period not passed"
    },
    {
      "code": 6003,
      "name": "insufficientFundsInTreasury",
      "msg": "[SNAKE:6003] Insufficient funds in treasury"
    },
    {
      "code": 6004,
      "name": "insufficientFunds",
      "msg": "[SNAKE:6004] Insufficient funds"
    },
    {
      "code": 6005,
      "name": "invalidTreasuryAuthority",
      "msg": "[SNAKE:6005] Invalid treasury authority"
    },
    {
      "code": 6006,
      "name": "arithmeticOverflow",
      "msg": "[SNAKE:6006] Arithmetic overflow occurred"
    },
    {
      "code": 6007,
      "name": "endedClaim",
      "msg": "[SNAKE:6007] Claim is ended"
    },
    {
      "code": 6008,
      "name": "patronApplicationExists",
      "msg": "[SNAKE:6008] Patron application already exists"
    },
    {
      "code": 6009,
      "name": "patronNotApproved",
      "msg": "[SNAKE:6009] Patron not approved"
    },
    {
      "code": 6010,
      "name": "patronAlreadyApproved",
      "msg": "[SNAKE:6010] Patron already approved"
    },
    {
      "code": 6011,
      "name": "patronTokensLocked",
      "msg": "[SNAKE:6011] Cannot exit as patron - tokens are locked"
    },
    {
      "code": 6012,
      "name": "onlyApprovedPatrons",
      "msg": "[SNAKE:6012] Only approved patrons can perform this action"
    },
    {
      "code": 6013,
      "name": "tokensLocked",
      "msg": "[SNAKE:6013] Tokens are currently locked"
    },
    {
      "code": 6014,
      "name": "noTokensLocked",
      "msg": "[SNAKE:6014] No tokens locked for staking"
    },
    {
      "code": 6015,
      "name": "lockPeriodNotCompleted",
      "msg": "[SNAKE:6015] Lock period not completed"
    },
    {
      "code": 6016,
      "name": "invalidLockDuration",
      "msg": "[SNAKE:6016] Invalid lock duration"
    },
    {
      "code": 6017,
      "name": "cannotLockZeroTokens",
      "msg": "[SNAKE:6017] Cannot lock zero tokens"
    },
    {
      "code": 6018,
      "name": "notEligibleForDao",
      "msg": "[SNAKE:6018] Not eligible for DAO membership"
    },
    {
      "code": 6019,
      "name": "noAvailableSeats",
      "msg": "[SNAKE:6019] No available DAO seats"
    },
    {
      "code": 6020,
      "name": "notDaoSeatHolder",
      "msg": "[SNAKE:6020] Not a DAO seat holder"
    },
    {
      "code": 6021,
      "name": "insufficientStakeForDao",
      "msg": "[SNAKE:6021] Insufficient stake for DAO eligibility"
    },
    {
      "code": 6022,
      "name": "lockDurationRequirementNotMet",
      "msg": "[SNAKE:6022] Lock duration requirement not met for DAO"
    },
    {
      "code": 6023,
      "name": "patronTransferRestricted",
      "msg": "[SNAKE:6023] Patron tokens can only be transferred to other patrons"
    },
    {
      "code": 6024,
      "name": "cannotSellBeforeLockEnds",
      "msg": "[SNAKE:6024] Cannot sell tokens before lock period ends"
    },
    {
      "code": 6025,
      "name": "earlySaleDetected",
      "msg": "[SNAKE:6025] Early sale detected - commitment violated"
    },
    {
      "code": 6026,
      "name": "invalidAmount",
      "msg": "[SNAKE:6026] Invalid amount specified"
    },
    {
      "code": 6027,
      "name": "invalidRate",
      "msg": "[SNAKE:6027] Invalid rate specified"
    },
    {
      "code": 6028,
      "name": "invalidRebate",
      "msg": "[SNAKE:6028] Invalid rebate percentage"
    },
    {
      "code": 6029,
      "name": "onlyNormalUsersCanSell",
      "msg": "[SNAKE:6029] Only normal users can sell tokens"
    },
    {
      "code": 6030,
      "name": "onlyPatronsCanBuy",
      "msg": "[SNAKE:6030] Only patrons can buy in this swap"
    },
    {
      "code": 6031,
      "name": "swapInactive",
      "msg": "[SNAKE:6031] Swap is not active"
    },
    {
      "code": 6032,
      "name": "swapAlreadyAccepted",
      "msg": "[SNAKE:6032] Swap already accepted"
    },
    {
      "code": 6033,
      "name": "cannotBuyOwnSwap",
      "msg": "[SNAKE:6033] Cannot buy your own swap"
    },
    {
      "code": 6034,
      "name": "swapExpired",
      "msg": "[SNAKE:6034] Swap has expired"
    },
    {
      "code": 6035,
      "name": "swapNotExpired",
      "msg": "[SNAKE:6035] Swap has not expired yet"
    },
    {
      "code": 6036,
      "name": "invalidSwapType",
      "msg": "[SNAKE:6036] Invalid swap type"
    },
    {
      "code": 6037,
      "name": "treasuryFallbackNotAllowed",
      "msg": "[SNAKE:6037] Treasury fallback not allowed for this swap"
    },
    {
      "code": 6038,
      "name": "mathOverflow",
      "msg": "[SNAKE:6038] Math overflow occurred"
    },
    {
      "code": 6039,
      "name": "onlyExitersCanSell",
      "msg": "[SNAKE:6039] Only exiters (None role) can sell in Phase 1"
    },
    {
      "code": 6040,
      "name": "tokensStillLocked",
      "msg": "[SNAKE:6040] Tokens are still locked and cannot be sold"
    },
    {
      "code": 6041,
      "name": "patronAlreadyExited",
      "msg": "[SNAKE:6041] Patron has already been marked as exited"
    },
    {
      "code": 6042,
      "name": "listingNotActive",
      "msg": "[SNAKE:6042] Listing is not active yet (cooldown period)"
    },
    {
      "code": 6043,
      "name": "maxOtcLimitExceeded",
      "msg": "[SNAKE:6043] Maximum OTC limit exceeded"
    },
    {
      "code": 6044,
      "name": "onlyTreasuryCanBuy",
      "msg": "[SNAKE:6044] Only treasury can buy in this swap"
    },
    {
      "code": 6045,
      "name": "governanceNotActive",
      "msg": "[SNAKE:6045] Governance is not active"
    },
    {
      "code": 6046,
      "name": "invalidProposalTitle",
      "msg": "[SNAKE:6046] Invalid proposal title"
    },
    {
      "code": 6047,
      "name": "invalidProposalDescription",
      "msg": "[SNAKE:6047] Invalid proposal description"
    },
    {
      "code": 6048,
      "name": "invalidProposal",
      "msg": "[SNAKE:6048] Invalid proposal"
    },
    {
      "code": 6049,
      "name": "proposalNotActive",
      "msg": "[SNAKE:6049] Proposal is not active"
    },
    {
      "code": 6050,
      "name": "proposalNotPassed",
      "msg": "[SNAKE:6050] Proposal has not passed"
    },
    {
      "code": 6051,
      "name": "votingPeriodEnded",
      "msg": "[SNAKE:6051] Voting period has ended"
    },
    {
      "code": 6052,
      "name": "votingPeriodNotEnded",
      "msg": "[SNAKE:6052] Voting period has not ended"
    },
    {
      "code": 6053,
      "name": "cannotCancelProposal",
      "msg": "[SNAKE:6053] Cannot cancel proposal with votes"
    },
    {
      "code": 6054,
      "name": "alreadyVoted",
      "msg": "[SNAKE:6054] Already voted on this proposal"
    },
    {
      "code": 6055,
      "name": "invalidRoleTransition",
      "msg": "[SNAKE:6055] Invalid role transition"
    },
    {
      "code": 6056,
      "name": "insufficientQualificationScore",
      "msg": "[SNAKE:6056] Insufficient qualification score for patron"
    },
    {
      "code": 6057,
      "name": "noMiningHistory",
      "msg": "[SNAKE:6057] No mining history - user must mine tokens in Phase 1"
    },
    {
      "code": 6058,
      "name": "vestingNotUnlocked",
      "msg": "[SNAKE:6058] Vesting not unlocked yet"
    },
    {
      "code": 6059,
      "name": "vestingAlreadyWithdrawn",
      "msg": "[SNAKE:6059] Vesting already withdrawn"
    },
    {
      "code": 6060,
      "name": "swapNotActive",
      "msg": "[SNAKE:6060] Swap is not active"
    },
    {
      "code": 6061,
      "name": "notWhitelistedBuyer",
      "msg": "[SNAKE:6061] Not whitelisted for this swap"
    },
    {
      "code": 6062,
      "name": "insufficientRole",
      "msg": "[SNAKE:6062] Insufficient role for this swap"
    },
    {
      "code": 6063,
      "name": "onlyPatrons",
      "msg": "[SNAKE:6063] Only patrons can perform this action"
    },
    {
      "code": 6064,
      "name": "noExitToTrack",
      "msg": "[SNAKE:6064] No exit to track"
    },
    {
      "code": 6065,
      "name": "onlyPatronsCanSell",
      "msg": "[SNAKE:6065] Only patrons can sell in this swap type"
    },
    {
      "code": 6066,
      "name": "notEligibleForOtc",
      "msg": "[SNAKE:6066] Not eligible for OTC trading"
    },
    {
      "code": 6067,
      "name": "patronOnlyOrder",
      "msg": "[SNAKE:6067] Order is for patrons only"
    },
    {
      "code": 6068,
      "name": "treasuryOnlyOrder",
      "msg": "[SNAKE:6068] Order is for treasury only"
    },
    {
      "code": 6069,
      "name": "insufficientPatronScore",
      "msg": "[SNAKE:6069] Insufficient patron score"
    },
    {
      "code": 6070,
      "name": "orderNotActive",
      "msg": "[SNAKE:6070] Order is not active"
    },
    {
      "code": 6071,
      "name": "invalidRole",
      "msg": "[SNAKE:6071] Invalid role for this action"
    },
    {
      "code": 6072,
      "name": "notApprovedPatron",
      "msg": "[SNAKE:6072] Not approved patron"
    },
    {
      "code": 6073,
      "name": "patronSoldEarly",
      "msg": "[SNAKE:6073] Patron sold early"
    },
    {
      "code": 6074,
      "name": "vestingNotActive",
      "msg": "[SNAKE:6074] Vesting is not active"
    },
    {
      "code": 6075,
      "name": "nothingToClaim",
      "msg": "[SNAKE:6075] Nothing to claim"
    },
    {
      "code": 6076,
      "name": "month6NotReached",
      "msg": "[SNAKE:6076] Month 6 milestone not reached"
    },
    {
      "code": 6077,
      "name": "maxSeatsReached",
      "msg": "[SNAKE:6077] Maximum DAO seats reached"
    },
    {
      "code": 6078,
      "name": "seatNotActive",
      "msg": "[SNAKE:6078] DAO seat is not active"
    },
    {
      "code": 6079,
      "name": "failedToFetchSeats",
      "msg": "[SNAKE:6079] Failed to fetch DAO seats"
    },
    {
      "code": 6080,
      "name": "insufficientStakingHistory",
      "msg": "[SNAKE:6080] Insufficient staking history"
    },
    {
      "code": 6081,
      "name": "patronEligibilityNotMet",
      "msg": "[SNAKE:6081] Patron eligibility criteria not met"
    },
    {
      "code": 6082,
      "name": "insufficientTokenAmount",
      "msg": "[SNAKE:6082] Insufficient token amount for staking"
    },
    {
      "code": 6083,
      "name": "invalidUserRole",
      "msg": "[SNAKE:6083] Invalid user role for this action"
    },
    {
      "code": 6084,
      "name": "invalidApyRate",
      "msg": "[SNAKE:6084] Invalid APY rate - must be between 0-100%"
    },
    {
      "code": 6085,
      "name": "swapAlreadyActive",
      "msg": "[SNAKE:6085] Swap is already active for this user"
    }
  ],
  "types": [
    {
      "name": "buyerRestrictions",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "patronsOnly",
            "type": "bool"
          },
          {
            "name": "treasuryOnly",
            "type": "bool"
          },
          {
            "name": "minPatronScore",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "claimReceipt",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tweetId",
            "type": "string"
          },
          {
            "name": "claimer",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "claimedReward",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "rewardAmount",
            "type": "u64"
          },
          {
            "name": "burnAmount",
            "type": "u64"
          },
          {
            "name": "rewardLevel",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "daoEligibilityAcquired",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "daoEligibilityRevoked",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "reason",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "daoRegistry",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalSeats",
            "type": "u32"
          },
          {
            "name": "allocatedSeats",
            "type": "u32"
          },
          {
            "name": "maxSeats",
            "type": "u32"
          },
          {
            "name": "minDaoStake",
            "type": "u64"
          },
          {
            "name": "month6Timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "daoRegistryInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalSeats",
            "type": "u32"
          },
          {
            "name": "minStake",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "daoSeat",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "holder",
            "type": "pubkey"
          },
          {
            "name": "allocatedAt",
            "type": "i64"
          },
          {
            "name": "isActive",
            "type": "bool"
          },
          {
            "name": "votingPower",
            "type": "u64"
          },
          {
            "name": "role",
            "type": {
              "defined": {
                "name": "userRole"
              }
            }
          },
          {
            "name": "patronScore",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "daoSeatAcquired",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "seatNumber",
            "type": "u32"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "daoSeatAllocated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "holder",
            "type": "pubkey"
          },
          {
            "name": "allocatedAt",
            "type": "i64"
          },
          {
            "name": "votingPower",
            "type": "u64"
          },
          {
            "name": "role",
            "type": {
              "defined": {
                "name": "userRole"
              }
            }
          }
        ]
      }
    },
    {
      "name": "daoSeatReturned",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "daoSeatRevoked",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "holder",
            "type": "pubkey"
          },
          {
            "name": "revokedAt",
            "type": "i64"
          },
          {
            "name": "reason",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "daoSeatTransferred",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "from",
            "type": "pubkey"
          },
          {
            "name": "to",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "enhancedSwapCompleted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seller",
            "type": "pubkey"
          },
          {
            "name": "buyer",
            "type": "pubkey"
          },
          {
            "name": "swapAccount",
            "type": "pubkey"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          },
          {
            "name": "totalCost",
            "type": "u64"
          },
          {
            "name": "patronRebate",
            "type": "u64"
          },
          {
            "name": "netPayment",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "enhancedSwapCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seller",
            "type": "pubkey"
          },
          {
            "name": "swapAccount",
            "type": "pubkey"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          },
          {
            "name": "pricePerToken",
            "type": "u64"
          },
          {
            "name": "swapType",
            "type": {
              "defined": {
                "name": "snake_contract::instructions::otc_swap_enhanced::SwapType"
              }
            }
          },
          {
            "name": "patronRebatePercentage",
            "type": "u64"
          },
          {
            "name": "expiresAt",
            "type": "i64"
          },
          {
            "name": "whitelistedBuyers",
            "type": {
              "vec": "pubkey"
            }
          }
        ]
      }
    },
    {
      "name": "globalStakingStats",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "initialized",
            "type": "bool"
          },
          {
            "name": "totalUsers",
            "type": "u32"
          },
          {
            "name": "totalStakers",
            "type": "u32"
          },
          {
            "name": "totalPatrons",
            "type": "u32"
          },
          {
            "name": "totalLockedAmount",
            "type": "u64"
          },
          {
            "name": "totalYieldDistributed",
            "type": "u64"
          },
          {
            "name": "lastUpdated",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "initializeRewardPoolParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "otcOrder",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "orderId",
            "type": "u64"
          },
          {
            "name": "seller",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "price",
            "type": "u64"
          },
          {
            "name": "isActive",
            "type": "bool"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "buyerRestrictions",
            "type": {
              "defined": {
                "name": "buyerRestrictions"
              }
            }
          }
        ]
      }
    },
    {
      "name": "otcTradeExecuted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "orderId",
            "type": "u64"
          },
          {
            "name": "seller",
            "type": "pubkey"
          },
          {
            "name": "buyer",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "price",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "otcSwap",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seller",
            "type": "pubkey"
          },
          {
            "name": "buyer",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          },
          {
            "name": "solRate",
            "type": "u64"
          },
          {
            "name": "buyerRebate",
            "type": "u64"
          },
          {
            "name": "sellerRole",
            "type": {
              "defined": {
                "name": "userRole"
              }
            }
          },
          {
            "name": "buyerRoleRequired",
            "type": {
              "defined": {
                "name": "userRole"
              }
            }
          },
          {
            "name": "swapType",
            "type": {
              "defined": {
                "name": "snake_contract::state::otc_swap::SwapType"
              }
            }
          },
          {
            "name": "isActive",
            "type": "bool"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "expiresAt",
            "type": "i64"
          },
          {
            "name": "treasuryFallback",
            "type": "bool"
          },
          {
            "name": "burnPenaltyRate",
            "type": "u64"
          },
          {
            "name": "fixedPrice",
            "type": "u64"
          },
          {
            "name": "maxOtcLimit",
            "type": "u64"
          },
          {
            "name": "sellerExited",
            "type": "bool"
          },
          {
            "name": "cooldownPeriod",
            "type": "i64"
          },
          {
            "name": "listingActiveAt",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "otcSwapTracker",
      "docs": [
        "Stub structure to track simulated OTC swap events"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "totalSwaps",
            "type": "u64"
          },
          {
            "name": "totalVolume",
            "type": "u64"
          },
          {
            "name": "lastSwapTimestamp",
            "type": "i64"
          },
          {
            "name": "exitTracked",
            "type": "bool"
          },
          {
            "name": "burnPenaltyApplied",
            "type": "bool"
          },
          {
            "name": "daoEligibilityRevoked",
            "type": "bool"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "patronApplicationSubmitted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "qualificationScore",
            "type": "u32"
          },
          {
            "name": "walletAgeDays",
            "type": "u32"
          },
          {
            "name": "communityScore",
            "type": "u32"
          },
          {
            "name": "totalMined",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "patronApproved",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "approvedBy",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "patronExitTracked",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "patronExited",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "patron",
            "type": "pubkey"
          },
          {
            "name": "exitAmount",
            "type": "u64"
          },
          {
            "name": "burnAmount",
            "type": "u64"
          },
          {
            "name": "earlyExit",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "patronExitedOld",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "burnAmount",
            "type": "u64"
          },
          {
            "name": "remainingAmount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "patronRebateDistributed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "buyer",
            "type": "pubkey"
          },
          {
            "name": "seller",
            "type": "pubkey"
          },
          {
            "name": "rebateAmount",
            "type": "u64"
          },
          {
            "name": "swapAccount",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "patronRevoked",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "revokedBy",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "patronStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "applied"
          },
          {
            "name": "approved"
          },
          {
            "name": "revoked"
          }
        ]
      }
    },
    {
      "name": "proposalCancelled",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposalId",
            "type": "u64"
          },
          {
            "name": "canceller",
            "type": "pubkey"
          },
          {
            "name": "cancelledAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "proposalCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposalId",
            "type": "u64"
          },
          {
            "name": "proposer",
            "type": "pubkey"
          },
          {
            "name": "title",
            "type": "string"
          },
          {
            "name": "description",
            "type": "string"
          },
          {
            "name": "proposalType",
            "type": {
              "defined": {
                "name": "proposalType"
              }
            }
          },
          {
            "name": "votingEndsAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "proposalExecuted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposalId",
            "type": "u64"
          },
          {
            "name": "executor",
            "type": "pubkey"
          },
          {
            "name": "executedAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "proposalFinalized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposalId",
            "type": "u64"
          },
          {
            "name": "status",
            "type": {
              "defined": {
                "name": "proposalStatus"
              }
            }
          },
          {
            "name": "votesFor",
            "type": "u32"
          },
          {
            "name": "votesAgainst",
            "type": "u32"
          },
          {
            "name": "quorumReached",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "proposalStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "active"
          },
          {
            "name": "passed"
          },
          {
            "name": "rejected"
          },
          {
            "name": "executed"
          },
          {
            "name": "cancelled"
          }
        ]
      }
    },
    {
      "name": "proposalType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "textProposal"
          },
          {
            "name": "parameterChange"
          },
          {
            "name": "treasurySpend"
          },
          {
            "name": "seatManagement"
          },
          {
            "name": "emergencyAction"
          }
        ]
      }
    },
    {
      "name": "rewardPool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "treasury",
            "type": "pubkey"
          },
          {
            "name": "tweetNumber",
            "type": "u64"
          },
          {
            "name": "mintedAccum",
            "type": "u64"
          },
          {
            "name": "burned",
            "type": "u64"
          },
          {
            "name": "airdropped",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "rewardPoolInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "treasury",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "stakingAction",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "lock"
          },
          {
            "name": "unlock"
          },
          {
            "name": "yieldClaim"
          },
          {
            "name": "roleChange"
          }
        ]
      }
    },
    {
      "name": "stakingHistoryEntry",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "action",
            "type": {
              "defined": {
                "name": "stakingAction"
              }
            }
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          },
          {
            "name": "role",
            "type": {
              "defined": {
                "name": "userRole"
              }
            }
          },
          {
            "name": "lockDurationMonths",
            "type": "u8"
          },
          {
            "name": "yieldAmount",
            "type": "u64"
          },
          {
            "name": "additionalData",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "swapCancelled",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seller",
            "type": "pubkey"
          },
          {
            "name": "otcSwap",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "swapCompleted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seller",
            "type": "pubkey"
          },
          {
            "name": "buyer",
            "type": "pubkey"
          },
          {
            "name": "otcSwap",
            "type": "pubkey"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          },
          {
            "name": "solPayment",
            "type": "u64"
          },
          {
            "name": "rebateAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "swapInitiated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seller",
            "type": "pubkey"
          },
          {
            "name": "buyerRoleRequired",
            "type": {
              "defined": {
                "name": "userRole"
              }
            }
          },
          {
            "name": "otcSwap",
            "type": "pubkey"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          },
          {
            "name": "solRate",
            "type": "u64"
          },
          {
            "name": "buyerRebate",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "tokensBurned",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "reason",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "tokensLocked",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "durationMonths",
            "type": "u8"
          },
          {
            "name": "lockStart",
            "type": "i64"
          },
          {
            "name": "lockEnd",
            "type": "i64"
          },
          {
            "name": "role",
            "type": {
              "defined": {
                "name": "userRole"
              }
            }
          }
        ]
      }
    },
    {
      "name": "tokensUnlocked",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "tokensVested",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "beneficiary",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "yieldAmount",
            "type": "u64"
          },
          {
            "name": "totalVested",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "updateRewardPoolParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "updateUserStatsParams",
      "docs": [
        "Update user statistics for patron qualification",
        "This allows admin to set mining amounts, wallet age, and community scores"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "phase1Mined",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "walletAgeDays",
            "type": {
              "option": "u32"
            }
          },
          {
            "name": "communityScore",
            "type": {
              "option": "u32"
            }
          },
          {
            "name": "phase2MiningCompleted",
            "type": {
              "option": "bool"
            }
          }
        ]
      }
    },
    {
      "name": "userClaim",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "initialized",
            "type": "bool"
          },
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "claimableAmount",
            "type": "u64"
          },
          {
            "name": "burnAmount",
            "type": "u64"
          },
          {
            "name": "lastClaimTimestamp",
            "type": "i64"
          },
          {
            "name": "role",
            "type": {
              "defined": {
                "name": "userRole"
              }
            }
          },
          {
            "name": "patronStatus",
            "type": {
              "defined": {
                "name": "patronStatus"
              }
            }
          },
          {
            "name": "patronApplicationTimestamp",
            "type": "i64"
          },
          {
            "name": "patronApprovalTimestamp",
            "type": "i64"
          },
          {
            "name": "lockedAmount",
            "type": "u64"
          },
          {
            "name": "lockStartTimestamp",
            "type": "i64"
          },
          {
            "name": "lockEndTimestamp",
            "type": "i64"
          },
          {
            "name": "lockDurationMonths",
            "type": "u8"
          },
          {
            "name": "lastYieldClaimTimestamp",
            "type": "i64"
          },
          {
            "name": "totalYieldClaimed",
            "type": "u64"
          },
          {
            "name": "soldEarly",
            "type": "bool"
          },
          {
            "name": "minedInPhase2",
            "type": "bool"
          },
          {
            "name": "daoEligible",
            "type": "bool"
          },
          {
            "name": "daoSeatHolder",
            "type": "bool"
          },
          {
            "name": "daoSeatAcquiredTimestamp",
            "type": "i64"
          },
          {
            "name": "totalMinedPhase1",
            "type": "u64"
          },
          {
            "name": "walletAgeDays",
            "type": "u32"
          },
          {
            "name": "communityScore",
            "type": "u32"
          },
          {
            "name": "patronQualificationScore",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "userClaimInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "userRole",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "staker"
          },
          {
            "name": "patron"
          }
        ]
      }
    },
    {
      "name": "userStakingHistory",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "initialized",
            "type": "bool"
          },
          {
            "name": "totalEntries",
            "type": "u32"
          },
          {
            "name": "totalLocked",
            "type": "u64"
          },
          {
            "name": "totalUnlocked",
            "type": "u64"
          },
          {
            "name": "totalYieldClaimed",
            "type": "u64"
          },
          {
            "name": "firstStakeTimestamp",
            "type": "i64"
          },
          {
            "name": "lastActivityTimestamp",
            "type": "i64"
          },
          {
            "name": "entries",
            "type": {
              "vec": {
                "defined": {
                  "name": "stakingHistoryEntry"
                }
              }
            }
          }
        ]
      }
    },
    {
      "name": "vestingCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "roleType",
            "type": {
              "defined": {
                "name": "vestingRoleType"
              }
            }
          },
          {
            "name": "startTimestamp",
            "type": "i64"
          },
          {
            "name": "unlockTimestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "vestingRoleType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "staker"
          },
          {
            "name": "patron"
          }
        ]
      }
    },
    {
      "name": "vestingSchedule",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "beneficiary",
            "type": "pubkey"
          },
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "vestedAmount",
            "type": "u64"
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "cliffTime",
            "type": "i64"
          },
          {
            "name": "durationMonths",
            "type": "u8"
          },
          {
            "name": "vestingType",
            "type": {
              "defined": {
                "name": "vestingType"
              }
            }
          },
          {
            "name": "isActive",
            "type": "bool"
          },
          {
            "name": "lastClaimTime",
            "type": "i64"
          },
          {
            "name": "yieldRate",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "vestingScheduleCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "beneficiary",
            "type": "pubkey"
          },
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "durationMonths",
            "type": "u8"
          },
          {
            "name": "vestingType",
            "type": {
              "defined": {
                "name": "vestingType"
              }
            }
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "vestingType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "staker"
          },
          {
            "name": "patron"
          }
        ]
      }
    },
    {
      "name": "vestingWithdrawn",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "yieldAmount",
            "type": "u64"
          },
          {
            "name": "totalWithdrawal",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "voteCast",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposalId",
            "type": "u64"
          },
          {
            "name": "voter",
            "type": "pubkey"
          },
          {
            "name": "voteFor",
            "type": "bool"
          },
          {
            "name": "votingPower",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "yieldClaimed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "yieldAmount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "snake_contract::instructions::otc_swap_enhanced::SwapType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "exiterToPatron"
          },
          {
            "name": "exiterToTreasury"
          },
          {
            "name": "patronToPatron"
          }
        ]
      }
    },
    {
      "name": "snake_contract::state::otc_swap::SwapType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "exiterToPatron"
          },
          {
            "name": "exiterToTreasury"
          },
          {
            "name": "patronToPatron"
          }
        ]
      }
    }
  ]
};
