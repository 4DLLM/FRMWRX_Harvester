import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Activity, CheckCircle, XCircle, Clock, AlertTriangle } from "lucide-react";

const ProgressMonitor = () => {
  const [progress, setProgress] = useState(null);
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const progressData = await invoke("get_harvest_progress");
        const parsed = JSON.parse(progressData);
        setProgress(parsed);
        setIsVisible(parsed.status === "InProgress" || parsed.processed_urls > 0);
      } catch (error) {
        console.error("Failed to get progress:", error);
      }
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  if (!isVisible || !progress) {
    return null;
  }

  const getStatusIcon = () => {
    switch (progress.status) {
      case "InProgress":
        return <Activity className="w-5 h-5 text-blue-500 animate-pulse" />;
      case "Completed":
        return <CheckCircle className="w-5 h-5 text-green-500" />;
      case "Failed":
        return <XCircle className="w-5 h-5 text-red-500" />;
      default:
        return <Clock className="w-5 h-5 text-gray-500" />;
    }
  };

  const getStatusColor = () => {
    switch (progress.status) {
      case "InProgress":
        return "border-blue-200 bg-blue-50";
      case "Completed":
        return "border-green-200 bg-green-50";
      case "Failed":
        return "border-red-200 bg-red-50";
      default:
        return "border-gray-200 bg-gray-50";
    }
  };

  const progressPercentage = progress.total_urls > 0 
    ? Math.round((progress.processed_urls / progress.total_urls) * 100)
    : 0;

  return (
    <div className="p-6">
      <div className="max-w-4xl mx-auto">
        <div className={`border rounded-lg p-6 ${getStatusColor()}`}>
          {/* Header */}
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center space-x-3">
              {getStatusIcon()}
              <div>
                <h3 className="text-lg font-semibold text-gray-900">Harvest Progress</h3>
                <p className="text-sm text-gray-600">
                  {progress.status === "InProgress" && "Processing documents..."}
                  {progress.status === "Completed" && "Harvest completed successfully"}
                  {progress.status === "Failed" && "Harvest completed with errors"}
                  {progress.status === "Idle" && "Ready to harvest"}
                </p>
              </div>
            </div>
            <div className="text-right">
              <div className="text-2xl font-bold text-gray-900">
                {progressPercentage}%
              </div>
              <div className="text-sm text-gray-600">
                {progress.processed_urls} / {progress.total_urls}
              </div>
            </div>
          </div>

          {/* Progress Bar */}
          <div className="mb-4">
            <div className="progress-bar h-2">
              <div 
                className="progress-fill h-full transition-all duration-300"
                style={{ width: `${progressPercentage}%` }}
              />
            </div>
          </div>

          {/* Stats */}
          <div className="grid grid-cols-3 gap-4 mb-4">
            <div className="text-center">
              <div className="text-lg font-semibold text-green-600">
                {progress.successful_documents}
              </div>
              <div className="text-sm text-gray-600">Successful</div>
            </div>
            <div className="text-center">
              <div className="text-lg font-semibold text-red-600">
                {progress.failed_documents}
              </div>
              <div className="text-sm text-gray-600">Failed</div>
            </div>
            <div className="text-center">
              <div className="text-lg font-semibold text-gray-600">
                {progress.total_urls - progress.processed_urls}
              </div>
              <div className="text-sm text-gray-600">Remaining</div>
            </div>
          </div>

          {/* Current URL */}
          {progress.current_url && (
            <div className="mb-4 p-3 bg-white rounded border">
              <div className="text-sm font-medium text-gray-700 mb-1">Currently processing:</div>
              <div className="text-sm text-gray-600 font-mono break-all">
                {progress.current_url}
              </div>
            </div>
          )}

          {/* Errors */}
          {progress.errors && progress.errors.length > 0 && (
            <div className="mt-4">
              <div className="flex items-center space-x-2 mb-2">
                <AlertTriangle className="w-4 h-4 text-amber-500" />
                <span className="text-sm font-medium text-gray-700">
                  Errors ({progress.errors.length})
                </span>
              </div>
              <div className="max-h-32 overflow-y-auto space-y-1">
                {progress.errors.slice(0, 5).map((error, index) => (
                  <div key={index} className="text-xs text-red-600 bg-white p-2 rounded border">
                    {error}
                  </div>
                ))}
                {progress.errors.length > 5 && (
                  <div className="text-xs text-gray-500 p-2">
                    ... and {progress.errors.length - 5} more errors
                  </div>
                )}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default ProgressMonitor;