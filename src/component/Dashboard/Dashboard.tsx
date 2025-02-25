import React from "react";
import { FaBitcoin, FaWallet, FaNetworkWired, FaCogs } from "react-icons/fa";

interface DashboardProps {
  onBlockchainInfoClick: () => void;
}

const Dashboard: React.FC<DashboardProps> = ({ onBlockchainInfoClick }) => {
  const sections = [
    {
      title: "Blockchain Information",
      icon: <FaBitcoin size={40} />,
      bg: "bg-yellow-500",
      onClick: onBlockchainInfoClick,
    },
    {
      title: "Wallet Actions",
      icon: <FaWallet size={40} />,
      bg: "bg-blue-500",
    },
    {
      title: "Network Management",
      icon: <FaNetworkWired size={40} />,
      bg: "bg-green-500",
    },
    {
      title: "Advanced",
      icon: <FaCogs size={40} />,
      bg: "bg-gray-700",
    },
  ];

  return (
    <div className="flex flex-col items-center min-h-screen bg-gray-900 text-white p-6">
      <h1 className="text-3xl font-bold mb-10 items-center text-center">Welcome to Bitcoin POC GUI</h1>
      <div className="grid grid-cols-2 gap-6 w-full max-w-4xl items-center text-center">
        {sections.map((section, index) => (
          <div
            key={index}
            onClick={section.onClick}
            className={`flex flex-col min-w-[9rem] min-h-[9rem] items-center justify-center p-2 md:p-6 rounded-lg shadow-lg ${section.bg} hover:rotate-6 transition-transform cursor-pointer`}
          >
            {section.icon}
            <h2 className="text-lg font-semibold mt-4">{section.title}</h2>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Dashboard;
