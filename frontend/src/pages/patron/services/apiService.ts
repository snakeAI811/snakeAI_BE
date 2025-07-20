// API Service for Patron Framework
const API_BASE_URL = process.env.REACT_APP_API_BASE_URL || 'http://localhost:8000/api/v1';

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

// Get session token from cookies
function getSessionToken(): string | null {
  const cookies = document.cookie.split(';');
  for (let cookie of cookies) {
    const [name, value] = cookie.trim().split('=');
    if (name === 'SID') {
      return value;
    }
  }
  return null;
}

// Helper function to make authenticated API calls
async function apiCall<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<ApiResponse<T>> {
  try {
    const sessionToken = getSessionToken();
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...options.headers as Record<string, string>,
    };

    // Add Bearer token if session exists
    if (sessionToken) {
      headers['Authorization'] = `Bearer ${sessionToken}`;
    }

    const response = await fetch(`${API_BASE_URL}${endpoint}`, {
      ...options,
      credentials: 'include', // Include cookies
      headers,
    });

    if (response.ok) {
      const data = await response.json();
      return { success: true, data };
    } else {
      const errorText = await response.text();
      return { success: false, error: errorText || 'API call failed' };
    }
  } catch (error) {
    return { 
      success: false, 
      error: error instanceof Error ? error.message : 'Network error' 
    };
  }
}

// Role Management API calls
export const roleApi = {
  selectRole: async (role: 'None' | 'Staker' | 'Patron') => {
    return apiCall<string>('/user/select_role', {
      method: 'POST',
      body: JSON.stringify({ role }),
    });
  },

  getUserRole: async () => {
    return apiCall<{ role: string; status?: string }>('/test/profile', {
      method: 'GET',
    });
  },
};

// Token Management API calls
export const tokenApi = {
  lockTokens: async (amount: number, role: string) => {
    return apiCall<string>('/user/lock_tokens', {
      method: 'POST',
      body: JSON.stringify({ amount, role }),
    });
  },

  unlockTokens: async () => {
    return apiCall<string>('/user/unlock_tokens', {
      method: 'POST',
    });
  },

  claimYield: async () => {
    return apiCall<string>('/user/claim_yield', {
      method: 'POST',
    });
  },

  claimTokensWithRole: async (role: string) => {
    return apiCall<string>('/user/claim_tokens_with_role', {
      method: 'POST',
      body: JSON.stringify({ role }),
    });
  },

  getTokenInfo: async () => {
    return apiCall<{
      balance: number;
      locked: number;
      staked: number;
      rewards: number;
    }>('/test/token-info', {
      method: 'GET',
    });
  },

  getMiningStatus: async () => {
    return apiCall<{
      current_phase: 'Phase1' | 'Phase2';
      phase1_tweet_count: number;
      phase2_tweet_count: number;
      total_phase1_mined: number;
      total_phase2_mined: number;
      phase2_start_date: string;
    }>('/user/mining_status', {
      method: 'GET',
    });
  },
};

// Patron Application API calls
export const patronApi = {
  applyForPatron: async (reason: string) => {
    return apiCall<string>('/user/apply_patron', {
      method: 'POST',
      body: JSON.stringify({ reason }),
    });
  },

  approvePatron: async (applicationId: string, approved: boolean) => {
    return apiCall<string>('/user/approve_patron', {
      method: 'POST',
      body: JSON.stringify({ application_id: applicationId, approved }),
    });
  },

  getApplicationStatus: async () => {
    return apiCall<{
      id: string;
      status: 'pending' | 'approved' | 'rejected';
      submitted_at: string;
      reason?: string;
    }>('/test/patron-application', {
      method: 'GET',
    });
  },

  getPendingApplications: async () => {
    return apiCall<Array<{
      id: string;
      status: string;
      submitted_at: string;
      reason?: string;
    }>>('/user/pending_patron_applications', {
      method: 'GET',
    });
  },
};

// OTC Trading API calls
export const otcApi = {
  initiateSwap: async (params: {
    token_amount: number;
    sol_rate: number;
    buyer_rebate: number;
    buyer_role_required: string;
  }) => {
    return apiCall<string>('/user/initiate_otc_swap', {
      method: 'POST',
      body: JSON.stringify(params),
    });
  },

  acceptSwap: async (sellerPubkey: string) => {
    return apiCall<string>('/user/accept_otc_swap', {
      method: 'POST',
      body: JSON.stringify({ seller_pubkey: sellerPubkey }),
    });
  },

  cancelSwap: async () => {
    return apiCall<string>('/user/cancel_otc_swap', {
      method: 'POST',
    });
  },

  getActiveSwaps: async () => {
    return apiCall<Array<{
      id: string;
      seller: string;
      token_amount: number;
      sol_rate: number;
      buyer_rebate: number;
      buyer_role_required: string;
      status: string;
      created_at: string;
    }>>('/test/active-swaps', {
      method: 'GET',
    });
  },

  getMySwaps: async () => {
    return apiCall<Array<{
      id: string;
      token_amount: number;
      sol_rate: number;
      buyer_rebate: number;
      buyer_role_required: string;
      status: string;
      created_at: string;
    }>>('/test/my-swaps', {
      method: 'GET',
    });
  },
};

// Vesting API calls
export const vestingApi = {
  createVesting: async (amount: number, roleType: string) => {
    return apiCall<string>('/user/create_vesting', {
      method: 'POST',
      body: JSON.stringify({ amount, role_type: roleType }),
    });
  },

  withdrawVesting: async () => {
    return apiCall<string>('/user/withdraw_vesting', {
      method: 'POST',
    });
  },

  getVestingInfo: async () => {
    return apiCall<{
      amount: number;
      role_type: string;
      created_at: string;
      withdrawal_available: boolean;
    }>('/test/vesting-info', {
      method: 'GET',
    });
  },
};

// User management API calls
export const userApi = {
  setWalletAddress: async (walletAddress: string) => {
    return apiCall<any>('/user/wallet_address', {
      method: 'POST',
      body: JSON.stringify({ wallet_address: walletAddress }),
    });
  },

  getMe: async () => {
    return apiCall<any>('/user/me', {
      method: 'GET',
    });
  },
};

const apiService = {
  roleApi,
  tokenApi,
  patronApi,
  otcApi,
  vestingApi,
  userApi,
};

export default apiService;
