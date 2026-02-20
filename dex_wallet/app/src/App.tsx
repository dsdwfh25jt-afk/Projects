import { useMemo } from "react";
import {
  ConnectionProvider,
  WalletProvider,
} from "@solana/wallet-adapter-react";
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";
import { PhantomWalletAdapter } from "@solana/wallet-adapter-wallets";
import { clusterApiUrl } from "@solana/web3.js";
import "@solana/wallet-adapter-react-ui/styles.css";

import SwapInterface from "./components/SwapInterface";
import WalletBalance from "./components/WalletBalance";
import "./App.css";

function App() {
  const endpoint = useMemo(
    () => process.env.VITE_SOLANA_RPC ?? clusterApiUrl("devnet"),
    []
  );

  const wallets = useMemo(() => [new PhantomWalletAdapter()], []);

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <div className="app">
            <header className="header">
              <div className="logo">
                <span className="logo-icon">◆</span>
                <h1>DEX Wallet</h1>
              </div>
              <p className="tagline">Decentralized Token Exchange on Solana</p>
              <WalletBalance />
            </header>

            <main className="main">
              <SwapInterface />
            </main>

            <footer className="footer">
              <p>Built with Anchor Framework • Solana Blockchain</p>
            </footer>
          </div>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}

export default App;
