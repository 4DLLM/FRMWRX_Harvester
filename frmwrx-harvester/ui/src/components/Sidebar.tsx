import React from 'react';
import { MessageSquare, Download, BookOpen, X, Zap } from 'lucide-react';

interface SidebarProps {
  isOpen: boolean;
  activeView: 'chat' | 'harvester' | 'knowledge';
  onViewChange: (view: 'chat' | 'harvester' | 'knowledge') => void;
  onToggle: () => void;
}

const Sidebar: React.FC<SidebarProps> = ({ isOpen, activeView, onViewChange, onToggle }) => {
  const navigationItems = [
    {
      id: 'chat' as const,
      icon: MessageSquare,
      label: 'Chat',
      description: 'AI Assistant',
    },
    {
      id: 'harvester' as const,
      icon: Download,
      label: 'Harvester',
      description: 'Document Processing',
    },
    {
      id: 'knowledge' as const,
      icon: BookOpen,
      label: 'Knowledge Base',
      description: 'Browse Documents',
    },
  ];

  return (
    <>
      {/* Overlay for mobile */}
      {isOpen && (
        <div
          className="fixed inset-0 bg-black/50 z-40 md:hidden"
          onClick={onToggle}
        />
      )}

      {/* Sidebar */}
      <aside
        className={`fixed md:relative inset-y-0 left-0 z-50 w-64 bg-gradient-surface border-r border-white/10 transform transition-transform duration-300 ease-in-out ${
          isOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0'
        }`}
      >
        <div className="flex flex-col h-full">
          {/* Header */}
          <div className="flex items-center justify-between p-6 border-b border-white/10">
            <div className="flex items-center gap-3">
              <div className="w-8 h-8 rounded-lg bg-gradient-primary flex items-center justify-center">
                <Zap className="w-5 h-5" />
              </div>
              <div>
                <h2 className="font-semibold text-lg mb-0">FRMWRX</h2>
                <p className="text-xs opacity-60 mb-0">Harvester</p>
              </div>
            </div>
            
            <button
              onClick={onToggle}
              className="btn btn-outline p-2 md:hidden"
              aria-label="Close sidebar"
            >
              <X className="w-4 h-4" />
            </button>
          </div>

          {/* Navigation */}
          <nav className="flex-1 p-4 space-y-2">
            {navigationItems.map((item) => {
              const Icon = item.icon;
              const isActive = activeView === item.id;
              
              return (
                <button
                  key={item.id}
                  onClick={() => {
                    onViewChange(item.id);
                    // Close sidebar on mobile after selection
                    if (window.innerWidth < 768) {
                      onToggle();
                    }
                  }}
                  className={`w-full flex items-center gap-3 p-3 rounded-lg transition-all ${
                    isActive
                      ? 'bg-gradient-primary shadow-md'
                      : 'hover:bg-white/10'
                  }`}
                >
                  <Icon className="w-5 h-5 flex-shrink-0" />
                  <div className="text-left">
                    <div className="font-medium">{item.label}</div>
                    <div className="text-xs opacity-60">{item.description}</div>
                  </div>
                </button>
              );
            })}
          </nav>

          {/* Footer */}
          <div className="p-4 border-t border-white/10">
            <div className="text-xs opacity-60 text-center">
              <p className="mb-1">Consciousness-Aware AI Platform</p>
              <p>Built with Rust + Tauri</p>
            </div>
          </div>
        </div>
      </aside>
    </>
  );
};

export default Sidebar;