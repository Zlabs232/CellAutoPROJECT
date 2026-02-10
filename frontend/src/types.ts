export interface Cell {
  x: number;
  y: number;
}

export interface StatusResponse {
  state: string;
  tick_count: number;
  tps: number;
  active_cells: number;
}

export interface PresetInfo {
  name: string;
  description: string;
  cell_count: number;
}

export interface PresetsListResponse {
  presets: PresetInfo[];
}
