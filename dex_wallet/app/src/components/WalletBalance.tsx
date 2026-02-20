import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { useEffect, useState } from "react";

export default function WalletBalance() {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [balance, setBalance] = useState<number | null>(null);

  useEffect(() => {
    if (!publicKey) {
      setBalance(null);
      return;
    }

    connection.getBalance(publicKey).then((lamports) => {
      setBalance(lamports / LAMPORTS_PER_SOL);
    });
  }, [connection, publicKey]);

  return (
    <div className="wallet-section">
      {publicKey && balance !== null && (
        <div className="balance-display">
          <span className="balance-label">Balance:</span>
          <span className="balance-value">{balance.toFixed(4)} SOL</span>
        </div>
      )}
      <WalletMultiButton className="wallet-button" />
    </div>
  );
}
