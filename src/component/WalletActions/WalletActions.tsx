import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { FaWallet } from "react-icons/fa";

interface WalletActionsProps {
  onBack: () => void;
}

const WalletActions: React.FC<WalletActionsProps> = ({ onBack }) => {
  // State for the forms/inputs that require user input
  const [walletName, setWalletName] = useState("");
  const [walletNameData, setWalletNameData] = useState<Record<string, any> | null>(null);
  const [fileName, setFileName] = useState("");
  const [loadWallet, setLoadWallet] = useState<Record<string, any> | null>(null);

  // Example handlers
  const handleCreateWallet = async () => {
    try {
      const res = await invoke<Record<string, any>>("create_wallet", { walletName });
      setWalletNameData(res);
    } catch (error) {
      console.error("Error creating wallet: ", error);
    }
  };

  const handleLoadWallet = async () => {
    try {
      const res = await invoke<Record<string, any>>("load_wallet", { filename: fileName });
      setLoadWallet(res);
    } catch (error) {
      console.error("Error loading wallet: ", error);
    }
  }

  return (
    <div className="w-full max-w-4xl mx-auto p-4 text-white">
      <button
        onClick={onBack}
        className="text-sm bg-gray-600 px-3 py-1 rounded-md hover:bg-gray-700 mb-4"
      >
        &larr; Back to Dashboard
      </button>

      <h2 className="text-2xl font-bold mb-4">Wallet Actions</h2>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">

        {/* Card: Create Wallet */}
        <div className="bg-gray-800 p-4 rounded-lg shadow-lg">
          <div className="flex items-center space-x-3 mb-4">
            <FaWallet size={30} />
            <h3 className="text-lg font-semibold">Create Wallet</h3>
          </div>
          <p className="text-sm text-gray-300 mb-2">Creates a new wallet</p>

          {/* Wallet Name Input */}
          <input
            type="text"
            className="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500 mb-4"
            placeholder="Enter wallet name"
            value={walletName}
            onChange={(e) => setWalletName(e.target.value)}
          />

          {/* Create Button */}
          <button
            onClick={handleCreateWallet}
            disabled={!walletName}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors disabled:opacity-50"
          >
            Create Wallet
          </button>

          {/* Display Wallet Created */}
          {walletNameData && (
            <p className="bg-gray-700 text-white px-3 py-2 rounded-md text-sm mt-3 break-words">
              {JSON.stringify(walletNameData, null, 2)}
            </p>
          )}
        </div>

        {/* Card: Load Wallet */}
        <div className="bg-gray-800 p-4 rounded-lg shadow-lg">
          <div className="flex items-center space-x-3 mb-4">
            <FaWallet size={30} />
            <h3 className="text-lg font-semibold">Load Wallet</h3>
          </div>
          <p className="text-sm text-gray-300 mb-2">Loads an existing wallet</p>

          {/* File Name Input */}
          <input
            type="text"
            className="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500 mb-4"
            placeholder="Enter wallet name"
            value={fileName}
            onChange={(e) => setFileName(e.target.value)}
          />

          {/* Create Button */}
          <button
            onClick={handleLoadWallet}
            disabled={!fileName}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors disabled:opacity-50"
          >
            Load Wallet
          </button>

          {/* Display Wallet Created */}
          {loadWallet && (
            <p className="bg-gray-700 text-white px-3 py-2 rounded-md text-sm mt-3 break-words">
              {JSON.stringify(loadWallet, null, 2)}
            </p>
          )}
        </div>
      </div>
    </div>
  );
};

export default WalletActions;
