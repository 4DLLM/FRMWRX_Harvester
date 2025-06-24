import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Search, FileText, Globe, Calendar, Filter, RefreshCw } from 'lucide-react';

interface Document {
  id: string;
  title: string;
  url: string;
  content: string;
  type: string;
  processed_at: number;
}

const KnowledgeBaseBrowser: React.FC = () => {
  const [searchQuery, setSearchQuery] = useState('');
  const [searchResults, setSearchResults] = useState<string[]>([]);
  const [isSearching, setIsSearching] = useState(false);
  const [selectedFilter, setSelectedFilter] = useState<'all' | 'pdf' | 'html' | 'text'>('all');

  const handleSearch = async () => {
    if (!searchQuery.trim()) {
      setSearchResults([]);
      return;
    }

    setIsSearching(true);
    try {
      const results = await invoke<string[]>('search_documents', {
        query: searchQuery,
        limit: 20,
      });
      setSearchResults(results);
    } catch (error) {
      console.error('Search failed:', error);
      setSearchResults([]);
    } finally {
      setIsSearching(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };

  const formatDate = (timestamp: number) => {
    return new Date(timestamp).toLocaleDateString([], {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const getDocumentIcon = (type: string) => {
    switch (type.toLowerCase()) {
      case 'pdf':
        return <FileText className="w-4 h-4 text-red-400" />;
      case 'html':
        return <Globe className="w-4 h-4 text-blue-400" />;
      default:
        return <FileText className="w-4 h-4 text-gray-400" />;
    }
  };

  return (
    <div className="p-6 max-w-6xl mx-auto">
      <div className="space-y-6">
        {/* Header */}
        <div className="card">
          <div className="card-header">
            <h2 className="text-2xl font-semibold mb-2">Knowledge Base Browser</h2>
            <p className="opacity-60 mb-0">
              Search and explore your harvested documents
            </p>
          </div>
        </div>

        {/* Search Section */}
        <div className="card">
          <div className="card-content">
            <div className="flex gap-4 mb-4">
              <div className="flex-1 flex gap-2">
                <div className="relative flex-1">
                  <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 opacity-60" />
                  <input
                    type="text"
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                    onKeyPress={handleKeyPress}
                    placeholder="Search your documents..."
                    className="input pl-10 pr-4"
                    disabled={isSearching}
                  />
                </div>
                <button
                  onClick={handleSearch}
                  disabled={isSearching || !searchQuery.trim()}
                  className="btn btn-primary"
                >
                  {isSearching ? (
                    <RefreshCw className="w-4 h-4 animate-spin" />
                  ) : (
                    <Search className="w-4 h-4" />
                  )}
                </button>
              </div>
            </div>

            {/* Filters */}
            <div className="flex items-center gap-4">
              <div className="flex items-center gap-2">
                <Filter className="w-4 h-4 opacity-60" />
                <span className="text-sm opacity-80">Filter by type:</span>
              </div>
              <div className="flex gap-2">
                {[
                  { value: 'all' as const, label: 'All' },
                  { value: 'pdf' as const, label: 'PDFs' },
                  { value: 'html' as const, label: 'Web Pages' },
                  { value: 'text' as const, label: 'Text' },
                ].map((filter) => (
                  <button
                    key={filter.value}
                    onClick={() => setSelectedFilter(filter.value)}
                    className={`px-3 py-1 rounded text-sm transition-all ${
                      selectedFilter === filter.value
                        ? 'bg-gradient-primary text-white'
                        : 'bg-white/10 hover:bg-white/20'
                    }`}
                  >
                    {filter.label}
                  </button>
                ))}
              </div>
            </div>
          </div>
        </div>

        {/* Search Results */}
        {searchQuery && (
          <div className="card">
            <div className="card-header">
              <h3 className="text-lg font-medium mb-0">
                Search Results
                {searchResults.length > 0 && (
                  <span className="text-sm opacity-60 ml-2">
                    ({searchResults.length} results)
                  </span>
                )}
              </h3>
            </div>
            <div className="card-content">
              {isSearching ? (
                <div className="flex items-center justify-center py-8">
                  <div className="flex items-center gap-2 text-white opacity-60">
                    <RefreshCw className="w-5 h-5 animate-spin" />
                    <span>Searching documents...</span>
                  </div>
                </div>
              ) : searchResults.length > 0 ? (
                <div className="space-y-4">
                  {searchResults.map((result, index) => (
                    <div
                      key={index}
                      className="border border-white/10 rounded-lg p-4 hover:bg-white/5 transition-colors"
                    >
                      <div className="flex items-start gap-3">
                        {getDocumentIcon('text')}
                        <div className="flex-1">
                          <div className="flex items-center gap-2 mb-1">
                            <h4 className="font-medium">Search Result {index + 1}</h4>
                            <span className="text-xs opacity-60 bg-white/10 px-2 py-1 rounded">
                              Relevance Score
                            </span>
                          </div>
                          <p className="text-sm opacity-80 leading-relaxed">
                            {result}
                          </p>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="text-center py-8 opacity-60">
                  <Search className="w-12 h-12 mx-auto mb-3 opacity-30" />
                  <p>No documents found matching your search.</p>
                  <p className="text-sm mt-1">
                    Try different keywords or harvest more documents.
                  </p>
                </div>
              )}
            </div>
          </div>
        )}

        {/* Browse Section */}
        <div className="card">
          <div className="card-header">
            <h3 className="text-lg font-medium mb-0">Document Library</h3>
          </div>
          <div className="card-content">
            <div className="text-center py-8 opacity-60">
              <FileText className="w-12 h-12 mx-auto mb-3 opacity-30" />
              <p>Document browser coming soon!</p>
              <p className="text-sm mt-1">
                This will show all your harvested documents with full browsing capabilities.
              </p>
            </div>
          </div>
        </div>

        {/* Statistics */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="card">
            <div className="card-content text-center">
              <div className="text-2xl font-bold text-blue-400 mb-1">0</div>
              <div className="text-sm opacity-60">Total Documents</div>
            </div>
          </div>
          <div className="card">
            <div className="card-content text-center">
              <div className="text-2xl font-bold text-green-400 mb-1">0</div>
              <div className="text-sm opacity-60">Indexed Pages</div>
            </div>
          </div>
          <div className="card">
            <div className="card-content text-center">
              <div className="text-2xl font-bold text-yellow-400 mb-1">0 MB</div>
              <div className="text-sm opacity-60">Storage Used</div>
            </div>
          </div>
        </div>

        {/* Quick Actions */}
        <div className="card">
          <div className="card-header">
            <h3 className="text-lg font-medium mb-0">Quick Actions</h3>
          </div>
          <div className="card-content">
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
              <button className="btn btn-secondary justify-start">
                <Search className="w-4 h-4 mr-2" />
                Advanced Search
              </button>
              <button className="btn btn-secondary justify-start">
                <Calendar className="w-4 h-4 mr-2" />
                Recent Documents
              </button>
              <button className="btn btn-secondary justify-start">
                <FileText className="w-4 h-4 mr-2" />
                Export Library
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default KnowledgeBaseBrowser;