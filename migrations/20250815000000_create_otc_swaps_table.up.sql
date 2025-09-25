-- Create OTC swaps table for tracking swap transactions
CREATE TABLE IF NOT EXISTS otc_swaps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    seller_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    buyer_id UUID REFERENCES users(id) ON DELETE SET NULL,
    seller_wallet VARCHAR(255) NOT NULL,
    buyer_wallet VARCHAR(255),
    otc_swap_pda VARCHAR(255) NOT NULL UNIQUE,
    token_amount BIGINT NOT NULL,
    sol_rate BIGINT NOT NULL,
    buyer_rebate BIGINT DEFAULT 0,
    swap_type VARCHAR(50) NOT NULL DEFAULT 'exiter_to_patron', -- exiter_to_patron, patron_to_patron, exiter_to_treasury
    buyer_role_required VARCHAR(50) DEFAULT 'none', -- none, staker, patron
    status VARCHAR(50) NOT NULL DEFAULT 'active', -- active, completed, cancelled, expired
    initiate_tx_signature VARCHAR(255),
    accept_tx_signature VARCHAR(255),
    cancel_tx_signature VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    completed_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (now() + INTERVAL '24 hours')
);

-- Indexes for efficient querying
CREATE INDEX idx_otc_swaps_seller_id ON otc_swaps(seller_id);
CREATE INDEX idx_otc_swaps_buyer_id ON otc_swaps(buyer_id);
CREATE INDEX idx_otc_swaps_seller_wallet ON otc_swaps(seller_wallet);
CREATE INDEX idx_otc_swaps_buyer_wallet ON otc_swaps(buyer_wallet);
CREATE INDEX idx_otc_swaps_otc_swap_pda ON otc_swaps(otc_swap_pda);
CREATE INDEX idx_otc_swaps_status ON otc_swaps(status);
CREATE INDEX idx_otc_swaps_swap_type ON otc_swaps(swap_type);
CREATE INDEX idx_otc_swaps_created_at ON otc_swaps(created_at);
CREATE INDEX idx_otc_swaps_expires_at ON otc_swaps(expires_at);
CREATE INDEX idx_otc_swaps_active ON otc_swaps(status) WHERE status = 'active';