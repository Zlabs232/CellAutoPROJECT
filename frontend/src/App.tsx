import { useEffect, useRef, useState } from 'react';
  import { api } from './api';
  import type { StatusResponse, PresetInfo, Cell } from './types';
  import './App.css';

  function App() {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const [status, setStatus] = useState<StatusResponse | null>(null);
    const [presets, setPresets] = useState<PresetInfo[]>([]);
    const [cells, setCells] = useState<Cell[]>([]);
    const [loading, setLoading] = useState(true);

    
    const [scale, setScale] = useState(20);
    const [offsetX, setOffsetX] = useState(0);
    const [offsetY, setOffsetY] = useState(0);
    const [isDragging, setIsDragging] = useState(false);
    const [dragStart, setDragStart] = useState({ x: 0, y: 0 });

    
    useEffect(() => {
      const init = async () => {
        try {
          const [statusData, presetsData] = await Promise.all([
            api.getStatus(),
            api.getPresets(),
          ]);
          setStatus(statusData);
          setPresets(presetsData.presets);
          setLoading(false);
        } catch (error) {
          console.error('Failed to initialize:', error);
          setLoading(false);
        }
      };
      init();
    }, []);

    
    useEffect(() => {
      const interval = setInterval(async () => {
        try {
          const [statusData, cellsData] = await Promise.all([
            api.getStatus(),
            api.getAllCells(),
          ]);
          setStatus(statusData);
          setCells(cellsData);
        } catch (error) {
          console.error('Failed to update:', error);
        }
      }, 100);

      return () => clearInterval(interval);
    }, []);

    
    useEffect(() => {
      const canvas = canvasRef.current;
      if (!canvas) return;

      const ctx = canvas.getContext('2d');
      if (!ctx) return;

      
      canvas.width = canvas.offsetWidth;
      canvas.height = canvas.offsetHeight;

      
      ctx.fillStyle = '#0a0e27';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      
      ctx.strokeStyle = '#1e2139';
      ctx.lineWidth = 1;

      const startX = Math.floor(-offsetX / scale) - 1;
      const endX = Math.ceil((canvas.width - offsetX) / scale) + 1;
      const startY = Math.floor(-offsetY / scale) - 1;
      const endY = Math.ceil((canvas.height - offsetY) / scale) + 1;

      for (let x = startX; x <= endX; x++) {
        const screenX = x * scale + offsetX;
        ctx.beginPath();
        ctx.moveTo(screenX, 0);
        ctx.lineTo(screenX, canvas.height);
        ctx.stroke();
      }

      for (let y = startY; y <= endY; y++) {
        const screenY = y * scale + offsetY;
        ctx.beginPath();
        ctx.moveTo(0, screenY);
        ctx.lineTo(canvas.width, screenY);
        ctx.stroke();
      }

      
      ctx.fillStyle = '#a78bfa';
      ctx.shadowColor = '#a78bfa';
      ctx.shadowBlur = 10;

      for (const cell of cells) {
        const x = cell.x * scale + offsetX;
        const y = cell.y * scale + offsetY;

        if (x > -scale && x < canvas.width && y > -scale && y < canvas.height) {
          ctx.fillRect(x + 1, y + 1, scale - 2, scale - 2);
        }
      }

      ctx.shadowBlur = 0;
    }, [cells, scale, offsetX, offsetY]);

    
    const handleMouseDown = (e: React.MouseEvent) => {
      setIsDragging(true);
      setDragStart({ x: e.clientX - offsetX, y: e.clientY - offsetY });
    };

    const handleMouseMove = (e: React.MouseEvent) => {
      if (isDragging) {
        setOffsetX(e.clientX - dragStart.x);
        setOffsetY(e.clientY - dragStart.y);
      }
    };

    const handleMouseUp = () => {
      setIsDragging(false);
    };

    const handleWheel = (e: React.WheelEvent) => {
      e.preventDefault();
      const delta = e.deltaY > 0 ? 0.9 : 1.1;
      setScale(prev => Math.max(5, Math.min(50, prev * delta)));
    };

    
    const handleStart = async () => {
      try {
        const data = await api.start();
        setStatus(data);
      } catch (error) {
        console.error('Failed to start:', error);
      }
    };

    const handleStop = async () => {
      try {
        const data = await api.stop();
        setStatus(data);
      } catch (error) {
        console.error('Failed to stop:', error);
      }
    };

    const handlePause = async () => {
      try {
        const data = await api.pause();
        setStatus(data);
      } catch (error) {
        console.error('Failed to pause:', error);
      }
    };

    const handleResume = async () => {
      try {
        const data = await api.resume();
        setStatus(data);
      } catch (error) {
        console.error('Failed to resume:', error);
      }
    };

    const handleStep = async () => {
      try {
        const data = await api.step();
        setStatus(data);
      } catch (error) {
        console.error('Failed to step:', error);
      }
    };

    const handleSpeedChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
      const tps = parseInt(e.target.value);
      try {
        const data = await api.setSpeed(tps);
        setStatus(data);
      } catch (error) {
        console.error('Failed to set speed:', error);
      }
    };

    const handleLoadPreset = async (name: string) => {
      try {
        await api.loadPreset(name, 0, 0);
        const cellsData = await api.getAllCells();
        setCells(cellsData);
      } catch (error) {
        console.error('Failed to load preset:', error);
      }
    };

    const handleClear = async () => {
      try {
        await api.clearWorld();
        setCells([]);
      } catch (error) {
        console.error('Failed to clear:', error);
      }
    };

    if (loading) {
      return (
        <div className="loading">
          <div className="spinner"></div>
          <p>Loading Cellular Automata...</p>
        </div>
      );
    }

    const isRunning = status?.state === 'running';
    const isPaused = status?.state === 'paused';
    const isStopped = status?.state === 'stopped';

    return (
      <div className="app">
        {/* Header */}
        <header className="header">
          <div className="header-content">
            <div className="logo">
              <div className="logo-icon">🎮</div>
              <div className="logo-text">
                <h1>Cellular Automata</h1>
                <p>Conway's Game of Life</p>
              </div>
            </div>

            <div className="stats">
              <div className="stat-item">
                <div className="stat-value">{status?.tick_count || 0}</div>
                <div className="stat-label">Ticks</div>
              </div>
              <div className="stat-item">
                <div className="stat-value">{status?.active_cells || 0}</div>
                <div className="stat-label">Cells</div>
              </div>
              <div className="stat-item">
                <div className="stat-value">{status?.tps || 0}</div>
                <div className="stat-label">TPS</div>
              </div>
            </div>
          </div>
        </header>

        {/* Main Content */}
        <div className="main-content">
          {/* Canvas */}
          <div className="canvas-container">
            <canvas
              ref={canvasRef}
              className="canvas"
              onMouseDown={handleMouseDown}
              onMouseMove={handleMouseMove}
              onMouseUp={handleMouseUp}
              onMouseLeave={handleMouseUp}
              onWheel={handleWheel}
            />
            <div className="canvas-overlay">
              Zoom: {scale.toFixed(1)}x | Use mouse wheel to zoom | Drag to pan
            </div>
          </div>

          {/* Sidebar */}
          <aside className="sidebar">
            {/* Status */}
            <div className="control-section">
              <div className="section-title">📊 Status</div>
              <div className={`status-badge ${status?.state || 'stopped'}`}>
                <div className="status-indicator"></div>
                {status?.state?.toUpperCase() || 'STOPPED'}
              </div>
            </div>

            {/* Controls */}
            <div className="control-section">
              <div className="section-title">🎮 Controls</div>
              <div className="button-group">
                {isStopped && (
                  <button onClick={handleStart} className="btn btn-success">
                    ▶️ Start
                  </button>
                )}
                {isRunning && (
                  <button onClick={handlePause} className="btn btn-warning">
                    ⏸️ Pause
                  </button>
                )}
                {isPaused && (
                  <button onClick={handleResume} className="btn btn-success">
                    ▶️ Resume
                  </button>
                )}
                <button onClick={handleStop} className="btn btn-danger">
                  ⏹️ Stop
                </button>
                <button onClick={handleStep} className="btn btn-secondary">
                  ⏭️ Step
                </button>
                <button onClick={handleClear} className="btn btn-secondary">
                  🧹  Clear
                </button>
              </div>
            </div>

            {/* Speed Control */}
            <div className="control-section">
              <div className="section-title">⚡ Speed</div>
              <div className="slider-control">
                <div className="slider-label">
                  <span>TPS (Ticks Per Second)</span>
                  <span className="slider-value">{status?.tps || 10}</span>
                </div>
                <input
                  type="range"
                  min="1"
                  max="100"
                  value={status?.tps || 10}
                  onChange={handleSpeedChange}
                  className="slider"
                />
              </div>
            </div>

            {/* Presets */}
            <div className="control-section">
              <div className="section-title">📦 Presets</div>
              <div className="preset-grid">
                {presets.map((preset) => (
                  <div
                    key={preset.name}
                    className="preset-card"
                    onClick={() => handleLoadPreset(preset.name)}
                  >
                    <div className="preset-name">
                      {preset.name}
                    </div>
                    <div className="preset-description">
                      {preset.description}
                    </div>
                    <div className="preset-cells">
                      {preset.cell_count} cells
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </aside>
        </div>
      </div>
    );
  }

  export default App;