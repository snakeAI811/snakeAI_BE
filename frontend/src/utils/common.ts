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

const KNOWN_ERROR_PATTERNS: { match: RegExp; message: string }[] = [
  {
    match: /prior credit/i,
    message:
      "The account has no SOL balance. This usually happens if fees or rent are due, or after a localnet validator restart when balances reset."
  },
  {
    match: /already in use/i,
    message:
      "This account or token has already been claimed or allocated."
  }
];

function matchKnownErrorFromLogs(logs: string[]): string | null {
  console.log('Matching known errors from logs:', logs);
  for (const log of logs) {
    for (const pattern of KNOWN_ERROR_PATTERNS) {
      if (pattern.match.test(log)) {
        return pattern.message;
      }
    }
  }
  return null;
}

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
  console.error('Parsing smart contract error:--------', error.message);
  // 1. Check SNAKE errors
  if (error.logs) {
    const snakeMessage = extractSnakeErrorMessage(error.logs);
    if (snakeMessage) return snakeMessage;

    // 2. Check known patterns in logs
    const knownLogMessage = matchKnownErrorFromLogs(error.logs);
    if (knownLogMessage) return knownLogMessage;
  }

  // 3. Check structured data
  if (error.data) {
    if (error.data.err) {
      const known = KNOWN_ERROR_PATTERNS.find(p =>
        p.match.test(error.data.err)
      );
      if (known) return known.message;
      return `Smart contract error: ${error.data.err}`;
    }
    if (Array.isArray(error.data.logs) && error.data.logs.length > 0) {
      const snakeMessage = extractSnakeErrorMessage(error.data.logs);
      if (snakeMessage) return snakeMessage;

      const knownLogMessage = matchKnownErrorFromLogs(error.data.logs);
      if (knownLogMessage) return knownLogMessage;
    }
  }

  // 4. Fallback to error.message
  if (error.message) {
    const knownLogMessage = matchKnownErrorFromLogs([error.message]);
    if (knownLogMessage) return knownLogMessage;
    return error.message;
  }

  // 5. Fallback for JSON-RPC style
  if (error.code && typeof error.code === "number") {
    return `RPC Error ${error.code}`;
  }

  // 6. Unknown
  return "An unknown error occurred";
}