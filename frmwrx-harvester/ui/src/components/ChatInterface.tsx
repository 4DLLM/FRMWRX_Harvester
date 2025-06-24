import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Send, User, Bot, Loader2, Trash2 } from 'lucide-react';

interface ChatMessage {
  id: string;
  content: string;
  sender: 'user' | 'assistant';
  timestamp: number;
}

interface ChatResponse {
  message: ChatMessage;
  success: boolean;
  error?: string;
}

const ChatInterface: React.FC = () => {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [inputValue, setInputValue] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isLoadingHistory, setIsLoadingHistory] = useState(true);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  useEffect(() => {
    loadChatHistory();
  }, []);

  const loadChatHistory = async () => {
    try {
      const history = await invoke<ChatMessage[]>('get_chat_history');
      setMessages(history);
    } catch (error) {
      console.error('Failed to load chat history:', error);
    } finally {
      setIsLoadingHistory(false);
    }
  };

  const sendMessage = async () => {
    if (!inputValue.trim() || isLoading) return;

    const userMessage: ChatMessage = {
      id: Date.now().toString(),
      content: inputValue,
      sender: 'user',
      timestamp: Date.now(),
    };

    setMessages(prev => [...prev, userMessage]);
    setInputValue('');
    setIsLoading(true);

    try {
      const response = await invoke<ChatResponse>('send_chat_message', {
        message: inputValue,
      });

      if (response.success) {
        setMessages(prev => [...prev, response.message]);
      } else {
        // Add error message
        const errorMessage: ChatMessage = {
          id: Date.now().toString(),
          content: response.error || 'An error occurred while processing your message.',
          sender: 'assistant',
          timestamp: Date.now(),
        };
        setMessages(prev => [...prev, errorMessage]);
      }
    } catch (error) {
      console.error('Failed to send message:', error);
      const errorMessage: ChatMessage = {
        id: Date.now().toString(),
        content: 'Failed to connect to the chat service. Please try again.',
        sender: 'assistant',
        timestamp: Date.now(),
      };
      setMessages(prev => [...prev, errorMessage]);
    } finally {
      setIsLoading(false);
    }
  };

  const clearHistory = async () => {
    try {
      await invoke('clear_chat_history');
      setMessages([]);
    } catch (error) {
      console.error('Failed to clear chat history:', error);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  };

  const formatTime = (timestamp: number) => {
    return new Date(timestamp).toLocaleTimeString([], {
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  if (isLoadingHistory) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="flex items-center gap-2 text-white opacity-60">
          <Loader2 className="w-5 h-5 animate-spin" />
          <span>Loading chat history...</span>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full">
      {/* Chat Header */}
      <div className="card-header flex items-center justify-between border-b border-white/10">
        <div>
          <h2 className="text-xl font-semibold mb-0">FRMWRX Assistant</h2>
          <p className="text-sm opacity-60 mb-0">
            Ask questions about your harvested documents
          </p>
        </div>
        <button
          onClick={clearHistory}
          className="btn btn-outline"
          title="Clear chat history"
          disabled={messages.length === 0}
        >
          <Trash2 className="w-4 h-4" />
        </button>
      </div>

      {/* Messages Area */}
      <div className="flex-1 overflow-y-auto p-6 space-y-4">
        {messages.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-center">
            <Bot className="w-16 h-16 opacity-30 mb-4" />
            <h3 className="text-lg font-medium mb-2">Welcome to FRMWRX</h3>
            <p className="opacity-60 max-w-md">
              Start a conversation with your AI assistant. Ask questions about your documents,
              request information, or explore your knowledge base.
            </p>
          </div>
        ) : (
          messages.map((message) => (
            <div
              key={message.id}
              className={`flex gap-3 animate-fade-in ${
                message.sender === 'user' ? 'justify-end' : ''
              }`}
            >
              {message.sender === 'assistant' && (
                <div className="w-8 h-8 rounded-full bg-gradient-primary flex items-center justify-center flex-shrink-0">
                  <Bot className="w-4 h-4" />
                </div>
              )}
              
              <div
                className={`max-w-[70%] rounded-lg p-4 ${
                  message.sender === 'user'
                    ? 'bg-gradient-primary ml-auto'
                    : 'bg-gradient-secondary'
                }`}
              >
                <div className="text-sm opacity-90 whitespace-pre-wrap break-words">
                  {message.content}
                </div>
                <div className="text-xs opacity-50 mt-2">
                  {formatTime(message.timestamp)}
                </div>
              </div>

              {message.sender === 'user' && (
                <div className="w-8 h-8 rounded-full bg-gradient-secondary flex items-center justify-center flex-shrink-0">
                  <User className="w-4 h-4" />
                </div>
              )}
            </div>
          ))
        )}
        
        {isLoading && (
          <div className="flex gap-3 animate-fade-in">
            <div className="w-8 h-8 rounded-full bg-gradient-primary flex items-center justify-center flex-shrink-0">
              <Bot className="w-4 h-4" />
            </div>
            <div className="bg-gradient-secondary rounded-lg p-4 max-w-[70%]">
              <div className="flex items-center gap-2 text-sm opacity-80">
                <Loader2 className="w-4 h-4 animate-spin" />
                <span>Thinking...</span>
              </div>
            </div>
          </div>
        )}
        
        <div ref={messagesEndRef} />
      </div>

      {/* Input Area */}
      <div className="card-footer border-t border-white/10">
        <div className="flex gap-2">
          <textarea
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyPress={handleKeyPress}
            placeholder="Ask me anything about your documents..."
            className="input textarea flex-1 resize-none"
            rows={1}
            style={{
              minHeight: '2.5rem',
              maxHeight: '8rem',
              height: Math.min(8, Math.max(1, inputValue.split('\n').length)) * 1.5 + 'rem',
            }}
            disabled={isLoading}
          />
          <button
            onClick={sendMessage}
            disabled={!inputValue.trim() || isLoading}
            className="btn btn-primary px-4"
          >
            {isLoading ? (
              <Loader2 className="w-4 h-4 animate-spin" />
            ) : (
              <Send className="w-4 h-4" />
            )}
          </button>
        </div>
        <div className="text-xs opacity-50 mt-2">
          Press Enter to send, Shift+Enter for new line
        </div>
      </div>
    </div>
  );
};

export default ChatInterface;