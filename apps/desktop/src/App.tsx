
import { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { Copy } from 'lucide-react';
import './App.css';

export default function Home() {
  const [color, setColor] = useState<string>("#FFFFFF");
  const [copied, setCopied] = useState<boolean>(false);
  const [isColorLight, setIsColorLight] = useState<boolean>(false);

  useEffect(() => {
    const unlisten = listen<string>("color-update", (event) => {
      const pickedColor = event.payload;
      setColor(pickedColor);


      const r = Number.parseInt(pickedColor.substring(1, 3), 16);
      const g = Number.parseInt(pickedColor.substring(3, 5), 16);
      const b = Number.parseInt(pickedColor.substring(5, 7), 16);
      const brightness = (r * 299 + g * 587 + b * 114) / 1000;

      setIsColorLight(brightness > 200);
    });

    return () => {
      unlisten.then((unsub) => unsub());
    };
  }, []);

  const handleCopy = () => {
    navigator.clipboard.writeText(color);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="grid grid-cols-2 gap-6 w-[600px] h-[300px] p-6 bg-gradient-to-br from-slate-600 via-neutral-900 to-slate-950 shadow-lg">
      <div className="flex flex-col items-center justify-center">
        <img src="/LOGO.webp" alt="Logo" className="w-24 h-24 mb-3 rounded-full shadow-sm" />
        <h1 className="text-2xl font-bold text-gray-400">PixelPickr</h1>
      </div>
      <div
        className={`flex flex-col items-center justify-center shadow-lg rounded-xl p-6 relative transition-colors duration-300 ${
          isColorLight ? "bg-gray-800 text-white" : "bg-white text-gray-800"
        }`}
      >
        <div className="flex items-center justify-center mb-6">
          <p className="text-base font-medium mr-2">
            Detected Color: <span className="font-bold" style={{ color: isColorLight ? "#FFFFFF" : color }}>{color}</span>
          </p>
          <button
            type="button"
            onClick={handleCopy} 
            className={`flex items-center focus:outline-none relative ${
              isColorLight ? "text-white hover:text-gray-400" : "text-gray-700 hover:text-gray-900"
            } transition-colors duration-200`}
            title="Click to copy the color"
          >
            <Copy size={20} className="hover:scale-110 transform transition-transform duration-200" />
            {copied && (
              <div className={`absolute bottom-full mb-2 left-1/2 transform -translate-x-1/2 rounded-lg shadow-md px-2 py-1 text-sm ${
                isColorLight ? "bg-gray-600 text-white" : "bg-green-500 text-white"
              } transition-opacity duration-300`}>
                Color copied!
              </div>
            )}
          </button>
        </div>

        <div
          className="w-20 h-20 rounded-lg shadow-md transition-transform duration-200 transform hover:scale-105"
          style={{ backgroundColor: color }}
        />
      </div>
    </div>
  );
}
