import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  FaCube,
  FaLink,
  FaExchangeAlt,
  FaBolt,
  FaClock
} from "react-icons/fa";

interface BlockchainInfoProps {
  onBack: () => void;
}

const BlockchainInfo: React.FC<BlockchainInfoProps> = ({ onBack }) => {
  // State for the forms/inputs that require user input
  const [blockHeight, setBlockHeight] = useState("");
  const [address, setAddress] = useState("");
  const [blockCount, setBlockCount] = useState<number | null>(null);
  const [bestBlockHash, setBestBlockHash] = useState<string | null>(null);

  // Example handlers
  const handleGetBlockCount = async () => {
    // console.log("Get block count");
    try {
      const count: number = await invoke("get_block_count");
      setBlockCount(count);
    } catch (error) {
      console.error("Error fetching block count: ", error);
    }
  };

  const handleGetBestBlockHash = async () => {
    // console.log("Get best block hash");
    try {
      const hash: string = await invoke("get_best_block_hash");
      setBestBlockHash(hash);
    } catch (error) {
      console.error("Error fetching best block hash: ", error)
    }
  };

  const handleGetBlockHash = () => {
    console.log("Get block hash for height:", blockHeight);
  };

  const handleGetBlock = () => {
    console.log("Get block for height:", blockHeight);
  };

  const handleGetRawMemPool = () => {
    console.log("Get raw mempool");
  };

  const handleGenerateToAddress = () => {
    console.log("Generate to address:", address);
  };

  const handleGetUptime = () => {
    console.log("Get uptime");
  };

  return (
    <div className="w-full max-w-4xl mx-auto p-4 text-white">
      <button
        onClick={onBack}
        className="text-sm bg-gray-600 px-3 py-1 rounded-md hover:bg-gray-700 mb-4"
      >
        &larr; Back to Dashboard
      </button>

      <h2 className="text-2xl font-bold mb-4">Blockchain Information</h2>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Card: Get Block Count */}
        <div className="bg-gray-800 p-4 rounded-lg shadow-lg">
          <div className="flex items-center space-x-3 mb-4">
            <FaCube size={30} />
            <h3 className="text-lg font-semibold">Get Block Count</h3>
          </div>
          <p className="text-sm text-gray-300 mb-4">
            Retrieves the total number of blocks in the longest blockchain.
          </p>
          <button
            onClick={handleGetBlockCount}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors"
          >
            Get Block Count
          </button>
          <p>{blockCount}</p>
        </div>

        {/* Card: Get Best Block Hash */}
        <div className="bg-gray-800 p-4 rounded-lg shadow-lg">
          <div className="flex items-center space-x-3 mb-4">
            <FaLink size={30} />
            <h3 className="text-lg font-semibold">Get Best Block Hash</h3>
          </div>
          <p className="text-sm text-gray-300 mb-4">
            Retrieves the hash of the latest block in the chain.
          </p>
          <button
            onClick={handleGetBestBlockHash}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors"
          >
            Get Best Block Hash
          </button>
        </div>

        {/* Card: Get Block Hash */}
        <div className="bg-gray-800 p-4 rounded-lg shadow-lg">
          <div className="flex items-center space-x-3 mb-4">
            <FaLink size={30} />
            <h3 className="text-lg font-semibold">Get Block Hash</h3>
          </div>
          <p className="text-sm text-gray-300 mb-2">
            Retrieve the block hash for a given block height.
          </p>
          <input
            type="number"
            className="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500 mb-4"
            placeholder="Enter block height"
            value={blockHeight}
            onChange={(e) => setBlockHeight(e.target.value)}
          />
          <button
            onClick={handleGetBlockHash}
            disabled={!blockHeight}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors disabled:opacity-50"
          >
            Get Block Hash
          </button>
        </div>

        {/* Card: Get Block (by height) */}
        <div className="bg-gray-800 p-4 rounded-lg shadow-lg">
          <div className="flex items-center space-x-3 mb-4">
            <FaCube size={30} />
            <h3 className="text-lg font-semibold">Get Block (by Height)</h3>
          </div>
          <input
            type="number"
            className="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500 mb-4"
            placeholder="Enter block height"
            value={blockHeight}
            onChange={(e) => setBlockHeight(e.target.value)}
          />
          <button
            onClick={handleGetBlock}
            disabled={!blockHeight}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors disabled:opacity-50"
          >
            Get Block
          </button>
        </div>

        {/* Card: Get Raw MemPool */}
        <div className="bg-gray-800 p-4 rounded-lg shadow-lg">
          <div className="flex items-center space-x-3 mb-4">
            <FaExchangeAlt size={30} />
            <h3 className="text-lg font-semibold">Get Raw MemPool</h3>
          </div>
          <p className="text-sm text-gray-300 mb-4">
            Retrieves all transaction IDs in the mempool.
          </p>
          <button
            onClick={handleGetRawMemPool}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors"
          >
            Get Raw MemPool
          </button>
        </div>

        {/* Card: Generate to Address */}
        <div className="bg-gray-800 p-4 rounded-lg shadow-lg">
          <div className="flex items-center space-x-3 mb-4">
            <FaBolt size={30} />
            <h3 className="text-lg font-semibold">Generate to Address</h3>
          </div>
          <p className="text-sm text-gray-300 mb-2">
            Generates a certain number of blocks to a specified address.
          </p>
          <input
            type="text"
            className="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500 mb-4"
            placeholder="Enter Bitcoin address"
            value={address}
            onChange={(e) => setAddress(e.target.value)}
          />
          <button
            onClick={handleGenerateToAddress}
            disabled={!address}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors disabled:opacity-50"
          >
            Generate
          </button>
        </div>

        {/* Card: Uptime */}
        <div className="bg-gray-800 p-4 rounded-lg shadow-lg">
          <div className="flex items-center space-x-3 mb-4">
            <FaClock size={30} />
            <h3 className="text-lg font-semibold">Uptime</h3>
          </div>
          <p className="text-sm text-gray-300 mb-4">
            Retrieves the total uptime of the Bitcoin node.
          </p>
          <button
            onClick={handleGetUptime}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors"
          >
            Get Uptime
          </button>
        </div>
      </div>
    </div>
  );
};

export default BlockchainInfo;
