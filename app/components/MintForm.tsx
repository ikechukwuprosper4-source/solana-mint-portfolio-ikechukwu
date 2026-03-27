import React, { useState } from 'react';

const MintForm = () => {
    const [tokenAmount, setTokenAmount] = useState(0);

    const handleMint = async (e) => {
        e.preventDefault();
        // Add your minting logic here using SPL token standards
        console.log(`Minting ${tokenAmount} tokens...`);
    };

    return (
        <form onSubmit={handleMint}>
            <div>
                <label htmlFor="amount">Amount to Mint:</label>
                <input
                    type="number"
                    id="amount"
                    value={tokenAmount}
                    onChange={(e) => setTokenAmount(e.target.value)}
                    required
                />
            </div>
            <button type="submit">Mint Tokens</button>
        </form>
    );
};

export default MintForm;