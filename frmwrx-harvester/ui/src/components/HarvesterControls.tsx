import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Play, Square, Plus, Trash2, Link, Loader2, AlertCircle, CheckCircle } from 'lucide-react';

interface HarvestProgress {
  total_documents: number;
  processed_documents: number;
  current_document?: string;
  status: string;
}

const HarvesterControls: React.FC = () => {
  const [urls, setUrls] = useState<string[]>(['']);
  const [isHarvesting, setIsHarvesting] = useState(false);
  const [progress, setProgress] = useState<HarvestProgress>({
    total_documents: 0,
    processed_documents: 0,
    status: 'Ready',
  });
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  const addUrlField = () => {
    setUrls([...urls, '']);
  };

  const removeUrlField = (index: number) => {
    setUrls(urls.filter((_, i) => i !== index));
  };

  const updateUrl = (index: number, value: string) => {
    const newUrls = [...urls];
    newUrls[index] = value;
    setUrls(newUrls);
  };

  const getValidUrls = () => {
    return urls.filter(url => url.trim() !== '');
  };

  const startHarvesting = async () => {
    const validUrls = getValidUrls();
    if (validUrls.length === 0) {
      setError('Please add at least one valid URL');
      return;
    }

    setIsHarvesting(true);
    setError(null);
    setSuccess(null);

    try {
      const result = await invoke<string>('start_document_harvest', {
        urls: validUrls,
      });
      setSuccess(result);
    } catch (err) {
      setError(err as string);
    } finally {
      setIsHarvesting(false);
    }
  };

  const updateProgress = async () => {
    if (!isHarvesting) return;
    
    try {
      const currentProgress = await invoke<HarvestProgress>('get_harvest_progress');
      setProgress(currentProgress);
    } catch (err) {
      console.error('Failed to get progress:', err);
    }
  };

  useEffect(() => {
    let interval: number;
    if (isHarvesting) {
      interval = window.setInterval(updateProgress, 1000);
    }
    return () => clearInterval(interval);
  }, [isHarvesting]);

  const getProgressPercentage = () => {
    if (progress.total_documents === 0) return 0;
    return (progress.processed_documents / progress.total_documents) * 100;
  };

  return (
    <div className="p-6 max-w-4xl mx-auto">
      <div className="space-y-6">
        {/* Header */}
        <div className="card">
          <div className="card-header">
            <h2 className="text-2xl font-semibold mb-2">Document Harvester</h2>
            <p className="opacity-60 mb-0">
              Harvest documents from web URLs and PDFs to build your knowledge base
            </p>
          </div>
        </div>

        {/* URL Input Section */}
        <div className="card">
          <div className="card-header">
            <h3 className="text-lg font-medium mb-0">Document URLs</h3>
          </div>
          <div className="card-content space-y-4">
            {urls.map((url, index) => (
              <div key={index} className="flex gap-2">
                <div className="flex-1 flex items-center gap-2">
                  <Link className="w-4 h-4 opacity-60 flex-shrink-0" />
                  <input
                    type="url"
                    value={url}
                    onChange={(e) => updateUrl(index, e.target.value)}
                    placeholder="https://example.com/document.pdf"
                    className="input flex-1"
                    disabled={isHarvesting}
                  />
                </div>
                {urls.length > 1 && (
                  <button
                    onClick={() => removeUrlField(index)}
                    className="btn btn-outline p-2"
                    disabled={isHarvesting}
                    title="Remove URL"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                )}
              </div>
            ))}
            
            <button
              onClick={addUrlField}
              className="btn btn-secondary"
              disabled={isHarvesting}
            >
              <Plus className="w-4 h-4 mr-2" />
              Add Another URL
            </button>
          </div>
        </div>

        {/* Control Buttons */}
        <div className="card">
          <div className="card-content">
            <div className="flex gap-4">
              <button
                onClick={startHarvesting}
                disabled={isHarvesting || getValidUrls().length === 0}
                className="btn btn-primary"
              >
                {isHarvesting ? (
                  <>
                    <Square className="w-4 h-4 mr-2" />
                    Harvesting...
                  </>
                ) : (
                  <>
                    <Play className="w-4 h-4 mr-2" />
                    Start Harvest
                  </>
                )}
              </button>

              <div className="text-sm opacity-60 flex items-center">
                {getValidUrls().length} document{getValidUrls().length !== 1 ? 's' : ''} ready
              </div>
            </div>
          </div>
        </div>

        {/* Progress Section */}
        {(isHarvesting || progress.total_documents > 0) && (
          <div className="card">
            <div className="card-header">
              <h3 className="text-lg font-medium mb-0">Progress</h3>
            </div>
            <div className="card-content space-y-4">
              <div>
                <div className="flex justify-between text-sm mb-2">
                  <span>Status: {progress.status}</span>
                  <span>
                    {progress.processed_documents} / {progress.total_documents} documents
                  </span>
                </div>
                <div className="w-full bg-gray-700 rounded-full h-2">
                  <div
                    className="bg-gradient-primary h-2 rounded-full transition-all duration-300"
                    style={{ width: `${getProgressPercentage()}%` }}
                  />
                </div>
              </div>

                             {progress.current_document && (
                 <div className="flex items-center gap-2 text-sm">
                   <Loader2 className="w-4 h-4 animate-spin" />
                   <span className="opacity-80">Processing:</span>
                   <span className="truncate">{progress.current_document}</span>
                 </div>
               )}
            </div>
          </div>
        )}

        {/* Success Message */}
        {success && (
          <div className="card border-green-500/30 bg-green-500/10">
            <div className="card-content">
              <div className="flex items-center gap-2 text-green-400">
                <CheckCircle className="w-5 h-5" />
                <span>{success}</span>
              </div>
            </div>
          </div>
        )}

        {/* Error Message */}
        {error && (
          <div className="card border-red-500/30 bg-red-500/10">
            <div className="card-content">
              <div className="flex items-center gap-2 text-red-400">
                <AlertCircle className="w-5 h-5" />
                <span>{error}</span>
              </div>
            </div>
          </div>
        )}

        {/* Info Section */}
        <div className="card">
          <div className="card-header">
            <h3 className="text-lg font-medium mb-0">Supported Formats</h3>
          </div>
          <div className="card-content">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 bg-blue-400 rounded-full" />
                <span>PDF Documents</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 bg-green-400 rounded-full" />
                <span>HTML Web Pages</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 bg-yellow-400 rounded-full" />
                <span>Plain Text Files</span>
              </div>
            </div>
            <p className="opacity-60 mt-4 text-sm">
              The harvester will automatically detect document types and extract relevant content
              for indexing in your knowledge base.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default HarvesterControls;