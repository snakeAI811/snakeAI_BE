import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { PublicKey, Connection } from '@solana/web3.js';
import { Program, AnchorProvider, web3, utils, BN } from '@project-serum/anchor';
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from '@solana/spl-token';

interface OTCOrder {
  orderId: string;
  seller: string;
  amount: string;
  price: string;
  isActive: boolean;
  createdAt: string;
  buyerRestrictions: {
    patronsOnly: boolean;
    treasuryOnly: boolean;
    minPatronScore: number;
  };
}

interface OTCTradingProps {
  program: Program<any>;
  connection: Connection;
}

const OTCTrading: React.FC<OTCTradingProps> = ({ program, connection }) => {
  const { publicKey, sendTransaction } = useWallet();
  const [orders, setOrders] = useState<OTCOrder[]>([]);
  const [loading, setLoading] = useState(false);
  const [createOrderForm, setCreateOrderForm] = useState({
    amount: '',
    price: '',
    patronsOnly: false,
    treasuryOnly: false,
    minPatronScore: 0,
  });
  const [showCreateForm, setShowCreateForm] = useState(false);

  useEffect(() => {
    if (publicKey) {
      fetchOrders();
    }
  }, [publicKey]);

  const fetchOrders = async () => {
    try {
      setLoading(true);
      // Fetch all OTC orders from the program
      const orderAccounts = await program.account.otcOrder.all();
      const activeOrders = orderAccounts
        .filter((order: any) => order.account.isActive)
        .map((order: any) => ({
          orderId: order.account.orderId.toString(),
          seller: order.account.seller.toString(),
          amount: order.account.amount.toString(),
          price: order.account.price.toString(),
          isActive: order.account.isActive,
          createdAt: new Date(order.account.createdAt * 1000).toLocaleString(),
          buyerRestrictions: order.account.buyerRestrictions,
        }));
      
      setOrders(activeOrders);
    } catch (error) {
      console.error('Error fetching orders:', error);
    } finally {
      setLoading(false);
    }
  };

  const createOrder = async () => {
    if (!publicKey) return;

    try {
      setLoading(true);
      const orderId = new BN(Date.now()); // Simple order ID generation
      
      const [userClaimPda] = await PublicKey.findProgramAddress(
        [Buffer.from('user_claim'), publicKey.toBuffer()],
        program.programId
      );

      const [otcOrderPda] = await PublicKey.findProgramAddress(
        [Buffer.from('otc_order'), publicKey.toBuffer(), orderId.toArrayLike(Buffer, 'le', 8)],
        program.programId
      );

      const userTokenAccount = await getAssociatedTokenAddress(
        new PublicKey(process.env.REACT_APP_TOKEN_MINT || ''),
        publicKey
      );

      const tx = await program.methods
        .createOtcOrder(
          orderId,
          new BN(parseFloat(createOrderForm.amount) * 1e9), // Convert to lamports
          new BN(parseFloat(createOrderForm.price) * 1e9),
          {
            patronsOnly: createOrderForm.patronsOnly,
            treasuryOnly: createOrderForm.treasuryOnly,
            minPatronScore: createOrderForm.minPatronScore,
          }
        )
        .accounts({
          seller: publicKey,
          sellerClaim: userClaimPda,
          sellerTokenAccount: userTokenAccount,
          otcOrder: otcOrderPda,
          systemProgram: web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .transaction();

      const signature = await sendTransaction(tx, connection);
      await connection.confirmTransaction(signature, 'processed');
      
      setShowCreateForm(false);
      setCreateOrderForm({
        amount: '',
        price: '',
        patronsOnly: false,
        treasuryOnly: false,
        minPatronScore: 0,
      });
      
      fetchOrders();
    } catch (error) {
      console.error('Error creating order:', error);
    } finally {
      setLoading(false);
    }
  };

  const executeOrder = async (order: OTCOrder) => {
    if (!publicKey) return;

    try {
      setLoading(true);
      
      const [buyerClaimPda] = await PublicKey.findProgramAddress(
        [Buffer.from('user_claim'), publicKey.toBuffer()],
        program.programId
      );

      const [otcOrderPda] = await PublicKey.findProgramAddress(
        [Buffer.from('otc_order'), new PublicKey(order.seller).toBuffer(), new BN(order.orderId).toArrayLike(Buffer, 'le', 8)],
        program.programId
      );

      const sellerTokenAccount = await getAssociatedTokenAddress(
        new PublicKey(process.env.REACT_APP_TOKEN_MINT || ''),
        new PublicKey(order.seller)
      );

      const buyerTokenAccount = await getAssociatedTokenAddress(
        new PublicKey(process.env.REACT_APP_TOKEN_MINT || ''),
        publicKey
      );

      const buyerPaymentAccount = await getAssociatedTokenAddress(
        new PublicKey(process.env.REACT_APP_PAYMENT_TOKEN_MINT || ''),
        publicKey
      );

      const sellerPaymentAccount = await getAssociatedTokenAddress(
        new PublicKey(process.env.REACT_APP_PAYMENT_TOKEN_MINT || ''),
        new PublicKey(order.seller)
      );

      const tx = await program.methods
        .executeOtcOrder()
        .accounts({
          buyer: publicKey,
          buyerClaim: buyerClaimPda,
          otcOrder: otcOrderPda,
          sellerTokenAccount,
          buyerTokenAccount,
          buyerPaymentAccount,
          sellerPaymentAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .transaction();

      const signature = await sendTransaction(tx, connection);
      await connection.confirmTransaction(signature, 'processed');
      
      fetchOrders();
    } catch (error) {
      console.error('Error executing order:', error);
    } finally {
      setLoading(false);
    }
  };

  const canBuyOrder = (order: OTCOrder) => {
    // Add logic to check if user meets buying restrictions
    // This would need to fetch user's role and patron score
    return true; // Simplified for now
  };

  return (
    <div className="max-w-6xl mx-auto p-6">
      <div className="bg-white rounded-lg shadow-lg">
        <div className="p-6 border-b border-gray-200">
          <div className="flex justify-between items-center">
            <h2 className="text-2xl font-bold text-gray-900">OTC Trading</h2>
            <button
              onClick={() => setShowCreateForm(true)}
              className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-md transition-colors"
              disabled={loading}
            >
              Create Order
            </button>
          </div>
          <p className="mt-2 text-gray-600">
            Fixed-price token trading between Sellers and Patrons/Treasury
          </p>
        </div>

        {showCreateForm && (
          <div className="p-6 bg-gray-50 border-b border-gray-200">
            <h3 className="text-lg font-semibold mb-4">Create Sell Order</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Amount (SNAKE)
                </label>
                <input
                  type="number"
                  value={createOrderForm.amount}
                  onChange={(e) => setCreateOrderForm({...createOrderForm, amount: e.target.value})}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="Enter amount to sell"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Price per Token
                </label>
                <input
                  type="number"
                  value={createOrderForm.price}
                  onChange={(e) => setCreateOrderForm({...createOrderForm, price: e.target.value})}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="Enter price per token"
                />
              </div>
            </div>
            
            <div className="mt-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Buyer Restrictions
              </label>
              <div className="space-y-2">
                <label className="flex items-center">
                  <input
                    type="checkbox"
                    checked={createOrderForm.patronsOnly}
                    onChange={(e) => setCreateOrderForm({...createOrderForm, patronsOnly: e.target.checked})}
                    className="mr-2"
                  />
                  <span className="text-sm text-gray-700">Patrons Only</span>
                </label>
                <label className="flex items-center">
                  <input
                    type="checkbox"
                    checked={createOrderForm.treasuryOnly}
                    onChange={(e) => setCreateOrderForm({...createOrderForm, treasuryOnly: e.target.checked})}
                    className="mr-2"
                  />
                  <span className="text-sm text-gray-700">Treasury Only</span>
                </label>
                <div className="flex items-center space-x-2">
                  <label className="text-sm text-gray-700">Min Patron Score:</label>
                  <input
                    type="number"
                    value={createOrderForm.minPatronScore}
                    onChange={(e) => setCreateOrderForm({...createOrderForm, minPatronScore: parseInt(e.target.value) || 0})}
                    className="w-20 px-2 py-1 border border-gray-300 rounded-md text-sm"
                    min="0"
                    max="100"
                  />
                </div>
              </div>
            </div>

            <div className="mt-6 flex space-x-3">
              <button
                onClick={createOrder}
                disabled={loading || !createOrderForm.amount || !createOrderForm.price}
                className="bg-green-600 hover:bg-green-700 disabled:bg-gray-300 text-white px-4 py-2 rounded-md transition-colors"
              >
                {loading ? 'Creating...' : 'Create Order'}
              </button>
              <button
                onClick={() => setShowCreateForm(false)}
                className="bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded-md transition-colors"
              >
                Cancel
              </button>
            </div>
          </div>
        )}

        <div className="p-6">
          <h3 className="text-lg font-semibold mb-4">Active Orders</h3>
          {loading ? (
            <div className="text-center py-8">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
              <p className="mt-2 text-gray-600">Loading orders...</p>
            </div>
          ) : orders.length === 0 ? (
            <div className="text-center py-8">
              <p className="text-gray-500">No active orders found</p>
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="min-w-full divide-y divide-gray-200">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Seller
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Amount
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Price
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Restrictions
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Created
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Actions
                    </th>
                  </tr>
                </thead>
                <tbody className="bg-white divide-y divide-gray-200">
                  {orders.map((order) => (
                    <tr key={order.orderId} className="hover:bg-gray-50">
                      <td className="px-6 py-4 whitespace-nowrap">
                        <div className="text-sm font-medium text-gray-900">
                          {order.seller.slice(0, 8)}...{order.seller.slice(-8)}
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <div className="text-sm text-gray-900">
                          {(parseInt(order.amount) / 1e9).toLocaleString()} SNAKE
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <div className="text-sm text-gray-900">
                          {(parseInt(order.price) / 1e9).toFixed(4)}
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <div className="text-sm text-gray-900">
                          {order.buyerRestrictions.patronsOnly && (
                            <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-purple-100 text-purple-800 mr-1">
                              Patrons Only
                            </span>
                          )}
                          {order.buyerRestrictions.treasuryOnly && (
                            <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 mr-1">
                              Treasury Only
                            </span>
                          )}
                          {order.buyerRestrictions.minPatronScore > 0 && (
                            <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
                              Min Score: {order.buyerRestrictions.minPatronScore}
                            </span>
                          )}
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        {order.createdAt}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
                        {order.seller === publicKey?.toString() ? (
                          <span className="text-gray-500">Your Order</span>
                        ) : canBuyOrder(order) ? (
                          <button
                            onClick={() => executeOrder(order)}
                            disabled={loading}
                            className="text-blue-600 hover:text-blue-900 disabled:text-gray-400"
                          >
                            Buy
                          </button>
                        ) : (
                          <span className="text-gray-400">Not Eligible</span>
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
    </div>
  );
};

export default OTCTrading;
