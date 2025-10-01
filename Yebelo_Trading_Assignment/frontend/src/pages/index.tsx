import React, { useState } from "react";
import TokenSelector from "../components/frontend/tokenselector";
import PriceChart from "../components/frontend/pricechart";
import RSIChart from "../components/frontend/rsichart";

const IndexPage: React.FC = () => {
  // Available tokens (dummy list, replace with API data later)
  const tokens = ["BTC", "ETH", "SOL", "ADA"];

  // State
  const [selectedToken, setSelectedToken] = useState(tokens[0]);

  // Dummy chart data (replace with real API later)
  const priceData = [
    { time: "10:00", price: 28000 },
    { time: "11:00", price: 28250 },
    { time: "12:00", price: 27950 },
    { time: "13:00", price: 28500 },
    { time: "14:00", price: 28300 },
  ];

  const rsiData = [
    { time: "10:00", rsi: 40 },
    { time: "11:00", rsi: 55 },
    { time: "12:00", rsi: 60 },
    { time: "13:00", rsi: 70 },
    { time: "14:00", rsi: 65 },
  ];

  return (
    <div className="min-h-screen bg-gray-100 dark:bg-gray-950 text-gray-900 dark:text-gray-100 p-6">
      <div className="max-w-6xl mx-auto space-y-6">
        {/* Token Selector */}
        <div className="flex justify-between items-center">
          <h1 className="text-2xl font-bold">Crypto Dashboard</h1>
          <TokenSelector
            tokens={tokens}
            selected={selectedToken}
            onChange={setSelectedToken}
          />
        </div>

        {/* Charts */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <PriceChart data={priceData} token={selectedToken} />
          <RSIChart data={rsiData} token={selectedToken} />
        </div>
      </div>
    </div>
  );
};

export default IndexPage;
