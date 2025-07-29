// Example usage of the error parser utility

import React from 'react';
import { parseSmartContractError, getErrorMessage } from './errorParser';

// Example error objects you might receive from smart contract transactions

// 1. Anchor error with error code
const anchorError = {
    message: "failed to send transaction: Transaction simulation failed: Error processing Instruction 0: custom program error: 0x1770",
    logs: [
        "Program 11111111111111111111111111111111 invoke [1]",
        "Program log: Error Code: 6000. Error Message: Swap is not active.",
        "Program 11111111111111111111111111111111 consumed 5000 of 200000 compute units",
        "Program 11111111111111111111111111111111 failed: custom program error: 0x1770"
    ]
};

// 2. User rejected error
const userRejectedError = {
    message: "User rejected the request.",
    code: 4001
};

// 3. Network error
const networkError = {
    message: "Failed to fetch",
    name: "NetworkError"
};

// 4. Insufficient funds error
const insufficientFundsError = {
    err: {
        InstructionError: [
            0,
            {
                Custom: 6005
            }
        ]
    },
    logs: [
        "Program log: Error Code: 6005. Error Message: Insufficient funds."
    ]
};

// Example component showing how to use the error parser
const ErrorParserExample: React.FC = () => {
    const handleDemoErrors = () => {
        console.log("=== Error Parser Demo ===");
        
        // Test different error types
        console.log("1. Anchor Error:", parseSmartContractError(anchorError));
        // Output: "Swap is not active"
        
        console.log("2. User Rejected:", parseSmartContractError(userRejectedError));
        // Output: "Transaction cancelled by user"
        
        console.log("3. Network Error:", parseSmartContractError(networkError));
        // Output: "Network error. Please check your connection and try again"
        
        console.log("4. Insufficient Funds:", parseSmartContractError(insufficientFundsError));
        // Output: "Insufficient funds"
        
        // Using getErrorMessage for general error handling
        console.log("5. General Error:", getErrorMessage(new Error("Something went wrong")));
        // Output: "Something went wrong"
    };

    // Example of how to use in a try-catch block
    const handleTransactionExample = async () => {
        try {
            // Simulated transaction that fails
            throw anchorError;
        } catch (error) {
            // Use the error parser to get a clean message
            const cleanError = parseSmartContractError(error);
            console.error("Transaction failed:", cleanError);
            // Show this clean message to the user instead of the raw error
            alert(cleanError); // "Swap is not active"
        }
    };

    return (
        <div style={{ padding: '20px' }}>
            <h3>Smart Contract Error Parser Demo</h3>
            <button onClick={handleDemoErrors} style={{ margin: '5px' }}>
                Test Error Parser (Check Console)
            </button>
            <button onClick={handleTransactionExample} style={{ margin: '5px' }}>
                Test Transaction Error
            </button>
            
            <div style={{ marginTop: '20px', padding: '10px', backgroundColor: '#f5f5f5' }}>
                <h4>How to use in your components:</h4>
                <pre style={{ fontSize: '12px' }}>{`
// Import the parser
import { parseSmartContractError } from '../utils/errorParser';

// In your try-catch blocks
try {
    await someSmartContractTransaction();
} catch (error) {
    // Instead of showing raw error:
    // showError(error.message); // ❌ Shows complex error
    
    // Use the parser for clean error:
    const cleanError = parseSmartContractError(error); // ✅ Shows "Insufficient funds"
    showError(cleanError);
}
                `}</pre>
            </div>
            
            <div style={{ marginTop: '20px', padding: '10px', backgroundColor: '#e8f5e8' }}>
                <h4>Supported Error Types:</h4>
                <ul>
                    <li>✅ Anchor program errors (with error codes)</li>
                    <li>✅ User rejected transactions</li>
                    <li>✅ Network/RPC errors</li>
                    <li>✅ Insufficient funds</li>
                    <li>✅ Blockhash expired</li>
                    <li>✅ Account not found</li>
                    <li>✅ Signature verification failed</li>
                    <li>✅ Custom program errors</li>
                </ul>
            </div>
        </div>
    );
};

export default ErrorParserExample;
