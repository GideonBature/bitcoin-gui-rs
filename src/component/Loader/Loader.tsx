import { FaBitcoin } from 'react-icons/fa';

const Loader = () => {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white">
      <FaBitcoin className="animate-spin text-yellow-500" size={80} />
      <p className="mt-4 text-lg font-semibold">Verifying the blockchain...</p>
    </div>
  );
};

export default Loader;
