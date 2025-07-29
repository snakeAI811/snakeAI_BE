// Custom hook for handling smart contract errors consistently

import { useCallback } from 'react';
import { parseSmartContractError } from '../utils/errorParser';
import { useToast } from '../contexts/ToastContext';

/**
 * Hook for consistent error handling across components
 * @returns Object with error handling utilities
 */
export const useErrorHandler = () => {
    const { showError, showWarning } = useToast();

    /**
     * Handle and display smart contract errors
     * @param error - Error object from transaction or API call
     * @param customMessage - Optional custom prefix message
     */
    const handleError = useCallback((error: any, customMessage?: string) => {
        const cleanErrorMessage = parseSmartContractError(error);
        const finalMessage = customMessage ? `${customMessage}: ${cleanErrorMessage}` : cleanErrorMessage;
        
        console.error('Error occurred:', error);
        showError(finalMessage);
    }, [showError]);

    /**
     * Handle warnings (non-critical errors)
     * @param error - Error object
     * @param customMessage - Optional custom prefix message
     */
    const handleWarning = useCallback((error: any, customMessage?: string) => {
        const cleanErrorMessage = parseSmartContractError(error);
        const finalMessage = customMessage ? `${customMessage}: ${cleanErrorMessage}` : cleanErrorMessage;
        
        console.warn('Warning occurred:', error);
        showWarning(finalMessage);
    }, [showWarning]);

    /**
     * Get clean error message without displaying it
     * @param error - Error object
     * @returns Clean error message string
     */
    const getCleanErrorMessage = useCallback((error: any): string => {
        return parseSmartContractError(error);
    }, []);

    /**
     * Check if error is user rejection
     * @param error - Error object
     * @returns True if user rejected the transaction
     */
    const isUserRejection = useCallback((error: any): boolean => {
        const message = parseSmartContractError(error);
        return message.toLowerCase().includes('cancelled by user') || 
               message.toLowerCase().includes('user rejected');
    }, []);

    /**
     * Check if error is network related
     * @param error - Error object
     * @returns True if error is network related
     */
    const isNetworkError = useCallback((error: any): boolean => {
        const message = parseSmartContractError(error);
        return message.toLowerCase().includes('network error') || 
               message.toLowerCase().includes('failed to fetch') ||
               message.toLowerCase().includes('service temporarily unavailable');
    }, []);

    return {
        handleError,
        handleWarning,
        getCleanErrorMessage,
        isUserRejection,
        isNetworkError,
    };
};

// Example usage:
/*
const MyComponent = () => {
    const { handleError, isUserRejection } = useErrorHandler();
    
    const handleTransaction = async () => {
        try {
            await someSmartContractCall();
        } catch (error) {
            if (isUserRejection(error)) {
                // Handle user rejection quietly
                return;
            }
            handleError(error, 'Failed to complete transaction');
        }
    };
    
    return <button onClick={handleTransaction}>Execute Transaction</button>;
};
*/
