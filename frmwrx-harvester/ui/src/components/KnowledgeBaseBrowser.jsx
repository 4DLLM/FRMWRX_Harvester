import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Search, FileText, Globe, Calendar, Hash } from "lucide-react";

const KnowledgeBaseBrowser = () => {
  const [documents, setDocuments] = useState([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [isSearching, setIsSearching] = useState(false);
  const [selectedDocument, setSelectedDocument] = useState(null);

  useEffect(() => {
    // Load all documents on mount
    handleSearch("");
  }, []);

  const handleSearch = async (query = searchQuery) => {
    setIsSearching(true);
    try {
      const results = await invoke("search_documents", { 
        query: query || " ", // Use space for "all documents" search
        limit: 50 
      });
      setDocuments(results);
    } catch (error) {
      console.error("Search failed:", error);
      setDocuments([]);
    } finally {
      setIsSearching(false);
    }
  };

  const handleSearchSubmit = (e) => {
    e.preventDefault();
    handleSearch();
  };

  const formatDate = (dateString) => {
    try {
      return new Date(dateString).toLocaleDateString();
    } catch {
      return "Unknown";
    }
  };

  const truncateText = (text, maxLength = 150) => {
    if (text.length <= maxLength) return text;
    return text.substring(0, maxLength) + "...";
  };

  return (
    <div className="h-full flex">
      {/* Search and Results Panel */}
      <div className="w-1/2 border-r border-gray-200 flex flex-col">
        {/* Search Header */}
        <div className="bg-white border-b border-gray-200 p-6">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">Knowledge Base</h2>
          <form onSubmit={handleSearchSubmit} className="flex space-x-3">
            <div className="flex-1 relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search documents..."
                className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              />
            </div>
            <button
              type="submit"
              disabled={isSearching}
              className="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:opacity-50"
            >
              {isSearching ? "Searching..." : "Search"}
            </button>
          </form>
        </div>

        {/* Results */}
        <div className="flex-1 overflow-y-auto scrollbar-thin">
          {documents.length === 0 ? (
            <div className="flex items-center justify-center h-full text-gray-500">
              <div className="text-center">
                <FileText className="w-12 h-12 mx-auto mb-4 text-gray-400" />
                <h3 className="text-lg font-medium mb-2">No Documents Found</h3>
                <p className="text-sm">
                  {searchQuery 
                    ? "Try a different search term or harvest some documents first."
                    : "Start by harvesting documents to build your knowledge base."
                  }
                </p>
              </div>
            </div>
          ) : (
            <div className="p-4 space-y-3">
              {documents.map((doc, index) => (
                <div
                  key={index}
                  onClick={() => setSelectedDocument(doc)}
                  className={`p-4 border rounded-lg cursor-pointer transition-colors ${
                    selectedDocument === doc
                      ? "border-blue-500 bg-blue-50"
                      : "border-gray-200 hover:border-gray-300 hover:bg-gray-50"
                  }`}
                >
                  <div className="flex items-start justify-between mb-2">
                    <div className="flex items-center space-x-2">
                      <FileText className="w-4 h-4 text-gray-500 flex-shrink-0" />
                      <span className="font-medium text-gray-900 text-sm">
                        Document #{index + 1}
                      </span>
                    </div>
                    <div className="flex items-center space-x-1 text-xs text-gray-500">
                      <Hash className="w-3 h-3" />
                      <span>{doc.length} chars</span>
                    </div>
                  </div>
                  
                  <p className="text-sm text-gray-600 line-clamp-3">
                    {truncateText(doc)}
                  </p>
                  
                  <div className="mt-2 flex items-center space-x-4 text-xs text-gray-500">
                    <div className="flex items-center space-x-1">
                      <Globe className="w-3 h-3" />
                      <span>Source</span>
                    </div>
                    <div className="flex items-center space-x-1">
                      <Calendar className="w-3 h-3" />
                      <span>Recently added</span>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Document Viewer Panel */}
      <div className="w-1/2 flex flex-col">
        {selectedDocument ? (
          <>
            {/* Document Header */}
            <div className="bg-white border-b border-gray-200 p-6">
              <div className="flex items-center space-x-3 mb-2">
                <div className="w-8 h-8 bg-blue-500 rounded-lg flex items-center justify-center">
                  <FileText className="w-4 h-4 text-white" />
                </div>
                <div>
                  <h3 className="text-lg font-semibold text-gray-900">Document Viewer</h3>
                  <p className="text-sm text-gray-600">{selectedDocument.length} characters</p>
                </div>
              </div>
            </div>

            {/* Document Content */}
            <div className="flex-1 overflow-y-auto scrollbar-thin p-6">
              <div className="prose prose-sm max-w-none">
                <div className="whitespace-pre-wrap text-gray-800 leading-relaxed">
                  {selectedDocument}
                </div>
              </div>
            </div>
          </>
        ) : (
          <div className="flex items-center justify-center h-full text-gray-500">
            <div className="text-center">
              <FileText className="w-12 h-12 mx-auto mb-4 text-gray-400" />
              <h3 className="text-lg font-medium mb-2">Select a Document</h3>
              <p className="text-sm">
                Choose a document from the search results to view its content.
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default KnowledgeBaseBrowser;