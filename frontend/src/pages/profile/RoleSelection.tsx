import React, { useState, useEffect } from 'react';
import { roleApi, tokenApi, userApi } from '../patron/services/apiService';
import { useToast } from '../../contexts/ToastContext';
import { useAppContext } from '../../contexts/AppContext';
import { useWalletContext } from '../../contexts/WalletContext';
import {
    Connection,
    PublicKey,
    Transaction,
    clusterApiUrl
} from '@solana/web3.js';
import { useErrorHandler } from '../../hooks/useErrorHandler';
import { SOLANA_RPC_URL, TOKEN_MINT } from '../../config/program';
import { ReactComponent as IconRedDot } from '../../svgs/role/red_dot.svg';
import { ReactComponent as IconDanger } from '../../svgs/role/danger.svg';
import { ReactComponent as IconExiter } from '../../svgs/role/exiter.svg';
import { ReactComponent as IconPatron } from '../../svgs/role/patron.svg';
import { ReactComponent as IconStaker } from '../../svgs/role/staker.svg';
import { ReactComponent as IconWarning } from '../../svgs/role/warning.svg';
import { ReactComponent as IconChecker } from '../../svgs/role/checker.svg';

export interface UserRole {
    role: 'none' | 'staker' | 'patron';
    status?: string;
    locked_until?: number;
    stake_amount?: number;
}

interface RoleSelectionProps {
    userRole: UserRole;
    onRoleChange: (role: UserRole) => void;
    tokenBalance: TokenBalance;
    userStats: UserStats;
}

interface TokenBalance {
    balance: number;
    locked: number;
    staked: number;
    rewards: number;
}

interface UserStats {
    total_mined_phase1: number;
    wallet_age_days: number;
    community_score: number;
    patron_qualification_score: number;
}

function RoleSelection({ userRole, onRoleChange, tokenBalance, userStats }: RoleSelectionProps) {
    const { showSuccess, showError, showInfo, showWarning } = useToast();
    const { updateUserRole } = useAppContext();
    const { connected, publicKey } = useWalletContext();
    const { handleError, isUserRejection } = useErrorHandler();
    const [selectedRole, setSelectedRole] = useState<'none' | 'staker' | 'patron'>(userRole.role);
    const [loading, setLoading] = useState(false);

    // Connection to Solana network
    const connection = new Connection(SOLANA_RPC_URL || 'https://api.devnet.solana.com', 'confirmed');

    // Constants from smart contract
    const MINIMUM_STAKER_TOKENS = 5000; // 500 SNAKE tokens (STAKE_AMOUNT)
    const MINIMUM_PATRON_QUALIFICATION_SCORE = 50; // Minimum score needed
    const MINIMUM_PATRON_MINED_TOKENS = 250000; // 100 SNAKE tokens minimum mined in Phase 1

    // Calculate patron qualification score (from smart contract logic)
    const calculatePatronScore = () => {
        let score = 0;

        // Mining contribution (40 points max)
        if (userStats.total_mined_phase1 >= 1000) score += 40;
        else if (userStats.total_mined_phase1 >= 500) score += 30;
        else if (userStats.total_mined_phase1 >= 100) score += 20;
        else if (userStats.total_mined_phase1 > 0) score += 10;

        // Wallet age (30 points max)
        if (userStats.wallet_age_days >= 90) score += 30;
        else if (userStats.wallet_age_days >= 30) score += 20;
        else if (userStats.wallet_age_days >= 7) score += 10;

        // Community contribution (30 points max)
        score += Math.min(userStats.community_score, 30);

        return score;
    };

    // Check if user can become staker
    const canBecomeStaker = () => {
        const totalTokens = tokenBalance.balance + tokenBalance.locked + tokenBalance.staked;
        return totalTokens >= MINIMUM_STAKER_TOKENS;
    };

    // Check if user can become patron
    const canBecomePatron = () => {
        const patronScore = calculatePatronScore();
        return patronScore >= MINIMUM_PATRON_QUALIFICATION_SCORE &&
            userStats.total_mined_phase1 >= MINIMUM_PATRON_MINED_TOKENS;
    };

    // Get role requirements and validation messages
    const getRoleRequirements = (role: 'none' | 'staker' | 'patron') => {
        switch (role) {
            case 'staker':
                const totalTokens = tokenBalance.balance + tokenBalance.locked + tokenBalance.staked;
                const stakerValid = canBecomeStaker();
                return {
                    valid: stakerValid,
                    message: stakerValid
                        ? (
                            <span>
                                <IconChecker /> You have {totalTokens.toLocaleString()} SNAKE tokens (minimum: {MINIMUM_STAKER_TOKENS})
                            </span>
                        )
                        : (
                            <span>
                                <IconDanger /> Need ${MINIMUM_STAKER_TOKENS} SNAKE tokens (you have: ${totalTokens.toLocaleString()})
                            </span>
                        )
                };

            case 'patron':
                const patronScore = calculatePatronScore();
                const patronValid = canBecomePatron();
                return {
                    valid: patronValid,
                    message: patronValid
                        ? (
                            <span>
                                <IconChecker /> Qualification score: {patronScore} (minimum: {MINIMUM_PATRON_QUALIFICATION_SCORE})
                            </span>
                        )
                        : (
                            <span>
                                <IconDanger /> Qualification score: {patronScore} (need: {MINIMUM_PATRON_QUALIFICATION_SCORE}). Improve mining history and community engagement.
                            </span>
                        )
                };

            default:
                return {
                    valid: true,
                    message: (
                        <span>
                            <IconChecker /> No requirements
                        </span>
                    )
                };
        }
    };

    const roleDescriptions = {
        none: {
            title: 'No Role',
            description: 'Basic access to mining and standard features',
            benefits: ['Basic tweet mining', 'Standard rewards', 'Community access'],
            icon: <IconExiter />,
            color: 'secondary'
        },
        staker: {
            title: 'staker',
            description: 'Lock tokens for enhanced rewards and yield generation',
            benefits: ['5% APY staking rewards', 'Enhanced mining multiplier', 'Priority support', '3-month lock period'],
            icon: <IconStaker />,
            color: 'primary'
        },
        patron: {
            title: 'patron',
            description: 'Premium tier with exclusive features and governance rights',
            benefits: ['All Staker benefits', 'DAO governance rights', 'OTC trading rebates', 'Exclusive features', '6-month lock period'],
            icon: <IconPatron />,
            color: 'warning'
        }
    };

    // Set up connection globally (you can also use useMemo/useEffect if needed)

    const handleRoleSelect = async () => {
        if (selectedRole === userRole.role) return;

        if (!connected || !publicKey) {
            showError('Please connect your wallet first');
            return;
        }

        const requirements = getRoleRequirements(selectedRole);
        if (!requirements.valid) {
            showError(`Cannot select ${selectedRole} role: ${requirements.message}`);
            setSelectedRole(userRole.role);
            return;
        }

        if (selectedRole === 'staker') {
            showWarning('Staker role requires locking tokens for 3 months.');
        } else if (selectedRole === 'patron') {
            showWarning('Patron role requires 6-month commitment and 20% burn penalty if you exit early.');
        }

        setLoading(true);
        showInfo(`Processing role change to ${selectedRole}...`);

        try {
            // 1. Get transaction from backend (as base64)
            const result = await roleApi.selectRole(selectedRole);

            if (!result.success || !result.data) {
                throw new Error(result.error || 'Failed to get role selection transaction');
            }

            // 2. Decode base64 transaction
            const txBytes = Uint8Array.from(atob(result.data), c => c.charCodeAt(0));
            const transaction = Transaction.from(txBytes);

            // 3. Fetch latest blockhash
            const latest = await connection.getLatestBlockhash('confirmed');
            transaction.recentBlockhash = latest.blockhash;
            transaction.feePayer = new PublicKey(publicKey);

            // 4. Sign with Phantom
            if (!(window as any).solana?.signTransaction) {
                throw new Error('Phantom wallet not found or not connected');
            }

            const signedTx = await (window as any).solana.signTransaction(transaction);

            // 5. Send and confirm
            showInfo('Submitting transaction...');
            const sig = await connection.sendRawTransaction(signedTx.serialize(), {
                skipPreflight: false,
                preflightCommitment: 'confirmed'
            });

            showInfo('Confirming transaction...');
            await connection.confirmTransaction(
                {
                    signature: sig,
                    blockhash: latest.blockhash,
                    lastValidBlockHeight: latest.lastValidBlockHeight
                },
                'confirmed'
            );

            console.log('✅ Role selection confirmed:', sig);

            // 6. Persist to backend
            await saveRoleToDatabase(selectedRole, sig);
            const newRole = { role: selectedRole };
            onRoleChange(newRole);
            updateUserRole(newRole);
            showSuccess(`Successfully changed role to ${selectedRole}!`);
        } catch (error) {
            if (!isUserRejection(error)) {
                handleError(error, 'Failed to select role');
            }
            console.error('Error selecting role:', error);
            setSelectedRole(userRole.role);
        } finally {
            setLoading(false);
        }
    };

    // Save role selection to database after successful transaction
    const saveRoleToDatabase = async (role: string, transactionSignature: string) => {
        try {
            showInfo('Saving role information to database...');

            // Call backend API to save role information
            const saveResult = await userApi.saveRoleSelection({
                role,
                transaction_signature: transactionSignature,
                timestamp: new Date().toISOString()
            });

            if (saveResult.success) {
                console.log('✅ Role saved to database successfully');
            } else {
                console.warn('⚠️ Failed to save role to database:', saveResult.error);
                // Don't throw error here as the blockchain transaction was successful
                showWarning('Role selected successfully but failed to save to database. This may cause display issues.');
            }
        } catch (error) {
            console.error('❌ Error saving role to database:', error);
            // Don't throw error here as the blockchain transaction was successful
            showWarning('Role selected successfully but failed to save to database. This may cause display issues.');
        }
    };

    return (
        <div className="w-100">
            <div className='sn-bold-border'>
                <div className="my-4 text-center fs-3 fs-lg-4 fs-xl-6 fs-xxl-6" > Choose Your Role</div>

                <div className="row g-4">
                    {Object.entries(roleDescriptions).map(([role, info]) => {
                        const requirements = getRoleRequirements(role as 'none' | 'staker' | 'patron');
                        const roleKey = role as 'none' | 'staker' | 'patron';

                        return (
                            <div key={role} className="col-lg-4">
                                <div
                                    className={`card h-100 border-3 ${selectedRole === role ? 'border-dark bg-light' :
                                        !requirements.valid ? 'border-danger' : 'border-secondary'
                                        }`}
                                    style={{
                                        cursor: requirements.valid ? 'pointer' : 'not-allowed',
                                        opacity: requirements.valid ? 1 : 0.9
                                    }}
                                    onClick={() => requirements.valid && setSelectedRole(roleKey)}
                                >
                                    <div className="card-body text-center">
                                        <div className="fs-1 mb-3">{info.icon}</div>
                                        <h3 className="card-title">{info.title}</h3>
                                        <p className="card-text text-muted">{info.description}</p>

                                        <div className="mt-3 mx-auto" style={{ width: '80%' }}>
                                            <h5 className='sn-y-border py-2 text-center'>Benefits:</h5>
                                            <ul className="list-unstyled">
                                                {info.benefits.map((benefit, index) => (
                                                    <li key={index} className="mb-1 text-start">
                                                        <small>✓ {benefit}</small>
                                                    </li>
                                                ))}
                                            </ul>
                                        </div>

                                        {/* Requirements Status */}
                                        <div className="my-2">
                                            <div className={`${requirements.valid ? 'sn-success' : 'sn-warning'} py-1 px-2 rounded d-flex align-items-center justify-content-center`}>
                                                <small>{requirements.message}</small>
                                            </div>
                                        </div>

                                        {selectedRole === role && (
                                            <div className="sn-bg-success">
                                                <span className="">Selected</span>
                                            </div>
                                        )}

                                        {/* Additional Info for Roles */}
                                        {/* {roleKey === 'staker' && (
                                        <div className="mt-2">
                                            <small className="text-muted">
                                                Requires token lock for 3 months
                                            </small>
                                        </div>
                                    )}

                                    {roleKey === 'patron' && (
                                        <div className="mt-2">
                                            <small className="text-muted">
                                                Score: {calculatePatronScore()}/100 | 6-month commitment
                                            </small>
                                        </div>
                                    )} */}

                                        {userRole.role === role && (
                                            <div className="mt-2">
                                                <span className="">[Current Role]</span>
                                            </div>
                                        )}
                                    </div>
                                </div>
                            </div>
                        );
                    })}
                </div>
            </div>

            {/* Role Selection Actions */}
            <div className="mt-4 text-center sn-select-role-actions">
                {/* {selectedRole !== userRole.role && (
                    <div className="alert alert-warning">
                        <strong>Warning:</strong> Changing roles may require token locking and have associated costs.
                        {selectedRole !== 'none' && (
                            <div className="mt-2">
                                <small>This role requires a commitment period where tokens will be locked.</small>
                            </div>
                        )}
                    </div>
                )} */}

                <button
                    className={`primary-btn btn-lg fs-5`}
                    onClick={handleRoleSelect}
                    disabled={loading || selectedRole === userRole.role}
                >
                    {loading ? (
                        <>
                            <span className="spinner-border spinner-border-sm me-2" />
                            Processing...
                        </>
                    ) : selectedRole === userRole.role ? (
                        'Current Role'
                    ) : (
                        `Select ${selectedRole} Role`
                    )}
                </button>
            </div>

            {/* Important Information */}
            <div className="mt-4">
                <div className="sn-bold-border">
                    {/* <h6 className="card-title text-center">📋 Role Requirements & Information</h6> */}
                    <div className="gap-2 d-flex sn-require-section">
                        <div className="sn-border sn-rq-staker p-3">
                            <h5 className='text-black'><IconStaker width={25} /> Staker Requirements:</h5>
                            <ul className='list-unstyled'>
                                <li><IconRedDot /> Minimum {MINIMUM_STAKER_TOKENS} SNAKE tokens</li>
                                <li><IconRedDot /> 3-month lock commitment</li>
                                <li><IconRedDot /> Earn 5% APY on locked tokens</li>
                                <li><IconRedDot /> Enhanced mining rewards</li>
                            </ul>
                        </div>
                        <div className="sn-border sn-rq-patron p-3">
                            <h5 className='text-black'><IconPatron width={25} /> Patron Requirements:</h5>
                            <ul className='list-unstyled'>
                                <li><IconRedDot /> Qualification score ≥ {MINIMUM_PATRON_QUALIFICATION_SCORE}</li>
                                <li><IconRedDot /> Minimum {MINIMUM_PATRON_MINED_TOKENS} tokens mined in Phase 1</li>
                                <li><IconRedDot /> 6-month commitment period</li>
                                <li><IconRedDot /> Community engagement history</li>
                                <li><IconRedDot /> Higher roles unlock additional features and better rewards</li>
                            </ul>
                        </div>
                    </div>

                    <div className="sn-danger p-1 d-flex justify-content-center rounded mt-3">
                        <p className='mb-0'><strong><IconWarning height={28} /> Important:</strong> Patron role has a 20% burn penalty for early exit. Staker tokens are locked for 3 months.</p>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default RoleSelection;
