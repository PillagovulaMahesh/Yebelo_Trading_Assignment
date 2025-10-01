import React from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  Tooltip,
  CartesianGrid,
  ReferenceLine,
  ResponsiveContainer,
} from "recharts";

type RSIData = {
  time: string;
  rsi: number;
};

interface RSIChartProps {
  data: RSIData[];
  token: string;
}

const RSIChart: React.FC<RSIChartProps> = ({ data, token }) => {
  return (
    <div className="p-4 bg-white dark:bg-gray-900 rounded-2xl shadow-md">
      <h2 className="text-lg font-semibold mb-3">{token} RSI Chart</h2>
      <ResponsiveContainer width="100%" height={250}>
        <LineChart data={data}>
          <CartesianGrid strokeDasharray="3 3" stroke="#ccc" />
          <XAxis dataKey="time" />
          <YAxis domain={[0, 100]} />
          <Tooltip />
          <ReferenceLine y={70} stroke="red" strokeDasharray="4 4" />
          <ReferenceLine y={30} stroke="green" strokeDasharray="4 4" />
          <Line
            type="monotone"
            dataKey="rsi"
            stroke="#10b981"
            strokeWidth={2}
            dot={false}
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
};

export default RSIChart;
