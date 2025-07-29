// Error parser utility for smart contract errors
import snakeContractIdl from '../services/snake_contract.json';

interface AnchorError {
    code: number;
    name: string;
    msg: string;
}

interface SolanaTransactionError {
    message?: string;
    logs?: string[];
    err?: any;
}

/**
 * Parse smart contract error and extract human-readable message
 * @param error - Error object from transaction or API call
 * @returns Clean error message
 */
export function parseSmartContractError(error: any): string {
    try {
        // Handle different error formats
        let errorMessage = '';
        let errorCode: number | null = null;
        let logs: string[] = [];

        // Extract error details from different error formats
        if (error?.message) {
            errorMessage = error.message;
        }

        if (error?.err) {
            if (typeof error.err === 'object' && error.err.InstructionError) {
                const instructionError = error.err.InstructionError[1];
                if (instructionError.Custom) {
                    errorCode = instructionError.Custom;
                }
            }
        }

        if (error?.logs) {
            logs = error.logs;
        }

        // Try to extract error code from logs
        if (logs.length > 0 && !errorCode) {
            for (const log of logs) {
                // Look for "Program log: AnchorError thrown in programs/[program]/src/[file]. Error Code: [code]. Error Number: [number]. Error Message: [message]."
                const anchorErrorMatch = log.match(/Error Code: (\d+)/);
                if (anchorErrorMatch) {
                    errorCode = parseInt(anchorErrorMatch[1]);
                    break;
                }

                // Look for "Program log: Error: [message]"
                const programLogMatch = log.match(/Program log: Error: (.+)/);
                if (programLogMatch) {
                    return programLogMatch[1].trim();
                }

                // Look for custom error messages in logs
                const customErrorMatch = log.match(/Program log: (.+)/);
                if (customErrorMatch && !customErrorMatch[1].includes('Instruction:')) {
                    const logMessage = customErrorMatch[1].trim();
                    if (logMessage.length > 0 && !logMessage.startsWith('Consumed')) {
                        errorMessage = logMessage;
                    }
                }
            }
        }

        // Try to extract error code from error message
        if (!errorCode && errorMessage) {
            const codeMatch = errorMessage.match(/Error Code: (\d+)/);
            if (codeMatch) {
                errorCode = parseInt(codeMatch[1]);
            }

            // Handle "custom program error: 0x[hex]" format
            const customErrorMatch = errorMessage.match(/custom program error: 0x([a-fA-F0-9]+)/);
            if (customErrorMatch) {
                errorCode = parseInt(customErrorMatch[1], 16);
            }
        }

        // Look up error code in IDL
        if (errorCode !== null && snakeContractIdl.errors) {
            const anchorError = snakeContractIdl.errors.find((err: AnchorError) => err.code === errorCode);
            if (anchorError) {
                return anchorError.msg;
            }
        }

        // Handle common Solana/Anchor errors
        if (errorMessage) {
            // User rejected transaction
            if (errorMessage.includes('User rejected') || errorMessage.includes('cancelled')) {
                return 'Transaction cancelled by user';
            }

            // Insufficient funds
            if (errorMessage.includes('insufficient funds') || errorMessage.includes('Insufficient funds')) {
                return 'Insufficient funds for this transaction';
            }

            // Network/RPC errors
            if (errorMessage.includes('Failed to fetch') || errorMessage.includes('NetworkError')) {
                return 'Network error. Please check your connection and try again';
            }

            if (errorMessage.includes('503') || errorMessage.includes('502')) {
                return 'Service temporarily unavailable. Please try again';
            }

            // Blockhash errors
            if (errorMessage.includes('Blockhash not found') || errorMessage.includes('block height exceeded')) {
                return 'Transaction expired. Please try again';
            }

            // Account errors
            if (errorMessage.includes('Account does not exist')) {
                return 'Account not found. You may need to initialize it first';
            }

            // Signature verification
            if (errorMessage.includes('signature verification failed')) {
                return 'Transaction signature verification failed';
            }

            // Program errors
            if (errorMessage.includes('Program failed to complete')) {
                return 'Smart contract execution failed';
            }

            // Extract clean message if it contains useful info
            const cleanMessagePatterns = [
                /Error Message: (.+?)\.?$/,
                /failed to send transaction: (.+)$/i,
                /Transaction simulation failed: (.+)$/i,
                /Error: (.+)$/,
            ];

            for (const pattern of cleanMessagePatterns) {
                const match = errorMessage.match(pattern);
                if (match && match[1] && match[1].trim().length > 0) {
                    return match[1].trim();
                }
            }

            // Return first meaningful part of error message
            const lines = errorMessage.split('\n');
            for (const line of lines) {
                const cleanLine = line.trim();
                if (cleanLine.length > 0 && !cleanLine.includes('at ') && !cleanLine.includes('stack trace')) {
                    return cleanLine;
                }
            }
        }

        // Fallback to generic message
        return 'Transaction failed. Please try again';

    } catch (parseError) {
        console.error('Error parsing smart contract error:', parseError);
        return 'An unexpected error occurred';
    }
}

/**
 * Get user-friendly error message for display
 * @param error - Any error object
 * @returns Clean, user-friendly error message
 */
export function getErrorMessage(error: any): string {
    if (!error) {
        return 'Unknown error occurred';
    }

    // First try to parse as smart contract error
    const smartContractError = parseSmartContractError(error);
    if (smartContractError && smartContractError !== 'Transaction failed. Please try again') {
        return smartContractError;
    }

    // Fallback to basic error handling
    if (error instanceof Error) {
        return error.message;
    }

    if (typeof error === 'string') {
        return error;
    }

    if (error.message) {
        return error.message;
    }

    return 'Unknown error occurred';
}

/**
 * Map of known error codes to user-friendly messages
 */
export const ERROR_MESSAGES = {
    6000: 'Swap is not active',
    6001: 'Swap already accepted', 
    6002: 'Invalid token amount',
    6003: 'Invalid SOL rate',
    6004: 'Invalid rebate percentage',
    6005: 'Insufficient funds',
    6006: 'Math overflow error',
    6007: 'Invalid role for this operation',
} as const;
