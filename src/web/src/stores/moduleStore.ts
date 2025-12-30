/**
 * Module store - modules status and control
 */

import { create } from 'zustand';
import type { Module } from '../types/modules';

interface ModuleStore {
  modules: Module[];
  setModules: (modules: Module[]) => void;
  updateModule: (id: string, updates: Partial<Module>) => void;

  selectedModule: string | null;
  setSelectedModule: (id: string | null) => void;

  loading: boolean;
  setLoading: (loading: boolean) => void;
}

export const useModuleStore = create<ModuleStore>((set) => ({
  modules: [],
  setModules: (modules) => set({ modules }),
  updateModule: (id, updates) =>
    set((state) => ({
      modules: state.modules.map((m) =>
        m.id === id ? { ...m, ...updates } : m
      ),
    })),

  selectedModule: null,
  setSelectedModule: (selectedModule) => set({ selectedModule }),

  loading: false,
  setLoading: (loading) => set({ loading }),
}));
