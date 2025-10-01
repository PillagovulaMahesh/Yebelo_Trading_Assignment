import React from "react";

interface TokenSelectorProps {
  tokens: string[];
  selected: string;
  onChange: (token: string) => void;
}

const TokenSelector: React.FC<TokenSelectorProps> = ({
  tokens,
  selected,
  onChange,
}) => {
  return (
    <div className="flex items-center space-x-2">
      <label className="font-medium">Select Token:</label>
      <select
        value={selected}
        onChange={(e) => onChange(e.target.value)}
        className="p-2 border rounded-lg bg-white dark:bg-gray-800 dark:text-white"
      >
        {tokens.map((token) => (
          <option key={token} value={token}>
            {token}
          </option>
        ))}
      </select>
    </div>
  );
};

export default TokenSelector;
