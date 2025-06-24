import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Download, Plus, Trash2, Globe, FileText, AlertCircle } from "lucide-react";

const HarvesterControls = ({ onStatusChange }) => {
  const [urls, setUrls] = useState("");
  const [isHarvesting, setIsHarvesting] = useState(false);
  const [error, setError] = useState(null);
  const [success, setSuccess] = useState(null);

  const parseUrls = (text) => {
    return text
      .split('\n')
      .map(url => url.trim())
      .filter(url => url.length > 0 && (url.startsWith('http://') || url.startsWith('https://')));
  };

  const handleStartHarvest = async () => {
    const urlList = parseUrls(urls);
    
    if (urlList.length === 0) {
      setError("Please enter at least one valid URL (must start with http:// or https://)");
      return;
    }

    setIsHarvesting(true);
    setError(null);
    setSuccess(null);

    try {
      const result = await invoke("start_harvest", { urls: urlList });
      setSuccess(`Successfully started harvesting ${urlList.length} URLs`);
      onStatusChange?.();
    } catch (error) {
      console.error("Harvest error:", error);
      setError(`Failed to start harvest: ${error}`);
    } finally {
      setIsHarvesting(false);
    }
  };

  const handleClearUrls = () => {
    setUrls("");
    setError(null);
    setSuccess(null);
  };

  const addSampleUrls = () => {
    const samples = [
      "https://docs.rs/tokio/latest/tokio/",
      "https://tauri.app/v1/guides/getting-started/prerequisites/",
      "https://doc.rust-lang.org/book/ch01-00-getting-started.html"
    ];
    
    const currentUrls = urls.trim();
    const newUrls = currentUrls ? currentUrls + '\n' + samples.join('\n') : samples.join('\n');
    setUrls(newUrls);
  };

  const urlCount = parseUrls(urls).length;

  return (
    <div className="p-6">
      <div className="max-w-4xl mx-auto">
        <div className="bg-white rounded-lg shadow-sm border">
          {/* Header */}
          <div className="px-6 py-4 border-b border-gray-200">
            <div className="flex items-center justify-between">
              <div className="flex items-center space-x-3">
                <div className="w-8 h-8 bg-green-500 rounded-lg flex items-center justify-center">
                  <Download className="w-4 h-4 text-white" />
                </div>
                <div>
                  <h2 className="text-lg font-semibold text-gray-900">Document Harvester</h2>
                  <p className="text-sm text-gray-500">Add URLs to harvest documents and build your knowledge base</p>
                </div>
              </div>
              <div className="flex items-center space-x-2">
                <Globe className="w-4 h-4 text-gray-400" />
                <span className="text-sm text-gray-600">{urlCount} URLs ready</span>
              </div>
            </div>
          </div>

          {/* Content */}
          <div className="p-6">
            {/* URL Input */}
            <div className="mb-6">
              <label htmlFor="urls" className="block text-sm font-medium text-gray-700 mb-2">
                URLs to Harvest
              </label>
              <textarea
                id="urls"
                value={urls}
                onChange={(e) => setUrls(e.target.value)}
                placeholder="Enter URLs, one per line:
https://example.com/document1.pdf
https://example.com/page.html
https://docs.example.com/guide"
                className="url-input w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono text-sm"
                disabled={isHarvesting}
              />
              <div className="mt-2 flex items-center justify-between text-sm text-gray-500">
                <span>Enter one URL per line. Supports HTML, PDF, and text documents.</span>
                <span>{urlCount} valid URLs</span>
              </div>
            </div>

            {/* Sample URLs */}
            <div className="mb-6">
              <button
                onClick={addSampleUrls}
                className="flex items-center space-x-2 text-sm text-blue-600 hover:text-blue-700"
                disabled={isHarvesting}
              >
                <Plus className="w-4 h-4" />
                <span>Add sample Rust documentation URLs</span>
              </button>
            </div>

            {/* Status Messages */}
            {error && (
              <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-lg">
                <div className="flex items-start space-x-2">
                  <AlertCircle className="w-5 h-5 text-red-500 flex-shrink-0 mt-0.5" />
                  <div>
                    <h4 className="text-sm font-medium text-red-800">Error</h4>
                    <p className="text-sm text-red-700 mt-1">{error}</p>
                  </div>
                </div>
              </div>
            )}

            {success && (
              <div className="mb-4 p-4 bg-green-50 border border-green-200 rounded-lg">
                <div className="flex items-start space-x-2">
                  <FileText className="w-5 h-5 text-green-500 flex-shrink-0 mt-0.5" />
                  <div>
                    <h4 className="text-sm font-medium text-green-800">Success</h4>
                    <p className="text-sm text-green-700 mt-1">{success}</p>
                  </div>
                </div>
              </div>
            )}

            {/* Actions */}
            <div className="flex items-center justify-between">
              <button
                onClick={handleClearUrls}
                className="flex items-center space-x-2 px-4 py-2 text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-lg"
                disabled={isHarvesting || !urls.trim()}
              >
                <Trash2 className="w-4 h-4" />
                <span>Clear</span>
              </button>

              <button
                onClick={handleStartHarvest}
                disabled={urlCount === 0 || isHarvesting}
                className="flex items-center space-x-2 px-6 py-3 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isHarvesting ? (
                  <>
                    <div className="spinner" />
                    <span>Harvesting...</span>
                  </>
                ) : (
                  <>
                    <Download className="w-4 h-4" />
                    <span>Start Harvest ({urlCount} URLs)</span>
                  </>
                )}
              </button>
            </div>
          </div>
        </div>

        {/* Info Panel */}
        <div className="mt-6 bg-blue-50 border border-blue-200 rounded-lg p-4">
          <h3 className="text-sm font-medium text-blue-800 mb-2">How it works</h3>
          <ul className="text-sm text-blue-700 space-y-1">
            <li>• Documents are downloaded and processed concurrently</li>
            <li>• Text content is extracted and chunked for better search</li>
            <li>• Embeddings are generated for semantic similarity search</li>
            <li>• Documents are securely validated before processing</li>
          </ul>
        </div>
      </div>
    </div>
  );
};

export default HarvesterControls;