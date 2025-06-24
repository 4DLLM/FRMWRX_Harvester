import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { 
  MessageSquare, 
  Download, 
  Settings, 
  Search,
  Send,
  FileText,
  Activity,
  AlertCircle,
  CheckCircle
} from "lucide-react";
import ChatInterface from "./components/ChatInterface";
import HarvesterControls from "./components/HarvesterControls";
import ProgressMonitor from "./components/ProgressMonitor";
import KnowledgeBaseBrowser from "./components/KnowledgeBaseBrowser";
import StatusBar from "./components/StatusBar";

function App() {
  const [activeTab, setActiveTab] = useState("chat");
  const [systemStatus, setSystemStatus] = useState(null);
  const [isLoading, setIsLoading] = useState(false);

  // Load system status on mount
  useEffect(() => {
    loadSystemStatus();
  }, []);

  const loadSystemStatus = async () => {
    try {
      const status = await invoke("get_system_status");
      setSystemStatus(JSON.parse(status));
    } catch (error) {
      console.error("Failed to load system status:", error);
    }
  };

  const tabs = [
    { id: "chat", label: "Chat", icon: MessageSquare },
    { id: "harvest", label: "Harvest", icon: Download },
    { id: "browse", label: "Browse", icon: Search },
    { id: "settings", label: "Settings", icon: Settings },
  ];

  const renderTabContent = () => {
    switch (activeTab) {
      case "chat":
        return <ChatInterface onStatusChange={loadSystemStatus} />;
      case "harvest":
        return (
          <div className="space-y-6">
            <HarvesterControls onStatusChange={loadSystemStatus} />
            <ProgressMonitor />
          </div>
        );
      case "browse":
        return <KnowledgeBaseBrowser />;
      case "settings":
        return (
          <div className="p-6">
            <h2 className="text-2xl font-bold mb-4">Settings</h2>
            <div className="bg-white rounded-lg p-6 shadow-sm border">
              <h3 className="text-lg font-semibold mb-3">Configuration</h3>
              <p className="text-gray-600">
                Settings are managed through the config.json file in the application directory.
                Restart the application after making changes.
              </p>
              {systemStatus && (
                <div className="mt-4 space-y-2">
                  <div className="flex items-center justify-between">
                    <span>Documents:</span>
                    <span className="font-mono">{systemStatus.document_count}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span>Max PDF Size:</span>
                    <span className="font-mono">{Math.round(systemStatus.config.max_pdf_size / 1024 / 1024)}MB</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span>Max Concurrent Downloads:</span>
                    <span className="font-mono">{systemStatus.config.max_concurrent_downloads}</span>
                  </div>
                </div>
              )}
            </div>
          </div>
        );
      default:
        return <ChatInterface onStatusChange={loadSystemStatus} />;
    }
  };

  return (
    <div className="flex flex-col h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white border-b border-gray-200 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <div className="w-8 h-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center">
              <FileText className="w-5 h-5 text-white" />
            </div>
            <div>
              <h1 className="text-xl font-bold text-gray-900">FRMWRX Harvester</h1>
              <p className="text-sm text-gray-500">Document Processing & AI Chat</p>
            </div>
          </div>
          <div className="flex items-center space-x-2">
            {systemStatus && (
              <div className="flex items-center space-x-1 text-sm text-gray-600">
                <Activity className="w-4 h-4" />
                <span>{systemStatus.document_count} docs</span>
              </div>
            )}
            <div className="flex items-center space-x-1">
              <div className="w-2 h-2 bg-green-500 rounded-full"></div>
              <span className="text-sm text-gray-600">Running</span>
            </div>
          </div>
        </div>
      </header>

      {/* Navigation Tabs */}
      <nav className="bg-white border-b border-gray-200 px-6">
        <div className="flex space-x-8">
          {tabs.map((tab) => {
            const Icon = tab.icon;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`flex items-center space-x-2 py-4 px-2 border-b-2 font-medium text-sm transition-colors ${
                  activeTab === tab.id
                    ? "border-blue-500 text-blue-600"
                    : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
                }`}
              >
                <Icon className="w-4 h-4" />
                <span>{tab.label}</span>
              </button>
            );
          })}
        </div>
      </nav>

      {/* Main Content */}
      <main className="flex-1 overflow-hidden">
        {renderTabContent()}
      </main>

      {/* Status Bar */}
      <StatusBar systemStatus={systemStatus} />
    </div>
  );
}

export default App;