// import { useState } from "react";

// import blockchainLogo from "./assets/blockchain.svg";
// import walletLogo from "./assets/wallet.svg";
// import networkLogo from "./assets/network.svg";
// import othersLogo from "./assets/others.svg";

// import bitcoinLogo from "./assets/bitcoin.svg";
// import reactLogo from "./assets/react.svg";
// import { invoke } from "@tauri-apps/api/core";
// import "./App.css";

// function App() {
//   const [greetMsg, setGreetMsg] = useState("");
//   const [name, setName] = useState("");

//   async function greet() {
//     // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//     setGreetMsg(await invoke("greet", { name }));
//   }

//   return (
//     <main className="container flex items-center">
//       <div className="row">
//         <a href="https://vitejs.dev" target="_blank">
//           <img src="/vite.svg" className="logo vite" alt="Vite logo" />
//         </a>
//         <a href="https://tauri.app" target="_blank">
//           <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
//         </a>
//         <a href="https://reactjs.org" target="_blank">
//           <img src={reactLogo} className="logo react" alt="React logo" />
//         </a>
//       </div>
//       <p>Click on the Tauri, Vite, and React logos to learn more.</p>

//       <form
//         className="row"
//         onSubmit={(e) => {
//           e.preventDefault();
//           greet();
//         }}
//       >
//         <input
//           id="greet-input"
//           onChange={(e) => setName(e.currentTarget.value)}
//           placeholder="Enter a name..."
//         />
//         <button type="submit">Greet</button>
//       </form>
//       <p>{greetMsg}</p>
//     </main>
//   );
// }

// export default App;


import { useState, useEffect } from 'react';


import BlockchainInfo from './component/BlockchainInfo/BlockchainInfo';
import Dashboard from './component/Dashboard/Dashboard';
import Loader from './component/Loader/Loader';

function App() {
  const [screen, setScreen] = useState('dashboard');
  const [isLoading, setIsLoading] = useState(true);

  // Simulate loading (e.g., API calls, initialization) for 2 seconds
  useEffect(() => {
    const timer = setTimeout(() => {
      setIsLoading(false);
    }, 2000);

    return () => clearTimeout(timer);
  }, []);

  const handleNavigateToBlockchainInfo = () => {
    setScreen('blockchainInfo');
  };

  const handleBackToDashboard = () => {
    setScreen('dashboard');
  };

  if (isLoading) {
    return <Loader />;
  }

  return (
    <>
      {screen === 'dashboard' && (
        <Dashboard onBlockchainInfoClick={handleNavigateToBlockchainInfo} />
      )}
      {screen === 'blockchainInfo' && (
        <BlockchainInfo onBack={handleBackToDashboard} />
      )}
    </>
  );
}

export default App;
