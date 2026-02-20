import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { PublicKey } from "@solana/web3.js";
import { useState } from "react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";

export default function SwapInterface() {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [amountIn, setAmountIn] = useState("");
  const [amountOut, setAmountOut] = useState("");
  const [tokenA, setTokenA] = useState("");
  const [tokenB, setTokenB] = useState("");
  const [status, setStatus] = useState<string>("");
  const [loading, setLoading] = useState(false);

  const handleSwap = async () => {
    if (!publicKey) {
      setStatus("Please connect your wallet");
      return;
    }
    setStatus("Swap functionality requires deployed program. See README for setup.");
    setLoading(false);
  };

  if (!publicKey) {
    return (
      <div className="swap-card connect-prompt">
        <h2>Connect Wallet to Swap</h2>
        <p>Connect your Solana wallet to start swapping tokens</p>
        <WalletMultiButton className="wallet-button connect-btn" />
      </div>
    );
  }

  return (
    <div className="swap-card">
      <h2>Swap Tokens</h2>

      <div className="swap-input-group">
        <label>You Pay</label>
        <div className="input-wrapper">
          <input
            type="text"
            placeholder="0.0"
            value={amountIn}
            onChange={(e) => setAmountIn(e.target.value)}
          />
          <input
            type="text"
            placeholder="Token A mint address"
            value={tokenA}
            onChange={(e) => setTokenA(e.target.value)}
            className="mint-input"
          />
        </div>
      </div>

      <div className="swap-divider">
        <span>↓</span>
      </div>

      <div className="swap-input-group">
        <label>You Receive</label>
        <div className="input-wrapper">
          <input
            type="text"
            placeholder="0.0"
            value={amountOut}
            onChange={(e) => setAmountOut(e.target.value)}
            readOnly
          />
          <input
            type="text"
            placeholder="Token B mint address"
            value={tokenB}
            onChange={(e) => setTokenB(e.target.value)}
            className="mint-input"
          />
        </div>
      </div>

      <button
        className="swap-button"
        onClick={handleSwap}
        disabled={loading || !amountIn}
      >
        {loading ? "Swapping..." : "Swap"}
      </button>

      {status && (
        <div className="status-message">{status}</div>
      )}

      <div className="info-box">
        <h3>How to use</h3>
        <ol>
          <li>Build the program: <code>anchor build</code></li>
          <li>Run tests: <code>anchor test</code></li>
          <li>Deploy to devnet: <code>anchor deploy</code></li>
          <li>Initialize a pool with your token mints</li>
          <li>Add liquidity and swap!</li>
        </ol>
      </div>
    </div>
  );
}
