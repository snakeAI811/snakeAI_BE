import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import { useToast } from '../../contexts/ToastContext';
import { useWalletContext } from '../../contexts/WalletContext';
import { SOLANA_RPC_URL } from '../../config/program';
import WalletGuard from "../../components/WalletGuard";
import ResponsiveMenu from "../../components/ResponsiveMenu";
import { otcApi, tokenApi, roleApi, OtcSwapResponse } from '../patron/services/apiService';
import { useErrorHandler } from '../../hooks/useErrorHandler';
import { useAuth } from '../../contexts/AuthContext';

interface OTCOrder {
  orderId: string;
  seller?: string;
  sellerWallet: string;
  amount: string;
  price: string;
  isActive: boolean;
  status: string;
  createdAt: string;
  expiresAt?: string;
  buyerRestrictions: {
    patronsOnly: boolean;
    treasuryOnly: boolean;
    minPatronScore: number;
  };
  buyerRebate?: number;
  sellerId?: string;
  totalSolPayment?: number;
  netSolPayment?: number;
  canAccept?: boolean;
  isExpired?: boolean;
}

const OTCTrading: React.FC = () => {
  const { showSuccess, showError, showInfo } = useToast();
  const { publicKey, connected } = useWalletContext();
  const { handleError, handleWarning } = useErrorHandler();
  const { user, logout } = useAuth();
  const { signTransaction } = useWallet();

  // Create connection with proper configuration
  const connection = new Connection(SOLANA_RPC_URL, {
    commitment: 'confirmed',
    confirmTransactionInitialTimeout: 60000,
  });

  const [orders, setOrders] = useState<OTCOrder[]>([]);
  const [myOrders, setMyOrders] = useState<OTCOrder[]>([]);
  const [loading, setLoading] = useState(false);
  const [activeTab, setActiveTab] = useState<'all' | 'my'>('all');
  const [createOrderForm, setCreateOrderForm] = useState({
    amount: '',
    price: '',
    swapType: 'ExiterToPatron' as 'ExiterToPatron' | 'ExiterToTreasury' | 'PatronToPatron',
  });
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [userRole, setUserRole] = useState<string>('none');
  const [refreshing, setRefreshing] = useState(false);

  useEffect(() => {
    if (connected && publicKey) {
      fetchUserRole();
    }
  }, [connected, publicKey]);

  const fetchUserRole = async () => {
    try {
      const result = await roleApi.getUserRole();
      if (result.success && result.data) {
        setUserRole(result.data.role || 'none');
      }
    } catch (error) {
      console.error('Error fetching user role:', error);
      setUserRole('none');
    }
  };

  useEffect(() => {
    if (publicKey && connected) {
      fetchOrders();
      fetchMyOrders();
    }
  }, [publicKey, connected]);

  const fetchOrders = async () => {
    if (!connected) return;

    try {
      setRefreshing(true);
      console.log('🔍 Fetching active orders...');
      const result = await otcApi.getActiveSwaps();
      console.log('🔍 Active orders result:', result);

      if (result.success && result.data) {
        const { swaps = [], total_count, page, per_page } = result.data;
        console.log(`📊 Found ${total_count} total swaps, showing page ${page} (${swaps.length} items)`);

        const activeOrders: OTCOrder[] = swaps.map((swap: any) => ({
          orderId: swap.id,
          seller: swap.seller_username || swap.seller || 'Unknown',
          sellerWallet: swap.seller_wallet,
          amount: (swap.token_amount).toString(), // Convert from lamports to tokens
          price: (swap.sol_rate / 1_000_000_000).toString(), // Convert lamports to SOL
          isActive: swap.status === 'active',
          status: swap.status,
          createdAt: new Date(swap.created_at).toLocaleString(),
          expiresAt: new Date(swap.expires_at).toLocaleString(),
          buyerRestrictions: {
            patronsOnly: swap.buyer_role_required === 'patron',
            treasuryOnly: swap.buyer_role_required === 'treasury',
            minPatronScore: 0,
          },
          buyerRebate: swap.buyer_rebate,
          sellerId: swap.seller_id,
          canAccept: swap.can_accept,
          isExpired: swap.is_expired,
        }));

        console.log('🔍 Processed active orders:', activeOrders);
        setOrders(activeOrders);
      } else {
        console.log('❌ Failed to fetch active orders:', result.error);
        if (result.error && !result.error.includes('Network error')) {
          showError(`Failed to fetch orders: ${result.error}`);
        }
      }
    } catch (error) {
      console.error('Error fetching orders:', error);
      handleError(error, 'Failed to fetch OTC orders');
    } finally {
      setRefreshing(false);
    }
  };

  const fetchMyOrders = async () => {
    if (!connected) return;

    try {
      console.log('🔍 Fetching my orders...');
      const result = await otcApi.getMySwaps();
      console.log('🔍 My orders result:', result);

      if (result.success && result.data) {
        const { active_swaps = [] } = result.data;
        console.log('🔍 Active swaps:', active_swaps);

        const myActiveOrders: OTCOrder[] = active_swaps.map((swap: any) => ({
          orderId: swap.id,
          sellerWallet: publicKey?.toString() || '',
          amount: (swap.token_amount).toString(), // Convert from lamports
          price: (swap.sol_rate / 1_000_000_000).toString(), // Convert lamports to SOL
          isActive: true,
          status: 'active',
          createdAt: new Date(swap.created_at).toLocaleString(),
          expiresAt: new Date(swap.expires_at).toLocaleString(),
          buyerRestrictions: {
            patronsOnly: swap.buyer_role_required === 'patron',
            treasuryOnly: swap.buyer_role_required === 'treasury',
            minPatronScore: 0,
          },
          buyerRebate: swap.buyer_rebate,
          totalSolPayment: swap.total_sol_payment,
          netSolPayment: swap.net_sol_payment,
          canAccept: swap.can_accept,
          isExpired: swap.is_expired,
        }));

        console.log('🔍 Processed my orders:', myActiveOrders);
        setMyOrders(myActiveOrders);
      } else {
        console.log('❌ Failed to fetch my orders:', result.error);
        if (result.error && !result.error.includes('Network error')) {
          console.log('My orders fetch error (non-critical):', result.error);
        }
      }
    } catch (error) {
      console.error('Error fetching my orders:', error);
      // Don't show error for my orders as it's less critical
    }
  };

  const createOrder = async () => {
    if (!connected || !publicKey) {
      showError('Please connect your wallet first');
      return;
    }

    if (!createOrderForm.amount || !createOrderForm.price) {
      showError('Please fill in all required fields');
      return;
    }

    const amount = parseFloat(createOrderForm.amount);
    const price = parseFloat(createOrderForm.price);

    if (amount <= 0 || price <= 0) {
      showError('Amount and price must be greater than 0');
      return;
    }

    if (price > 1) {
      showError('Price seems high. Please enter price in SOL (e.g., 0.001 for 0.001 SOL per token)');
      return;
    }

    try {
      setLoading(true);
      showInfo('Creating OTC swap order...');

      const swapData = {
        token_amount: amount, // Send as regular number, backend converts to lamports
        sol_rate: price * 1_000_000_000, // Convert SOL to lamports
        buyer_rebate: 200, // 2% rebate for patrons
        swap_type: createOrderForm.swapType,
        txSignature: ''
      };

      console.log('📤 Sending enhanced swap data:', swapData);
      const result = await otcApi.initiateSwapEnhanced(swapData);

      if (result.success && result.data) {
        if (result.requiresWalletSignature) {
          const { signature } = await handleWalletTransaction(
            result.data,
            'Please approve the OTC swap creation...',
            'Submitting OTC swap...',
            'Waiting for confirmation...',
            'OTC swap created successfully!'
          );
          swapData.txSignature = signature;
          // ✅ Submit txSignature to backend to mark swap active
          await otcApi.updateSwapSignature(swapData);
          showSuccess('OTC swap order created successfully!');
        } else {
          showSuccess('OTC swap order created successfully!');
        }

        setShowCreateForm(false);
        setCreateOrderForm({ amount: '', price: '', swapType: 'ExiterToPatron' });
        await Promise.all([fetchOrders(), fetchMyOrders()]);
      }
    } catch (error) {
      console.error('Error creating order:', error);
      handleError(error, 'Failed to create OTC order');
    } finally {
      setLoading(false);
    }
  };


  const executeOrder = async (order: OTCOrder) => {
    if (!connected || !publicKey) {
      showError('Please connect your wallet first');
      return;
    }

    try {
      setLoading(true);
      showInfo(`Accepting OTC swap from ${order.sellerWallet.slice(0, 8)}...${order.sellerWallet.slice(-8)}...`);

      const result = await otcApi.acceptSwap(order.sellerWallet);

      if (result.success && result.data) {
        // Handle transaction if needed
        if (typeof result.data === 'string') {
          const { signature } = await handleWalletTransaction(
            result.data,
            'Please approve the swap acceptance...',
            'Processing swap...',
            'Confirming swap...',
            'Swap executed successfully!'
          );
          // You can optionally send signature to backend if needed
        }

        showSuccess('OTC swap executed successfully!');
        await Promise.all([fetchOrders(), fetchMyOrders()]);
      } else {
        throw new Error(result.error || 'Failed to execute order');
      }
    } catch (error) {
      console.error('Error executing order:', error);
      handleError(error, 'Failed to execute OTC order');
    } finally {
      setLoading(false);
    }
  };


  // Improved cancelOrder function with better error handling
  const cancelOrder = async (orderId: string) => {
    if (!connected || !publicKey) {
      showError('Please connect your wallet first');
      return;
    }

    try {
      setLoading(true);
      showInfo('Preparing to cancel OTC swap order...');

      // Step 1: Get unsigned transaction with better error handling
      console.log('🔍 Requesting unsigned cancel transaction...');
      const unsignedTxResponse = await otcApi.cancelSwap();
      console.log('🔍 Cancel swap response:', unsignedTxResponse);

      // More robust response parsing
      let unsignedTxBase64: string;

      if (typeof unsignedTxResponse === 'string') {
        unsignedTxBase64 = unsignedTxResponse;
      } else if (unsignedTxResponse && typeof unsignedTxResponse === 'object') {
        const responseObj = unsignedTxResponse as any;

        // Try different possible response structures
        if (typeof responseObj.data === 'string') {
          unsignedTxBase64 = responseObj.data;
        } else if (responseObj.success && typeof responseObj.data === 'string') {
          unsignedTxBase64 = responseObj.data;
        } else if (typeof responseObj === 'string') {
          unsignedTxBase64 = responseObj;
        } else {
          throw new Error(`Unexpected response format: ${JSON.stringify(responseObj)}`);
        }
      } else {
        throw new Error(`Invalid response type: ${typeof unsignedTxResponse}`);
      }

      // Validate the base64 string
      if (!unsignedTxBase64 || typeof unsignedTxBase64 !== 'string') {
        throw new Error(`Invalid transaction data received: ${typeof unsignedTxBase64}`);
      }

      // Basic base64 validation
      try {
        atob(unsignedTxBase64); // This will throw if invalid base64
      } catch (e) {
        throw new Error('Invalid base64 transaction received from server');
      }

      console.log('✅ Unsigned transaction received, length:', unsignedTxBase64.length);

      // Step 2: Sign and send transaction with better error handling
      showInfo('Please approve the cancellation in your wallet...');

      let signature: string;
      let signedTxBase64: string;
      try {
        const res = await handleWalletTransaction(
          unsignedTxBase64,
          'Please approve the cancellation...',
          'Submitting cancellation...',
          'Confirming cancellation...'
        );
        signature = res.signature;
        signedTxBase64 = res.signedTxBase64;
        console.log('✅ Transaction signed and confirmed:', signature);
      } catch (walletError: any) {
        console.error('❌ Wallet transaction failed:', walletError);

        // Better wallet error handling
        if (walletError.message?.includes('User rejected') ||
          walletError.message?.includes('cancelled by user') ||
          walletError.code === 4001) {
          throw new Error('Transaction was cancelled by user');
        } else if (walletError.message?.includes('Insufficient funds')) {
          throw new Error('Insufficient SOL for transaction fees');
        } else if (walletError.message?.includes('timeout')) {
          throw new Error('Transaction timeout - please try again');
        } else {
          throw new Error(`Wallet error: ${walletError.message || 'Unknown error'}`);
        }
      }

      // Step 3: Update database with the signed transaction
      showInfo('Updating swap record...');

      try {
        console.log('📤 Sending signed transaction to backend, length:', signedTxBase64.length);

        const cancelResponse = await otcApi.processCancelSwap(signedTxBase64);
        console.log('✅ Backend processing successful:', cancelResponse);

        const cancelledSwap = (cancelResponse as any).data || cancelResponse;

        if (cancelledSwap && cancelledSwap.status) {
          showSuccess(`OTC swap canceled successfully! Status: ${cancelledSwap.status}`);
        } else {
          showSuccess('OTC swap canceled successfully!');
        }

      } catch (updateError: any) {
        console.error('❌ Database update failed:', updateError);

        // Parse backend error messages
        let errorMessage = 'Failed to update database';

        if (updateError.message?.includes('No active swap')) {
          errorMessage = 'No active swap found to cancel';
        } else if (updateError.message?.includes('Transaction not signed')) {
          errorMessage = 'Transaction signature invalid';
        } else if (updateError.message?.includes('already processed')) {
          errorMessage = 'Transaction already processed';
        } else if (updateError.message?.includes('insufficient funds')) {
          errorMessage = 'Insufficient SOL for transaction fees';
        } else if (updateError.message?.includes('timeout')) {
          errorMessage = 'Request timeout - please try again';
        } else if (updateError.response?.status === 500) {
          errorMessage = 'Server error - please try again';
        }

        // If transaction was confirmed on blockchain but DB update failed
        if (signature && updateError.message?.includes('database update failed')) {
          showError(`Transaction confirmed (${signature.slice(0, 8)}...) but database update failed. Please refresh.`);
        } else {
          throw new Error(errorMessage);
        }
      }

      // Refresh the data
      await Promise.all([fetchOrders(), fetchMyOrders()]);

    } catch (error: any) {
      console.error('❌ Cancel order error:', error);

      // Comprehensive error handling
      if (error.message?.includes('User rejected') || error.message?.includes('cancelled by user')) {
        showError('Transaction was cancelled');
      } else if (error.message?.includes('No active swap')) {
        showError('No active swap found to cancel');
      } else if (error.message?.includes('connect to Solana network')) {
        showError('Network connection error. Please check your connection.');
      } else if (error.message?.includes('insufficient')) {
        showError('Insufficient SOL for transaction fees');
      } else if (error.message?.includes('timeout')) {
        showError('Request timeout. Please try again.');
      } else if (error.message?.includes('Invalid transaction')) {
        showError('Transaction format error. Please try again.');
      } else {
        showError(error.message || 'Failed to cancel OTC order');
      }
    } finally {
      setLoading(false);
    }
  };

  // Improved function to get signed transaction bytes
  async function getSignedTransactionBytesImproved(unsignedTxBase64: string): Promise<Uint8Array> {
    try {
      // Decode the unsigned transaction
      const unsignedTxBytes = new Uint8Array(
        atob(unsignedTxBase64)
          .split('')
          .map(char => char.charCodeAt(0))
      );

      console.log('🔍 Unsigned transaction bytes length:', unsignedTxBytes.length);

      // Deserialize to get the transaction structure
      const transaction = Transaction.from(unsignedTxBytes);
      console.log('🔍 Transaction deserialized:', {
        signatures: transaction.signatures.length,
        instructions: transaction.instructions.length
      });

      // Re-sign the transaction (this should have been done by handleWalletTransaction)
      if (!connected || !publicKey) {
        throw new Error('Wallet not connected');
      }

      // The transaction should already be signed, just serialize it
      const signedBytes = transaction.serialize();
      console.log('🔍 Signed transaction bytes length:', signedBytes.length);

      return signedBytes;

    } catch (error) {
      console.error('❌ Error processing transaction bytes:', error);
      throw new Error('Failed to process transaction for signing');
    }
  }

  // Helper function to handle wallet transactions
  const handleWalletTransaction = async (
    base64Transaction: string,
    approveMessage: string,
    submitMessage: string,
    confirmMessage: string,
    successMessage?: string
  ): Promise<{ signature: string; signedTxBase64: string }> => {
    showInfo(approveMessage);

    // Decode transaction
    const transactionBytes = Uint8Array.from(atob(base64Transaction), c => c.charCodeAt(0));
    const transaction = Transaction.from(transactionBytes);

    // Update with fresh blockhash
    showInfo('Getting fresh blockhash...');
    // Step 1: Get latest blockhash
    const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash('confirmed');

    // Step 2: Set new blockhash + fee payer
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = new PublicKey(publicKey!);

    // Sign transaction
    if (typeof window !== 'undefined' && (window as any).solana) {
      // Step 3: Re-sign transaction with Phantom
      const signedTransaction = await (window as any).solana.signTransaction(transaction);

      // Step 4: Send transaction
      showInfo(submitMessage);
      const serialized = signedTransaction.serialize() as Uint8Array;
      const signature = await connection.sendRawTransaction(serialized, {
        skipPreflight: false,
        preflightCommitment: 'confirmed'
      });

      // Step 5: Confirm
      showInfo(confirmMessage);
      await connection.confirmTransaction(
        {
          signature,
          blockhash,
          lastValidBlockHeight,
        },
        'confirmed'
      );

      if (successMessage) {
        showInfo(successMessage);
      }

      const signedTxBase64 = btoa(String.fromCharCode(...Array.from(serialized)));
      return { signature, signedTxBase64 };
    } else {
      throw new Error('Phantom wallet not found');
    }
  };

  // Helper to get signed transaction bytes for database update
  const getSignedTransactionBytes = async (base64Transaction: string): Promise<Uint8Array> => {
    const transactionBytes = Uint8Array.from(atob(base64Transaction), c => c.charCodeAt(0));
    const transaction = Transaction.from(transactionBytes);

    const { blockhash } = await connection.getLatestBlockhash('confirmed');
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = new PublicKey(publicKey!);

    if (typeof window !== 'undefined' && (window as any).solana) {
      const signedTransaction = await (window as any).solana.signTransaction(transaction);
      return signedTransaction.serialize();
    }

    throw new Error('Phantom wallet not found');
  };

  const canBuyOrder = (order: OTCOrder) => {
    // Check if user can buy this order based on restrictions
    if (order.buyerRestrictions.patronsOnly && userRole !== 'patron') {
      return false;
    }
    if (order.buyerRestrictions.treasuryOnly && userRole !== 'treasury') {
      return false;
    }
    return !order.isExpired && order.canAccept !== false;
  };

  const getBuyerRestrictionBadge = (order: OTCOrder) => {
    if (order.buyerRestrictions.treasuryOnly) {
      return <span className="badge bg-primary">🏛️ Treasury Only</span>;
    }
    if (order.buyerRestrictions.patronsOnly) {
      return <span className="badge bg-warning">👑 Patrons Only</span>;
    }
    return <span className="badge bg-success">✅ Open to All</span>;
  };

  // Add this to your otcApi object (where you have cancelSwap and processCancelSwap)

  return (
    <div className="w-100 p-3" style={{ height: "100vh" }}>
      <div className="d-flex gap-4" style={{ height: "calc(100vh-60px)", paddingTop: '55px' }}>
        {/* Menu Begin */}
        <ResponsiveMenu />
        {/* Menu End */}
        <div className="custom-content">
          <div className="w-100">
            <div className="d-flex justify-content-between align-items-center">
              <div className="fs-1" style={{ lineHeight: 'normal' }}>🔄 OTC Trading</div>
              <div className="text-end d-flex align-items-center gap-2">
                <div className="fs-6 text-muted">
                  Connected: @{user?.twitter_username || 'Not authenticated'}
                </div>
                <button
                  onClick={async () => await logout()}
                  className="fs-6 fw-bold second-btn py-1 px-2 text-decoration-none text-center"
                >
                  LOGOUT
                </button>
              </div>
            </div>
          </div>
          <div className="custom-border-y custom-content-height d-flex flex-column px-3">
            <WalletGuard>
              <div className="">
                {!connected && (
                  <div className="alert alert-warning mb-4" role="alert">
                    <div className="d-flex">
                      <div className="flex-shrink-0">
                        <i className="bi bi-exclamation-triangle-fill text-warning"></i>
                      </div>
                      <div className="ms-3">
                        <h6 className="alert-heading">Wallet Not Connected</h6>
                        <p className="mb-0">Please connect your wallet to view and trade OTC orders.</p>
                      </div>
                    </div>
                  </div>
                )}

                <div className="card">
                  <div className="card-header">
                    <div className="d-flex justify-content-between align-items-center">
                      <div className="d-flex gap-2">
                        <button
                          onClick={() => setShowCreateForm(true)}
                          className="primary-btn"
                          disabled={loading || !connected}
                        >
                          {loading ? (
                            <>
                              <span className="spinner-border spinner-border-sm me-2" />
                              Creating...
                            </>
                          ) : (
                            'Create Order'
                          )}
                        </button>
                        <button
                          onClick={() => Promise.all([fetchOrders(), fetchMyOrders()])}
                          className="btn btn-outline-secondary"
                          disabled={refreshing || !connected}
                        >
                          {refreshing ? (
                            <>
                              <span className="spinner-border spinner-border-sm me-2" />
                              Refreshing...
                            </>
                          ) : (
                            <>
                              <i className="bi bi-arrow-clockwise me-1"></i>
                              Refresh
                            </>
                          )}
                        </button>
                      </div>
                    </div>
                    <p className="text-muted mt-2 mb-0">
                      Fixed-price token trading between Sellers and Patrons/Treasury
                    </p>

                    {/* Tabs */}
                    <div className="mt-3">
                      <ul className="nav nav-tabs">
                        <li className="nav-item">
                          <button
                            onClick={() => setActiveTab('all')}
                            className={`nav-link ${activeTab === 'all' ? 'active' : ''}`}
                          >
                            All Orders ({orders.length})
                          </button>
                        </li>
                        <li className="nav-item">
                          <button
                            onClick={() => setActiveTab('my')}
                            className={`nav-link ${activeTab === 'my' ? 'active' : ''}`}
                          >
                            My Orders ({myOrders.length})
                          </button>
                        </li>
                      </ul>
                    </div>
                  </div>

                  {showCreateForm && (
                    <div className="card-body border-bottom">
                      <h5 className="card-title">📝 Create Sell Order</h5>
                      <div className="row g-3">
                        <div className="col-md-4">
                          <label className="form-label">Amount (SNAKE)</label>
                          <input
                            type="number"
                            min="1"
                            value={createOrderForm.amount}
                            onChange={(e) => setCreateOrderForm({ ...createOrderForm, amount: e.target.value })}
                            className="form-control"
                            placeholder="e.g., 1000"
                          />
                          <div className="form-text">Number of SNAKE tokens to sell</div>
                        </div>
                        <div className="col-md-4">
                          <label className="form-label">Price per Token (SOL)</label>
                          <input
                            type="number"
                            step="0.0001"
                            min="0.0001"
                            max="1"
                            value={createOrderForm.price}
                            onChange={(e) => setCreateOrderForm({ ...createOrderForm, price: e.target.value })}
                            className="form-control"
                            placeholder="e.g., 0.001"
                          />
                          <div className="form-text">Price in SOL per token</div>
                        </div>
                        <div className="col-md-4">
                          <label className="form-label">Swap Type</label>
                          <select
                            value={createOrderForm.swapType}
                            onChange={(e) => setCreateOrderForm({
                              ...createOrderForm,
                              swapType: e.target.value as 'ExiterToPatron' | 'ExiterToTreasury' | 'PatronToPatron'
                            })}
                            className="form-select"
                          >
                            <option value="ExiterToPatron">Exiter → Patron</option>
                            <option value="ExiterToTreasury">Exiter → Treasury</option>
                            <option value="PatronToPatron">Patron → Patron</option>
                          </select>
                          <div className="form-text">Who can buy your tokens</div>
                        </div>
                      </div>

                      {createOrderForm.amount && createOrderForm.price && (
                        <div className="mt-3 p-3 bg-light rounded">
                          <h6>Order Summary:</h6>
                          <p className="mb-1">
                            <strong>Selling:</strong> {parseFloat(createOrderForm.amount).toLocaleString()} SNAKE
                          </p>
                          <p className="mb-1">
                            <strong>Total Value:</strong> {(parseFloat(createOrderForm.amount) * parseFloat(createOrderForm.price)).toFixed(4)} SOL
                          </p>
                          <p className="mb-0">
                            <strong>Type:</strong> {createOrderForm.swapType}
                          </p>
                        </div>
                      )}

                      <div className="mt-4 d-flex gap-2">
                        <button
                          onClick={createOrder}
                          disabled={loading || !createOrderForm.amount || !createOrderForm.price}
                          className="btn btn-success"
                        >
                          {loading ? (
                            <>
                              <span className="spinner-border spinner-border-sm me-2" />
                              Creating...
                            </>
                          ) : (
                            'Create Order'
                          )}
                        </button>
                        <button
                          onClick={() => setShowCreateForm(false)}
                          className="btn btn-secondary"
                        >
                          Cancel
                        </button>
                      </div>
                    </div>
                  )}

                  <div className="card-body">
                    <div className="d-flex justify-content-between align-items-center mb-4">
                      <h5 className="card-title mb-0">
                        {activeTab === 'all' ? '📊 All Active Orders' : '📋 My Orders'}
                      </h5>
                      {(activeTab === 'all' ? orders : myOrders).length > 0 && (
                        <small className="text-muted">
                          {(activeTab === 'all' ? orders : myOrders).length} order(s) found
                        </small>
                      )}
                    </div>

                    {loading && !refreshing ? (
                      <div className="text-center py-5">
                        <div className="spinner-border text-primary" role="status">
                          <span className="visually-hidden">Loading...</span>
                        </div>
                        <p className="mt-3 text-muted">Loading orders...</p>
                      </div>
                    ) : (activeTab === 'all' ? orders : myOrders).length === 0 ? (
                      <div className="text-center py-5">
                        <div className="mb-3">
                          <i className="bi bi-inbox fs-1 text-muted"></i>
                        </div>
                        <p className="text-muted">
                          {activeTab === 'all' ? 'No active orders found' : 'You have no active orders'}
                        </p>
                        {activeTab === 'my' && (
                          <button
                            onClick={() => setShowCreateForm(true)}
                            className="primary-btn mt-2"
                            disabled={!connected}
                          >
                            Create Your First Order
                          </button>
                        )}
                      </div>
                    ) : (
                      <div className="table-responsive">
                        <table className="table table-hover">
                          <thead className="table-light">
                            <tr>
                              <th scope="col">Seller</th>
                              <th scope="col">Amount</th>
                              <th scope="col">Price</th>
                              <th scope="col">Total Value</th>
                              <th scope="col">Restrictions</th>
                              <th scope="col">Created</th>
                              <th scope="col">Status</th>
                              <th scope="col">Actions</th>
                            </tr>
                          </thead>
                          <tbody>
                            {(activeTab === 'all' ? orders : myOrders).map((order) => (
                              <tr key={order.orderId} className={order.isExpired ? 'table-warning' : ''}>
                                <td>
                                  <div className="d-flex align-items-center">
                                    <div className="d-flex align-items-center gap-2 me-2">
                                      <img
                                        alt="seller avatar"
                                        className="custom-s-avatar"
                                        src="/avatars/logo-colored.svg"
                                      />
                                    </div>
                                    <div>
                                      <div className="fw-bold small">
                                        {order.seller || `${order.sellerWallet.slice(0, 8)}...${order.sellerWallet.slice(-8)}`}
                                      </div>
                                      <small className="text-muted">SNAKE
                                        {order.sellerWallet.slice(0, 8)}...{order.sellerWallet.slice(-8)}
                                      </small>
                                    </div>
                                  </div>
                                </td>
                                <td>
                                  <div className="fw-bold text-success">
                                    {parseFloat(order.amount).toLocaleString()}
                                  </div>
                                  <small className="text-muted"></small>
                                </td>
                                <td>
                                  <div className="fw-bold text-warning">
                                    {parseFloat(order.price).toFixed(6)}
                                  </div>
                                  <small className="text-muted">SOL per token</small>
                                </td>
                                <td>
                                  <div className="fw-bold text-info">
                                    {(parseFloat(order.amount) * parseFloat(order.price)).toFixed(4)}
                                  </div>
                                  <small className="text-muted">SOL total</small>
                                </td>
                                <td>
                                  {getBuyerRestrictionBadge(order)}
                                </td>
                                <td>
                                  <div className="small">
                                    {order.createdAt}
                                  </div>
                                  {order.expiresAt && (
                                    <small className="text-muted">
                                      Expires: {order.expiresAt}
                                    </small>
                                  )}
                                </td>
                                <td>
                                  {order.isExpired ? (
                                    <span className="badge bg-danger">⏰ Expired</span>
                                  ) : (
                                    <span className="badge bg-success">✅ Active</span>
                                  )}
                                </td>
                                <td>
                                  {activeTab === 'my' ? (
                                    <button
                                      onClick={() => cancelOrder(order.orderId)}
                                      disabled={loading || order.isExpired}
                                      className="btn btn-outline-danger btn-sm"
                                      title={order.isExpired ? "Cannot cancel expired order" : "Cancel this order"}
                                    >
                                      {loading ? (
                                        <span className="spinner-border spinner-border-sm" />
                                      ) : (
                                        <>
                                          <i className="bi bi-x-circle me-1"></i>
                                          Cancel
                                        </>
                                      )}
                                    </button>
                                  ) : order.sellerWallet === publicKey?.toString() ? (
                                    <span className="badge bg-secondary">Your Order</span>
                                  ) : order.isExpired ? (
                                    <span className="badge bg-warning">Expired</span>
                                  ) : canBuyOrder(order) ? (
                                    <button
                                      onClick={() => executeOrder(order)}
                                      disabled={loading}
                                      className="btn btn-outline-primary btn-sm"
                                      title="Buy this order"
                                    >
                                      {loading ? (
                                        <span className="spinner-border spinner-border-sm" />
                                      ) : (
                                        <>
                                          <i className="bi bi-cart-plus me-1"></i>
                                          Buy
                                        </>
                                      )}
                                    </button>
                                  ) : (
                                    <span
                                      className="badge bg-dark"
                                      title={
                                        order.buyerRestrictions.patronsOnly
                                          ? `Patrons only (you are: ${userRole})`
                                          : order.buyerRestrictions.treasuryOnly
                                            ? `Treasury only (you are: ${userRole})`
                                            : "Not eligible"
                                      }
                                    >
                                      Not Eligible
                                    </span>
                                  )}
                                </td>
                              </tr>
                            ))}
                          </tbody>
                        </table>
                      </div>
                    )}
                  </div>
                </div>

                {/* Help Section */}
                <div className="card mt-4">
                  <div className="card-body">
                    <h6 className="card-title">
                      <i className="bi bi-info-circle me-2"></i>
                      How OTC Trading Works
                    </h6>
                    <div className="row">
                      <div className="col-md-6">
                        <h6 className="text-primary">For Sellers:</h6>
                        <ul className="list-unstyled">
                          <li><i className="bi bi-check-circle text-success me-2"></i>Create sell orders with your desired price</li>
                          <li><i className="bi bi-check-circle text-success me-2"></i>Choose who can buy (Patrons, Treasury, or Open)</li>
                          <li><i className="bi bi-check-circle text-success me-2"></i>Orders expire automatically after 24 hours</li>
                          <li><i className="bi bi-check-circle text-success me-2"></i>Cancel anytime before expiration</li>
                        </ul>
                      </div>
                      <div className="col-md-6">
                        <h6 className="text-primary">For Buyers:</h6>
                        <ul className="list-unstyled">
                          <li><i className="bi bi-check-circle text-success me-2"></i>Browse active orders from all sellers</li>
                          <li><i className="bi bi-check-circle text-success me-2"></i>Check eligibility requirements</li>
                          <li><i className="bi bi-check-circle text-success me-2"></i>Execute orders with one click</li>
                          <li><i className="bi bi-check-circle text-success me-2"></i>Patrons receive rebates on purchases</li>
                        </ul>
                      </div>
                    </div>

                    <div className="alert alert-info mt-3">
                      <small>
                        <i className="bi bi-lightbulb me-2"></i>
                        <strong>Tip:</strong> All transactions require wallet approval and network fees.
                        Prices are in SOL per SNAKE token. Orders show total SOL value for convenience.
                      </small>
                    </div>
                  </div>
                </div>
              </div>
            </WalletGuard>
          </div>
        </div>
      </div>
    </div>
  );
};

export default OTCTrading;
