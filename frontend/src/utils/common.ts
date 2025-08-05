export const safeFetch = async <T>(
  apiFn: () => Promise<{ success: boolean; data: T; error?: any }>,
  onSuccess: (data: T) => void,
  label = 'fetch'
) => {
  try {
    const result = await apiFn();
    if (result.success && result.data) {
      onSuccess(result.data);
    } else {
      console.error(`Failed to ${label}:`, result.error);
    }
  } catch (error) {
    console.error(`Error during ${label}:`, error);
  }
};

function extractSnakeErrorMessage(logs: string[]): string | null {
  const errorLog = logs.find((line) =>
    line.includes("[SNAKE:")
  );

  if (!errorLog) return null;

  const match = errorLog.match(/\[SNAKE:(\d+)\]\s(.+)/);
  if (match) {
    const [, code, message] = match;
    return `[Error ${code}] ${message}`;
  }

  return null;
}
export function parseSmartContractError(error: any): string {
  if (error.logs) {
    const message = extractSnakeErrorMessage(error.logs);
    if (message) return message;
  }

  if (error.message) {
    return error.message;
  }

  return 'An unknown error occurred';
}