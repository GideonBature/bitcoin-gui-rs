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
  const [bestBlockHash, setBestBlockHash] = useState<string | null>(null);
  const [blockHeight2, setBlockHeight2] = useState("");
  const [address, setAddress] = useState("");
  const [blockCount, setBlockCount] = useState<number | null>(null);
  const [blockHash, setBlockHash] = useState<string | null>(null);
  const [blockData, setBlockData] = useState<any | null>(null);
  const [mempoolData, setMempoolData] = useState<string[] | null>(null);
  const [uptime, setUptime] = useState<number | null>(null);

  const [numBlocks, setNumBlocks] = useState("1"); // State for number of blocks
  const [generatedHashes, setGeneratedHashes] = useState<string[]>([]);
  const [error, setError] = useState('');

  // Utility function
  const formatUptime = (seconds: any) => {
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    return `${days}d ${hours}h ${minutes}m ${secs}s`;
  };
  

  // Example handlers
  const handleGetBlockCount = async () => {
    // console.log("Get block count");
    try {
      const count: number = await invoke("get_chain_tip_height");
      setBlockCount(count);
    } catch (error) {
      console.error("Error fetching block count: ", error);
    }
  };

  const handleGetBestBlockHash = async () => {
    // console.log("Get best block hash");
    try {
      const hash: string = await invoke("get_chain_tip_hash");
      setBestBlockHash(hash);
    } catch (error) {
      console.error("Error fetching best block hash: ", error)
    }
  };

  const handleGetBlockHash = async () => {
    // console.log("Get block hash for height:", blockHeight);
    try {
      const hash: string = await invoke("get_chain_hash_by_height", { height: parseInt(blockHeight) });
      setBlockHash(hash);
    } catch (error) {
      console.error("Error fetching block hash: ", error);
    }
  };

  const handleGetBlock = async () => {
    // console.log("Get block for height:", blockHeight);
    try {
      const blockString: string = await invoke("get_block_by_height", { height: parseInt(blockHeight2) });

      const blockObject = JSON.parse(blockString);

      setBlockData(blockObject);
      
    } catch (error) {
      console.error("Error fetching block: ", error);
    }
  };

  const handleGetRawMemPool = async () => {
    // console.log("Get raw mempool");
    try {
      const mempool: string[] = await invoke("get_raw_mempool");
      setMempoolData(mempool);
    } catch (error) {
      console.error("Error fetching raw mempool: ", error);
    }
  };

  const handleGenerateToAddress = async () => {
    // console.log("Generate to address:", address);
    setError("");

    if (!address) {
      return setError("Please enter a valid Bitcoin address.");
    }
    // if (!numBlocks || isNaN(Number(numBlocks)) || Number(numBlocks) <= 0) {
    //   return setError("Please enter a valid number of blocks.");
    // }
    try {
      const hashes: string[] = await invoke("generate_to_address", {
        address,
        num_blocks: parseInt(numBlocks, 10),
      });
      
      setGeneratedHashes(hashes);
    } catch (err) {
      console.error("Error generating blocks: ", err);
      setError(`Failed to generate blocks: ${err}`);
    }
  };

  const handleGetUptime = async () => {
    // console.log("Get uptime");
    try {
      const uptime: number = await invoke("get_uptime");
      setUptime(uptime);
    } catch (error) {
      console.log("Error fetching uptime: ", error);
    }
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
          <div className="flex items-center space-x-3">
            <button
              onClick={handleGetBlockCount}
              className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors"
            >
              Get Block Count
            </button>
            {blockCount !== null && (
              <span className="bg-gray-700 text-white px-3 py-2 rounded-md text-sm">
                {blockCount}
              </span>
            )}
          </div>
        </div>


        {/* Card: Get Best Block Hash */}
        <div className="bg-gray-800 p-4 rounded-lg shadow-lg">
          <div className="flex items-center space-x-3 mb-4">
            <FaLink size={30} />
            <h3 className="text-lg font-semibold">Get Best Block Hash</h3>
          </div>
          <p className="text-sm text-gray-300 mb-4">
            Retrieves the hash of the latest block in the blockchain.
          </p>
          <button
            onClick={handleGetBestBlockHash}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors"
          >
            Get Best Block Hash
          </button>
          {bestBlockHash && (
            <p className="bg-gray-700 text-white px-3 py-2 rounded-md text-sm mt-3 break-words">
              {bestBlockHash}
            </p>
          )}
        </div>



        {/* Card: Get Block Hash */}
        <div className="bg-gray-800 p-4 rounded-lg shadow-lg">
          <div className="flex items-center space-x-3 mb-4">
            <FaLink size={30} />
            <h3 className="text-lg font-semibold">Get Block Hash (by Height)</h3>
          </div>
          <p className="text-sm text-gray-300 mb-2">
            Retrieve the block hash for a given block height.
          </p>
          
          {/* Input Field */}
          <input
            type="number"
            className="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500 mb-4"
            placeholder="Enter block height"
            value={blockHeight}
            onChange={(e) => setBlockHeight(e.target.value)}
          />
          
          {/* Button */}
          <button
            onClick={handleGetBlockHash}
            disabled={!blockHeight}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors disabled:opacity-50"
          >
            Get Block Hash
          </button>

          {/* Display Block Hash */}
          {blockHash && (
            <p className="bg-gray-700 text-white px-3 py-2 rounded-md text-sm mt-3 break-words">
              {blockHash}
            </p>
          )}
        </div>


        {/* Card: Get Block (by height) */}
        <div className="bg-gray-800 p-4 rounded-lg shadow-lg">
          <div className="flex items-center space-x-3 mb-4">
            <FaCube size={30} />
            <h3 className="text-lg font-semibold">Get Block (by Height)</h3>
          </div>
          
          <p className="text-sm text-gray-300 mb-2">
            Retrieve the block information for a given block height.
          </p>

          {/* Input Field */}
          <input
            type="number"
            className="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500 mb-4"
            placeholder="Enter block height"
            value={blockHeight2}
            onChange={(e) => setBlockHeight2(e.target.value)}
          />

          {/* Button */}
          <button
            onClick={handleGetBlock}
            disabled={!blockHeight2}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors disabled:opacity-50"
          >
            Get Block
          </button>

          {/* Display Block Data */}
          {blockData && (
            <pre className="bg-gray-700 text-white px-3 py-2 rounded-md text-sm mt-3 overflow-auto max-h-60">
              {JSON.stringify(blockData, null, 2)}
            </pre>
          )}
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

          {/* Button */}
          <button
            onClick={handleGetRawMemPool}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors"
          >
            Get Raw MemPool
          </button>

          {/* Display Mempool Transactions */}
          {mempoolData && mempoolData.length > 0 && (
            <div className="bg-gray-700 text-white px-3 py-2 rounded-md text-sm mt-3 max-h-60 overflow-auto">
              {mempoolData.map((txid, index) => (
                <div key={index} className="truncate">{txid}</div>
              ))}
            </div>
          )}

          {/* No transactions message */}
          {mempoolData && mempoolData.length === 0 && (
            <p className="text-sm text-gray-400 mt-3">No transactions in the mempool.</p>
          )}
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

          {/* Bitcoin Address Input */}
          <input
            type="text"
            className="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500 mb-4"
            placeholder="Enter Bitcoin address"
            value={address}
            onChange={(e) => setAddress(e.target.value)}
          />

          {/* Number of Blocks Input */}
          <input
            type="number"
            className="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-yellow-500 mb-4"
            placeholder="Enter number of blocks"
            value={numBlocks}
            onChange={(e) => setNumBlocks(e.target.value)}
            min="1" // Optional: Ensure the number of blocks is at least 1
          />

          {/* Generate Button */}
          <button
            onClick={handleGenerateToAddress}
            disabled={!address || !numBlocks}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors disabled:opacity-50"
          >
            Generate
          </button>

          {/* Result Display */}
          {generatedHashes.length > 0 && (
            <div className="mt-4 p-3 bg-gray-700 rounded-md text-green-400">
              <h4 className="font-semibold">Generated Block Hashes:</h4>
              <ul>
                {generatedHashes.map((hash, index) => (
                  <li key={index} className="text-sm">{hash}</li>
                ))}
              </ul>
            </div>
          )}

          {/* Error Message */}
          {error && (
            <div className="mt-4 p-3 bg-red-700 rounded-md text-white">
              <h4 className="font-semibold">Error:</h4>
              <p className="text-sm">{error}</p>
            </div>
          )}
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

          {/* Button */}
          <button
            onClick={handleGetUptime}
            className="bg-yellow-500 hover:bg-yellow-600 text-gray-900 px-4 py-2 rounded-md transition-colors"
          >
            Get Uptime
          </button>

          {/* Display Uptime */}
          {uptime !== null && (
            <p className="mt-3 text-lg font-semibold text-yellow-400">
              ⏳ {formatUptime(uptime)}
            </p>
          )}
        </div>

      </div>
    </div>
  );
};

export default BlockchainInfo;
