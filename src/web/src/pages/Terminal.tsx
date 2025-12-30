/**
 * Terminal page - Web-based terminal emulator
 */

import { useEffect, useRef, useState } from 'react';
import { Card, Button, Space, Select, Tooltip, message } from 'antd';
import {
  ClearOutlined,
  DownloadOutlined,
  FullscreenOutlined,
  FullscreenExitOutlined,
  ReloadOutlined,
} from '@ant-design/icons';
import { useTranslation } from 'react-i18next';
import { Terminal as XTerm } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import { ws } from '../services/websocket';
import { WS_CHANNELS } from '../utils/constants';
import 'xterm/css/xterm.css';

const { Option } = Select;

export default function Terminal() {
  const { t } = useTranslation();
  const terminalRef = useRef<HTMLDivElement>(null);
  const xtermRef = useRef<XTerm | null>(null);
  const fitAddonRef = useRef<FitAddon | null>(null);
  const [fullscreen, setFullscreen] = useState(false);
  const [fontSize, setFontSize] = useState(14);
  const [theme, setTheme] = useState<'dark' | 'light'>('dark');
  const inputBufferRef = useRef<string>('');

  useEffect(() => {
    if (!terminalRef.current) return;

    // Initialize xterm
    const term = new XTerm({
      cursorBlink: true,
      fontSize,
      fontFamily: 'Monaco, Menlo, "Ubuntu Mono", monospace',
      theme: {
        background: theme === 'dark' ? '#1e1e1e' : '#ffffff',
        foreground: theme === 'dark' ? '#d4d4d4' : '#000000',
        cursor: '#00ff00',
        selection: 'rgba(255, 255, 255, 0.3)',
      },
      cols: 80,
      rows: 24,
    });

    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(terminalRef.current);
    fitAddon.fit();

    xtermRef.current = term;
    fitAddonRef.current = fitAddon;

    // Welcome message
    term.writeln('\x1b[1;32mNeuroGraph Terminal\x1b[0m');
    term.writeln('Type commands and press Enter to execute.\n');
    term.write('$ ');

    // Handle user input
    term.onData((data) => {
      const code = data.charCodeAt(0);

      if (code === 13) {
        // Enter
        term.write('\r\n');
        const command = inputBufferRef.current.trim();
        if (command) {
          sendCommand(command);
        }
        inputBufferRef.current = '';
        term.write('$ ');
      } else if (code === 127) {
        // Backspace
        if (inputBufferRef.current.length > 0) {
          inputBufferRef.current = inputBufferRef.current.slice(0, -1);
          term.write('\b \b');
        }
      } else if (code >= 32) {
        // Printable character
        inputBufferRef.current += data;
        term.write(data);
      }
    });

    // Subscribe to terminal WebSocket
    ws.subscribe(WS_CHANNELS.TERMINAL, handleTerminalMessage);

    // Handle window resize
    const handleResize = () => {
      fitAddon.fit();
    };
    window.addEventListener('resize', handleResize);

    return () => {
      ws.unsubscribe(WS_CHANNELS.TERMINAL, handleTerminalMessage);
      window.removeEventListener('resize', handleResize);
      term.dispose();
    };
  }, [fontSize, theme]);

  const handleTerminalMessage = (data: any) => {
    if (!xtermRef.current) return;

    if (data.type === 'output') {
      xtermRef.current.write(data.content);
    } else if (data.type === 'error') {
      xtermRef.current.write(`\x1b[1;31m${data.content}\x1b[0m`);
    } else if (data.type === 'complete') {
      xtermRef.current.write('\r\n$ ');
    }
  };

  const sendCommand = (command: string) => {
    // Send command via WebSocket
    ws.send({
      channel: WS_CHANNELS.TERMINAL,
      data: { command },
    });
  };

  const handleClear = () => {
    if (xtermRef.current) {
      xtermRef.current.clear();
      xtermRef.current.write('$ ');
      inputBufferRef.current = '';
    }
  };

  const handleReset = () => {
    if (xtermRef.current) {
      xtermRef.current.reset();
      xtermRef.current.writeln('\x1b[1;32mNeuroGraph Terminal\x1b[0m');
      xtermRef.current.writeln('Type commands and press Enter to execute.\n');
      xtermRef.current.write('$ ');
      inputBufferRef.current = '';
    }
  };

  const handleDownload = () => {
    if (!xtermRef.current) return;

    // Get terminal buffer content
    const buffer = xtermRef.current.buffer.active;
    let content = '';
    for (let i = 0; i < buffer.length; i++) {
      const line = buffer.getLine(i);
      if (line) {
        content += line.translateToString(true) + '\n';
      }
    }

    // Download as text file
    const blob = new Blob([content], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `terminal-${Date.now()}.txt`;
    a.click();
    URL.revokeObjectURL(url);

    message.success(t('terminal.downloadSuccess'));
  };

  const handleFullscreen = () => {
    setFullscreen(!fullscreen);
    setTimeout(() => {
      if (fitAddonRef.current) {
        fitAddonRef.current.fit();
      }
    }, 100);
  };

  const handleFontSizeChange = (value: number) => {
    setFontSize(value);
  };

  const handleThemeChange = (value: 'dark' | 'light') => {
    setTheme(value);
  };

  return (
    <div
      style={{
        padding: fullscreen ? 0 : 24,
        height: fullscreen ? '100vh' : 'calc(100vh - 120px)',
        display: 'flex',
        flexDirection: 'column',
      }}
    >
      {/* Header */}
      {!fullscreen && (
        <div
          style={{
            marginBottom: 16,
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
          }}
        >
          <h1 style={{ margin: 0 }}>{t('terminal.title')}</h1>
          <Space>
            <Select value={fontSize} onChange={handleFontSizeChange} style={{ width: 80 }}>
              <Option value={12}>12px</Option>
              <Option value={14}>14px</Option>
              <Option value={16}>16px</Option>
              <Option value={18}>18px</Option>
              <Option value={20}>20px</Option>
            </Select>
            <Select value={theme} onChange={handleThemeChange} style={{ width: 100 }}>
              <Option value="dark">{t('terminal.themeDark')}</Option>
              <Option value="light">{t('terminal.themeLight')}</Option>
            </Select>
            <Tooltip title={t('terminal.clear')}>
              <Button icon={<ClearOutlined />} onClick={handleClear} />
            </Tooltip>
            <Tooltip title={t('terminal.reset')}>
              <Button icon={<ReloadOutlined />} onClick={handleReset} />
            </Tooltip>
            <Tooltip title={t('terminal.download')}>
              <Button icon={<DownloadOutlined />} onClick={handleDownload} />
            </Tooltip>
            <Tooltip title={fullscreen ? t('terminal.exitFullscreen') : t('terminal.fullscreen')}>
              <Button
                icon={fullscreen ? <FullscreenExitOutlined /> : <FullscreenOutlined />}
                onClick={handleFullscreen}
              />
            </Tooltip>
          </Space>
        </div>
      )}

      {/* Terminal */}
      <Card
        style={{
          flex: 1,
          overflow: 'hidden',
          display: 'flex',
          flexDirection: 'column',
        }}
        bodyStyle={{
          flex: 1,
          overflow: 'hidden',
          padding: 0,
          background: theme === 'dark' ? '#1e1e1e' : '#ffffff',
        }}
      >
        <div
          ref={terminalRef}
          style={{
            height: '100%',
            padding: 8,
          }}
        />
      </Card>
    </div>
  );
}
