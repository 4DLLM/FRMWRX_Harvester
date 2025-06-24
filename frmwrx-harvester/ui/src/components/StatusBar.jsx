import React from "react";
import { Activity, Database, Wifi, Clock } from "lucide-react";

const StatusBar = ({ systemStatus }) => {
  const formatMemoryUsage = (bytes) => {
    if (!bytes) return "0 MB";
    const mb = bytes / (1024 * 1024);
    return `${mb.toFixed(1)} MB`;
  };

  const getStatusColor = () => {
    if (!systemStatus) return "text-gray-500";
    return systemStatus.status === "running" ? "text-green-600" : "text-red-600";
  };

  return (
    <footer className="bg-white border-t border-gray-200 px-6 py-2">
      <div className="flex items-center justify-between text-sm">
        {/* Left side - System status */}
        <div className="flex items-center space-x-6">
          <div className="flex items-center space-x-1">
            <Activity className={`w-3 h-3 ${getStatusColor()}`} />
            <span className="text-gray-600">
              Status: <span className={getStatusColor()}>
                {systemStatus?.status || "Unknown"}
              </span>
            </span>
          </div>
          
          {systemStatus && (
            <div className="flex items-center space-x-1">
              <Database className="w-3 h-3 text-gray-500" />
              <span className="text-gray-600">
                Documents: <span className="font-mono">{systemStatus.document_count}</span>
              </span>
            </div>
          )}
        </div>

        {/* Right side - Configuration info */}
        <div className="flex items-center space-x-6 text-gray-500">
          {systemStatus?.config && (
            <>
              <div className="flex items-center space-x-1">
                <span>Max PDF Size: {Math.round(systemStatus.config.max_pdf_size / 1024 / 1024)}MB</span>
              </div>
              <div className="flex items-center space-x-1">
                <Wifi className="w-3 h-3" />
                <span>Max Downloads: {systemStatus.config.max_concurrent_downloads}</span>
              </div>
            </>
          )}
          
          <div className="flex items-center space-x-1">
            <Clock className="w-3 h-3" />
            <span>{new Date().toLocaleTimeString()}</span>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default StatusBar;