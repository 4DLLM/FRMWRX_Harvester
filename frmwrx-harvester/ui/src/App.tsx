import React, { useState } from 'react';
import ChatInterface from './components/ChatInterface';
import HarvesterControls from './components/HarvesterControls';
import KnowledgeBaseBrowser from './components/KnowledgeBaseBrowser';
import Header from './components/Header';
import Sidebar from './components/Sidebar';

type ActiveView = 'chat' | 'harvester' | 'knowledge';

function App() {
  const [activeView, setActiveView] = useState<ActiveView>('chat');
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);

  const renderActiveView = () => {
    switch (activeView) {
      case 'chat':
        return <ChatInterface />;
      case 'harvester':
        return <HarvesterControls />;
      case 'knowledge':
        return <KnowledgeBaseBrowser />;
      default:
        return <ChatInterface />;
    }
  };

  return (
    <div className="flex h-full w-full">
      {/* Sidebar */}
      <Sidebar 
        isOpen={isSidebarOpen}
        activeView={activeView}
        onViewChange={setActiveView}
        onToggle={() => setIsSidebarOpen(!isSidebarOpen)}
      />
      
      {/* Main Content */}
      <div className="flex-1 flex flex-col">
        {/* Header */}
        <Header 
          activeView={activeView}
          onMenuClick={() => setIsSidebarOpen(!isSidebarOpen)}
        />
        
        {/* Content Area */}
        <main className="flex-1 overflow-hidden">
          {renderActiveView()}
        </main>
      </div>
    </div>
  );
}

export default App;