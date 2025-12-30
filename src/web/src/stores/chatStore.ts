/**
 * Chat store - message history and sessions
 */

import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { ChatMessage, ChatSession } from '../types/chat';

interface ChatStore {
  // Current session
  currentSessionId: string | null;
  sessions: ChatSession[];

  // Actions
  createSession: (title?: string) => string;
  deleteSession: (id: string) => void;
  setCurrentSession: (id: string) => void;

  // Messages
  addMessage: (message: Omit<ChatMessage, 'id' | 'timestamp'>) => void;
  updateMessage: (id: string, updates: Partial<ChatMessage>) => void;
  clearMessages: () => void;

  // Current messages
  getCurrentMessages: () => ChatMessage[];
}

export const useChatStore = create<ChatStore>()(
  persist(
    (set, get) => ({
      currentSessionId: null,
      sessions: [],

      createSession: (title = 'New Chat') => {
        const id = `session-${Date.now()}`;
        const session: ChatSession = {
          id,
          title,
          messages: [],
          createdAt: Date.now(),
          updatedAt: Date.now(),
        };

        set((state) => ({
          sessions: [...state.sessions, session],
          currentSessionId: id,
        }));

        return id;
      },

      deleteSession: (id) => {
        set((state) => {
          const newSessions = state.sessions.filter((s) => s.id !== id);
          const newCurrentId = state.currentSessionId === id
            ? newSessions[0]?.id || null
            : state.currentSessionId;

          return {
            sessions: newSessions,
            currentSessionId: newCurrentId,
          };
        });
      },

      setCurrentSession: (id) => {
        set({ currentSessionId: id });
      },

      addMessage: (message) => {
        const { currentSessionId, sessions } = get();

        if (!currentSessionId) {
          // Create new session if none exists
          const newSessionId = get().createSession();
          set({ currentSessionId: newSessionId });
        }

        const newMessage: ChatMessage = {
          ...message,
          id: `msg-${Date.now()}-${Math.random()}`,
          timestamp: Date.now(),
        };

        set((state) => ({
          sessions: state.sessions.map((session) =>
            session.id === state.currentSessionId
              ? {
                  ...session,
                  messages: [...session.messages, newMessage],
                  updatedAt: Date.now(),
                }
              : session
          ),
        }));
      },

      updateMessage: (id, updates) => {
        set((state) => ({
          sessions: state.sessions.map((session) =>
            session.id === state.currentSessionId
              ? {
                  ...session,
                  messages: session.messages.map((msg) =>
                    msg.id === id ? { ...msg, ...updates } : msg
                  ),
                  updatedAt: Date.now(),
                }
              : session
          ),
        }));
      },

      clearMessages: () => {
        set((state) => ({
          sessions: state.sessions.map((session) =>
            session.id === state.currentSessionId
              ? { ...session, messages: [], updatedAt: Date.now() }
              : session
          ),
        }));
      },

      getCurrentMessages: () => {
        const { currentSessionId, sessions } = get();
        const session = sessions.find((s) => s.id === currentSessionId);
        return session?.messages || [];
      },
    }),
    {
      name: 'neurograph-chat-storage',
    }
  )
);
