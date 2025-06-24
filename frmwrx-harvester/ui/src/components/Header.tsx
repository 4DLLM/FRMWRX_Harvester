import React from 'react';
import { Menu, MessageSquare, Download, BookOpen } from 'lucide-react';

interface HeaderProps {
  activeView: 'chat' | 'harvester' | 'knowledge';
  onMenuClick: () => void;
}

const Header: React.FC<HeaderProps> = ({ activeView, onMenuClick }) => {
  const getViewIcon = () => {
    switch (activeView) {
      case 'chat':
        return <MessageSquare className="w-5 h-5" />;
      case 'harvester':
        return <Download className="w-5 h-5" />;
      case 'knowledge':
        return <BookOpen className="w-5 h-5" />;
      default:
        return <MessageSquare className="w-5 h-5" />;
    }
  };

  const getViewTitle = () => {
    switch (activeView) {
      case 'chat':
        return 'Chat Interface';
      case 'harvester':
        return 'Document Harvester';
      case 'knowledge':
        return 'Knowledge Base';
      default:
        return 'Chat Interface';
    }
  };

  return (
    <header className="flex items-center justify-between px-6 py-4 border-b border-white/10 bg-gradient-surface">
      <div className="flex items-center gap-4">
        <button
          onClick={onMenuClick}
          className="btn btn-outline p-2 md:hidden"
          aria-label="Toggle sidebar"
        >
          <Menu className="w-5 h-5" />
        </button>
        
        <div className="flex items-center gap-3">
          {getViewIcon()}
          <h1 className="text-xl font-semibold mb-0">{getViewTitle()}</h1>
        </div>
      </div>

      <div className="flex items-center gap-4">
        <div className="hidden sm:flex items-center gap-2 text-sm opacity-60">
          <div className="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
          <span>FRMWRX Harvester v0.1.0</span>
        </div>
      </div>
    </header>
  );
};

export default Header;