import type { StatusResponse, PresetsListResponse, Cell } from './types';

const API_BASE = '/api';

export const api = {
  // Control endpoints
  async start() {
    const res = await fetch(`${API_BASE}/control/start`, { method: 'POST' });
    return res.json() as Promise<StatusResponse>;
  },

  async stop() {
    const res = await fetch(`${API_BASE}/control/stop`, { method: 'POST' });
    return res.json() as Promise<StatusResponse>;
  },

  async pause() {
    const res = await fetch(`${API_BASE}/control/pause`, { method: 'POST' });
    return res.json() as Promise<StatusResponse>;
  },

  async resume() {
    const res = await fetch(`${API_BASE}/control/resume`, { method: 'POST' });
    return res.json() as Promise<StatusResponse>;
  },

  async step() {
    const res = await fetch(`${API_BASE}/control/step`, { method: 'POST' });
    return res.json() as Promise<StatusResponse>;
  },

  async setSpeed(tps: number) {
    const res = await fetch(`${API_BASE}/control/speed`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ tps }),
    });
    return res.json() as Promise<StatusResponse>;
  },

  async getStatus() {
    const res = await fetch(`${API_BASE}/control/status`);
    return res.json() as Promise<StatusResponse>;
  },

  // World endpoints
  async getAllCells() {
    const res = await fetch(`${API_BASE}/world/all`);
    return res.json() as Promise<Cell[]>;
  },

  async getPresets() {
    const res = await fetch(`${API_BASE}/world/presets`);
    return res.json() as Promise<PresetsListResponse>;
  },

  async loadPreset(name: string, offsetX = 0, offsetY = 0) {
    const res = await fetch(`${API_BASE}/world/preset`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name, offset_x: offsetX, offset_y: offsetY }),
    });
    return res.json();
  },

  async setCell(x: number, y: number, alive: boolean) {
    const res = await fetch(`${API_BASE}/world/cell`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ x, y, alive }),
    });
    return res.json();
  },

  async clearWorld() {
    const res = await fetch(`${API_BASE}/world/clear`, { method: 'POST' });
    return res.json();
  },
};
