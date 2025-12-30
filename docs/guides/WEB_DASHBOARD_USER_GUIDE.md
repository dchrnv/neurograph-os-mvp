# NeuroGraph Web Dashboard - User Guide

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Dashboard Overview](#dashboard-overview)
4. [Module Management](#module-management)
5. [System Configuration](#system-configuration)
6. [Bootstrap Process](#bootstrap-process)
7. [Chat Interface](#chat-interface)
8. [Terminal Emulator](#terminal-emulator)
9. [Settings & Preferences](#settings--preferences)
10. [Troubleshooting](#troubleshooting)
11. [Keyboard Shortcuts](#keyboard-shortcuts)
12. [FAQ](#faq)

---

## Introduction

The NeuroGraph Web Dashboard is a modern, React-based single-page application (SPA) that provides real-time monitoring and management of the NeuroGraph OS cognitive architecture system. Built with TypeScript, Ant Design Pro, and WebSocket technology, it offers an intuitive interface for system administrators, developers, and researchers.

### Key Features

- **Real-time Monitoring** - Live system metrics and activity logs
- **Module Management** - Start, stop, configure system modules
- **Configuration Editor** - Adjust system and CDNA settings
- **AI Chat Interface** - Interactive chat with the cognitive system
- **Web Terminal** - Command-line interface in your browser
- **Multi-language Support** - English and Russian interfaces
- **Dark/Light Themes** - Customizable visual appearance
- **Responsive Design** - Works on desktop, tablet, and mobile devices

### System Requirements

**Browser Support:**
- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Mobile Safari (iOS 14+)
- Chrome Mobile (Android 90+)

**Network Requirements:**
- HTTP connection to NeuroGraph backend
- WebSocket support for real-time updates
- Recommended: 1 Mbps or faster internet connection

---

## Getting Started

### Installation

1. **Install Node.js and npm**

   On Arch Linux:
   ```bash
   sudo pacman -S nodejs npm
   ```

   On Ubuntu/Debian:
   ```bash
   sudo apt install nodejs npm
   ```

   On macOS:
   ```bash
   brew install node
   ```

2. **Navigate to Web Dashboard Directory**

   ```bash
   cd neurograph-os-mvp/src/web
   ```

3. **Install Dependencies**

   ```bash
   npm install
   ```

   This will install all required packages (~450 MB).

4. **Configure Backend URL** (Optional)

   Create `.env` file:
   ```bash
   VITE_API_URL=http://localhost:8000/api/v1
   VITE_WS_URL=ws://localhost:8000/ws
   ```

5. **Start Development Server**

   ```bash
   npm run dev
   ```

   The dashboard will be available at: **http://localhost:5173**

### First Launch

When you first open the dashboard:

1. **Language Selection** - Choose your preferred language (EN/RU)
2. **Theme Selection** - Choose Dark or Light theme
3. **Connection Status** - Check the WebSocket connection indicator
4. **System Status** - Verify the backend is running and reachable

### Navigation

The main navigation sidebar provides access to all features:

| Icon | Page | Description |
|------|------|-------------|
| 📊 | **Dashboard** | System overview and metrics |
| 🧩 | **Modules** | Module management interface |
| ⚙️ | **Config** | Configuration editor |
| 🚀 | **Bootstrap** | System initialization |
| 💬 | **Chat** | AI chat interface |
| 💻 | **Terminal** | Web-based terminal |
| 👤 | **Admin** | Admin tools (coming soon) |

---

## Dashboard Overview

The Dashboard is the main monitoring interface, providing real-time insights into system performance.

### Main Metrics

**Top Row - Key Performance Indicators:**

1. **Total Tokens**
   - Shows total number of tokens processed
   - Trend indicator (↑ green = increasing, ↓ red = decreasing)
   - Updates in real-time via WebSocket

2. **Active Connections**
   - Number of active client connections
   - Color changes based on load (green = low, yellow = medium, red = high)
   - Critical for monitoring system capacity

3. **Queries/Hour**
   - Query processing rate
   - Progress bar shows utilization
   - Helps identify peak usage times

4. **Events/sec**
   - Event throughput metric
   - Real-time updates
   - Indicator of system activity level

### Performance Metrics

**6 Detailed Metrics:**

1. **Avg Latency** (microseconds)
   - Average response time for queries
   - Lower is better (<100μs is excellent)
   - Helps identify performance bottlenecks

2. **Fast Path %**
   - Percentage of queries using optimized fast path
   - Higher is better (>80% is good)
   - Indicates cache efficiency

3. **Cache Hit %**
   - Cache hit rate percentage
   - Higher is better (>90% is excellent)
   - Key performance indicator

4. **CPU Usage**
   - CPU utilization percentage
   - Color-coded progress bar
   - Alert if >80% sustained

5. **Memory Usage**
   - Memory utilization percentage
   - Important for stability monitoring
   - Consider scaling if >85%

6. **Disk Usage**
   - Disk space utilization
   - Critical for data storage
   - Clean up logs if >90%

### Activity Log

The activity table shows recent system events:

| Column | Description |
|--------|-------------|
| **Time** | Event timestamp (HH:MM:SS format) |
| **Event** | Event type (Query, Connection, Error, etc.) |
| **Details** | Event description and context |
| **Duration** | Processing time (if applicable) |

**Features:**
- Auto-updates via WebSocket
- Shows last 50 events
- Color-coded event types
- Sortable columns
- Search/filter functionality

### Using the Dashboard

**Real-time Monitoring:**
- Metrics update automatically every 1-30 seconds
- WebSocket provides instant updates when available
- Refresh button to manually update data

**Identifying Issues:**
1. **High CPU/Memory** - Check module status, consider restarting
2. **Low Cache Hit Rate** - Review query patterns, tune cache
3. **High Latency** - Check database performance, network
4. **Error Events** - Click to view details in activity log

**Best Practices:**
- Monitor dashboard during peak hours
- Set up alerts for critical metrics (future feature)
- Review activity log daily for errors
- Export metrics for analysis (future feature)

---

## Module Management

The Modules page provides comprehensive control over system modules.

### Module List

Each module is displayed as a card with:

- **Module Name** - Identifier and display name
- **Status Badge** - Visual status indicator
  - 🟢 **Running** - Module is active
  - 🟡 **Starting** - Module is initializing
  - ⚫ **Stopped** - Module is not running
  - 🔴 **Error** - Module encountered an error
  - 🔵 **Restarting** - Module is restarting

- **Version** - Module version number
- **Description** - Brief module description
- **Action Buttons** - Control buttons (see below)

### Module Actions

**5 Primary Actions:**

1. **Start** ▶️
   - Starts a stopped module
   - Disabled if module is already running
   - Shows loading indicator during startup
   - Displays success/error notification

2. **Stop** ⏹️
   - Stops a running module gracefully
   - Confirmation dialog before stopping
   - Waits for clean shutdown
   - Error handling for forced stop

3. **Restart** 🔄
   - Stops and restarts the module
   - Useful for applying configuration changes
   - Maintains module state where possible
   - Automatic retry on failure

4. **View Logs** 📄
   - Opens logs drawer
   - Shows real-time log stream
   - Filterable by log level (INFO, WARN, ERROR)
   - Download logs option

5. **Configure** ⚙️
   - Opens module configuration editor
   - JSON editor with syntax highlighting
   - Validation before saving
   - Reset to defaults option

### Module Details Drawer

Click **View Details** to open the comprehensive module drawer with 4 tabs:

#### 1. Info Tab

- **Module ID** - Unique identifier
- **Name & Version** - Display information
- **Status** - Current state with health indicator
- **Description** - Detailed module description
- **Dependencies** - List of required modules
- **Configuration Summary** - Key settings overview

#### 2. Metrics Tab

Real-time module metrics:
- **CPU Usage Graph** - Last 60 seconds
- **Memory Usage Graph** - Memory consumption over time
- **Request Rate** - Requests per second
- **Error Rate** - Errors per minute
- **Custom Metrics** - Module-specific metrics

**Features:**
- Auto-updating charts
- Configurable time range (1m, 5m, 1h, 24h)
- Export to CSV/PNG
- Zoom and pan controls

#### 3. Configuration Tab

Module configuration editor:
```json
{
  "module_id": "semantic_memory",
  "enabled": true,
  "config": {
    "max_connections": 100,
    "cache_ttl": 3600,
    "log_level": "INFO"
  }
}
```

**Features:**
- JSON syntax highlighting
- Schema validation
- Auto-save draft (localStorage)
- Revert to saved
- Reset to defaults
- Import/Export configuration

#### 4. Logs Tab

Real-time log viewer:
- **Log Levels:**
  - 🔵 INFO - Normal operation logs
  - 🟡 WARN - Warning messages
  - 🔴 ERROR - Error messages
  - 🟣 DEBUG - Debug information

- **Features:**
  - Filter by log level
  - Search logs by keyword
  - Auto-scroll to bottom
  - Download logs as .txt
  - Clear logs button
  - Timestamp display

### Bulk Operations

Select multiple modules to perform bulk actions:

1. **Select Modules** - Click checkboxes on module cards
2. **Choose Action** - Use bulk action buttons
   - Start All Selected
   - Stop All Selected
   - Restart All Selected
   - Export Configuration
3. **Confirm** - Review and confirm bulk operation
4. **Monitor** - Watch progress in notification area

### Filtering & Search

**Filter Options:**
- **Status Filter** - Show only Running, Stopped, Error, or All
- **Search** - Search by module name or ID
- **Sort** - By name, status, or last updated

**Example Use Cases:**
- Find all stopped modules: Filter by "Stopped"
- Search for specific module: Type in search box
- Review error modules: Filter by "Error" status

### Best Practices

1. **Regular Monitoring** - Check module status daily
2. **Graceful Shutdowns** - Always use Stop before Restart
3. **Configuration Backups** - Export configs before changes
4. **Log Review** - Check logs after errors
5. **Restart After Config** - Restart modules after configuration changes
6. **Dependencies** - Start dependency modules first

---

## System Configuration

The Configuration page allows you to adjust system-wide settings and CDNA cognitive dimensions.

### System Settings Tab

**JSON Configuration Editor:**

```json
{
  "api": {
    "base_url": "http://localhost:8000",
    "timeout": 30000,
    "max_retries": 3
  },
  "websocket": {
    "url": "ws://localhost:8000/ws",
    "reconnect_delay": 3000,
    "heartbeat_interval": 30000
  },
  "logging": {
    "level": "INFO",
    "max_file_size": 104857600,
    "max_files": 10
  },
  "system": {
    "max_connections": 100,
    "enable_cache": true,
    "cache_ttl": 3600,
    "enable_metrics": true
  }
}
```

**Configuration Sections:**

1. **API Settings**
   - `base_url` - Backend API URL
   - `timeout` - Request timeout (ms)
   - `max_retries` - Retry attempts on failure

2. **WebSocket Settings**
   - `url` - WebSocket server URL
   - `reconnect_delay` - Reconnection delay (ms)
   - `heartbeat_interval` - Keep-alive interval (ms)

3. **Logging Settings**
   - `level` - Log level (DEBUG, INFO, WARN, ERROR)
   - `max_file_size` - Max log file size (bytes)
   - `max_files` - Number of log files to retain

4. **System Settings**
   - `max_connections` - Maximum concurrent connections
   - `enable_cache` - Enable caching
   - `cache_ttl` - Cache time-to-live (seconds)
   - `enable_metrics` - Enable metrics collection

**Editor Features:**
- Syntax highlighting
- Auto-completion
- Schema validation
- Format on save
- Error indicators
- Undo/Redo (Ctrl+Z / Ctrl+Y)

### CDNA Scales Tab

**Cognitive Dimension Network Architecture (CDNA) Parameters:**

These 8 dimensions control the cognitive behavior of the NeuroGraph system.

#### 1. Sensitivity (0.0 - 1.0)

**Description:** How sensitive the system is to input changes

- **Low (0.0-0.3):** Ignores small variations, stable responses
- **Medium (0.4-0.7):** Balanced sensitivity
- **High (0.8-1.0):** Highly responsive to subtle changes

**Use Cases:**
- Low: Production systems requiring stability
- High: Research systems exploring edge cases

#### 2. Plasticity (0.0 - 1.0)

**Description:** Ability to adapt and learn from experience

- **Low:** Minimal learning, relies on pre-trained knowledge
- **High:** Rapid adaptation to new patterns

**Use Cases:**
- Low: Static knowledge bases
- High: Adaptive systems in changing environments

#### 3. Stability (0.0 - 1.0)

**Description:** Resistance to random fluctuations

- **Low:** More stochastic, varied responses
- **High:** Deterministic, consistent outputs

**Use Cases:**
- Low: Creative applications
- High: Production systems requiring reliability

#### 4. Integration (0.0 - 1.0)

**Description:** Degree of component interconnection

- **Low:** Independent modules, loose coupling
- **High:** Highly interconnected, emergent behavior

**Use Cases:**
- Low: Modular systems
- High: Holistic cognitive processing

#### 5. Differentiation (0.0 - 1.0)

**Description:** Specialization of subsystems

- **Low:** Generalized modules
- **High:** Highly specialized components

**Use Cases:**
- Low: General-purpose systems
- High: Domain-specific applications

#### 6. Phase Transition (0.0 - 1.0)

**Description:** Ability to shift between cognitive states

- **Low:** Gradual state changes
- **High:** Rapid mode switching

**Use Cases:**
- Low: Stable operation modes
- High: Dynamic multi-mode systems

#### 7. Criticality (0.0 - 1.0)

**Description:** Operating point between order and chaos

- **Low:** Ordered, predictable behavior
- **Medium (0.5):** Critical point, optimal complexity
- **High:** Chaotic, unpredictable behavior

**Recommended:** 0.5 for optimal performance

#### 8. Meta-awareness (0.0 - 1.0)

**Description:** Self-monitoring and introspection capability

- **Low:** No self-reflection
- **High:** Extensive introspection and self-modification

**Use Cases:**
- Low: Fast, unconscious processing
- High: Deliberate reasoning and planning

### Adjusting CDNA Scales

**Step-by-Step:**

1. **Open CDNA Scales Tab**
2. **Review Current Values** - See current scale settings
3. **Adjust Sliders** - Drag sliders to desired values
4. **Preview Changes** - See real-time value updates
5. **Save Changes** - Click "Save Changes" button
6. **Confirm** - Confirm configuration update
7. **Monitor Effects** - Observe system behavior changes

**Visual Feedback:**
- Slider color changes based on value (blue → green → yellow → red)
- Percentage display next to each slider
- Description tooltips on hover

### Configuration Best Practices

1. **Backup Before Changes** - Export config before modifications
2. **Incremental Changes** - Adjust one parameter at a time
3. **Monitor Impact** - Watch metrics after changes
4. **Document Changes** - Note what you changed and why
5. **Revert if Needed** - Use "Reset to Defaults" if issues arise
6. **Test Thoroughly** - Validate in dev before production

### Saving Configuration

**Two Save Options:**

1. **Save Changes** - Saves to backend immediately
   - Updates running system
   - Persisted to configuration file
   - Notification on success/failure

2. **Auto-save Draft** - Saves to browser localStorage
   - Automatic backup of unsaved changes
   - Restored if you navigate away
   - Cleared after successful save

**Reset Options:**

- **Reset to Defaults** - Restore factory settings
- **Revert Changes** - Undo unsaved modifications
- **Load from File** - Import configuration JSON

---

## Bootstrap Process

The Bootstrap page guides you through system initialization.

### Bootstrap Steps

The bootstrap process consists of 6 sequential steps:

#### Step 1: Initialize Core Services (≈5 seconds)

**Actions:**
- Start message queue service
- Initialize database connections
- Load configuration files
- Set up error handling

**Success Criteria:**
- All services respond to health checks
- Configuration loaded without errors
- Message queue is operational

**Common Errors:**
- Database connection refused → Check database is running
- Configuration file not found → Verify file paths

#### Step 2: Load CDNA Model (≈8 seconds)

**Actions:**
- Load neural network weights from disk
- Initialize cognitive dimension parameters
- Validate model integrity (checksums)
- Allocate GPU/CPU memory

**Success Criteria:**
- Model weights loaded successfully
- Memory allocated within limits
- Model passes validation checks

**Common Errors:**
- Out of memory → Reduce model size or free RAM
- Corrupted weights → Re-download model files

#### Step 3: Start Module Manager (≈4 seconds)

**Actions:**
- Discover available modules in module directory
- Load module metadata and dependencies
- Initialize module registry
- Start module supervisor process

**Success Criteria:**
- All modules discovered and registered
- No dependency conflicts
- Supervisor process running

**Common Errors:**
- Module not found → Check module directory
- Dependency conflict → Update module versions

#### Step 4: Initialize Graph Database (≈6 seconds)

**Actions:**
- Connect to Neo4j/ArangoDB instance
- Create necessary indexes
- Load existing graph data
- Validate graph schema

**Success Criteria:**
- Database connection established
- Indexes created
- Schema validation passed

**Common Errors:**
- Connection timeout → Check database service
- Schema mismatch → Run migration scripts

#### Step 5: Start WebSocket Server (≈3 seconds)

**Actions:**
- Bind WebSocket server to configured port
- Initialize channel routing
- Start heartbeat monitor
- Run connection test

**Success Criteria:**
- Server listening on port
- Test connection successful
- Heartbeat active

**Common Errors:**
- Port already in use → Kill existing process or change port
- Firewall blocking → Allow port in firewall

#### Step 6: Finalize Bootstrap (≈2 seconds)

**Actions:**
- Run comprehensive health checks
- Register all system services
- Mark system as ready
- Emit ready event to all modules

**Success Criteria:**
- All health checks pass
- System status: READY
- All modules notified

### Using the Bootstrap Page

**Starting Bootstrap:**

1. **Click "Start Bootstrap"** - Initiates the process
2. **Watch Progress** - Follow the step-by-step progress
3. **Read Logs** - Monitor logs for detailed information
4. **Wait for Completion** - Allow all steps to finish

**Progress Indicators:**

- **Steps Component** - Visual step-by-step progress
  - Current step highlighted in blue
  - Completed steps shown in green
  - Failed steps shown in red

- **Progress Bar** - Overall completion percentage
  - Animates smoothly from 0% to 100%
  - Color changes: blue → green (success) or red (error)

- **Log Output** - Real-time logs
  - Timestamped entries
  - Color-coded by level (INFO, WARN, ERROR)
  - Auto-scrolls to latest entry

**Error Handling:**

If a step fails:
1. **Read Error Message** - Check logs for details
2. **Fix Issue** - Address the root cause
3. **Click "Retry"** - Retry the failed step
4. **Or "Start Over"** - Restart from beginning

**Success:**

When bootstrap completes successfully:
- All steps marked green (✓)
- Success message displayed
- System ready for use
- Navigate to Dashboard to verify

### Bootstrap Scenarios

**Scenario 1: Fresh Installation**
- All steps will run for the first time
- Expect longer initialization times
- May require additional setup (database creation, etc.)

**Scenario 2: Restart After Shutdown**
- Faster bootstrap (cached data)
- Skips some initialization steps
- Reconnects to existing services

**Scenario 3: Configuration Changes**
- May require full bootstrap
- Some steps may be skipped
- Watch logs for specific actions

**Scenario 4: Error Recovery**
- Identify failed step
- Fix underlying issue
- Retry from failed step
- Continue to completion

### Troubleshooting Bootstrap

| Error | Possible Cause | Solution |
|-------|----------------|----------|
| Database connection failed | DB not running | Start database service |
| Port already in use | Previous instance running | Kill process or change port |
| Out of memory | Insufficient RAM | Close other applications or add RAM |
| Model file not found | Missing model files | Download model files |
| Permission denied | Insufficient permissions | Run with appropriate permissions |

---

## Chat Interface

The Chat page provides an interactive interface for conversing with the NeuroGraph cognitive system.

### Chat Features

**Multi-Session Support:**
- Create unlimited chat sessions
- Switch between sessions instantly
- Each session maintains separate conversation history
- Sessions persist across browser restarts (localStorage)

**Message Types:**
- **User Messages** (You) - Blue bubbles, right-aligned
- **Assistant Messages** (AI) - Dark gray bubbles, left-aligned
- **System Messages** - Yellow bubbles, center-aligned (errors, notifications)

**Real-time Streaming:**
- Responses stream in real-time as they're generated
- Blinking cursor indicates active streaming
- Smooth text updates
- Interrupt streaming by closing session (future feature)

### Using the Chat Interface

#### Starting a New Chat

1. **Click "New Chat" Button** - Creates a fresh session
2. **Session Auto-named** - Named "Chat 1", "Chat 2", etc.
3. **Appears in Session List** - Accessible from Sessions drawer
4. **Auto-selected** - Becomes active session

#### Sending Messages

**Method 1: Enter Key**
1. Type your message in the input box
2. Press **Enter** to send
3. Wait for AI response

**Method 2: Send Button**
1. Type your message
2. Click **Send** button (→)
3. Wait for response

**Multi-line Messages:**
- Press **Shift+Enter** to add new line
- Compose longer messages
- Press **Enter** alone to send

#### Managing Sessions

**Opening Sessions Drawer:**
- Click **Sessions** button in toolbar
- Drawer opens from right side
- Shows all chat sessions

**Session List Display:**
- Session title (editable in future)
- Message count (e.g., "15 messages")
- Current session highlighted in blue
- Delete button (X) for each session

**Switching Sessions:**
1. Open Sessions drawer
2. Click on desired session
3. Drawer closes automatically
4. Messages load instantly

**Deleting Sessions:**
1. Open Sessions drawer
2. Click **X** button on session
3. Confirm deletion (future feature)
4. Session removed permanently

#### Clearing Messages

**Clear Current Conversation:**
1. Click **Clear** button in toolbar
2. All messages in current session deleted
3. Session remains (empty)
4. Start fresh conversation

**Note:** This action cannot be undone.

### Message Formatting

**Supported Formatting:**
- **Line breaks** - Preserved from input
- **Whitespace** - Maintained in display
- **Long text** - Auto-wraps within bubble
- **Code blocks** - Monospace font (future: syntax highlighting)
- **Links** - Clickable URLs (future: auto-detection)

**Current Limitations:**
- No markdown rendering (planned for v0.63)
- No image attachments (planned for v0.64)
- No file uploads (planned for v0.64)

### Chat Commands

**Special Commands (Future Feature):**
```
/help          - Show available commands
/clear         - Clear current conversation
/export        - Export chat to file
/summarize     - Summarize conversation
/context       - Show conversation context
```

### Example Conversations

**Example 1: System Status Query**
```
You: What is the current system status?

AI: The NeuroGraph system is currently running normally.
Here are the key metrics:
- Status: RUNNING
- Uptime: 3 days, 14 hours
- Active connections: 42
- CPU usage: 34.2%
- Memory usage: 56.8%
- All modules are operational.
```

**Example 2: Module Query**
```
You: Which modules are currently stopped?

AI: Currently, the following modules are stopped:
1. backup_service (v1.2.0)
2. analytics_engine (v2.0.1)

Would you like me to start these modules?

You: Yes, please start both modules.

AI: Starting modules...
✓ backup_service started successfully
✓ analytics_engine started successfully

Both modules are now running.
```

**Example 3: Configuration Assistance**
```
You: How do I increase the CDNA sensitivity parameter?

AI: To increase the CDNA sensitivity parameter:

1. Navigate to the Configuration page
2. Click on the "CDNA Scales" tab
3. Find the "Sensitivity" slider
4. Drag it to the right to increase (current: 0.5)
5. Click "Save Changes"

Note: Higher sensitivity (0.8-1.0) makes the system more
responsive to input changes but may reduce stability. I
recommend values between 0.6-0.8 for most use cases.
```

### Best Practices

1. **Be Specific** - Clear, specific questions get better answers
2. **One Question at a Time** - Avoid multi-part questions
3. **Provide Context** - Mention relevant module names, error messages
4. **Use Sessions** - Organize conversations by topic
5. **Review History** - Check previous messages for context
6. **Clear Old Sessions** - Delete unused sessions to save space

### Privacy & Data

- **Session Storage** - Sessions saved in browser localStorage
- **No Server Storage** - Chat history not saved on server (by default)
- **Clear Data** - Delete sessions to remove all traces
- **Secure Connection** - Use HTTPS for sensitive conversations
- **No Encryption** - Messages not encrypted in localStorage (use incognito for sensitive data)

---

## Terminal Emulator

The Terminal page provides a full-featured web-based terminal emulator for command-line interaction with the NeuroGraph system.

### Terminal Features

**Full Terminal Emulation:**
- xterm.js-based terminal
- ANSI color support (16 colors + RGB)
- Cursor blinking and positioning
- Text selection and copy/paste
- Auto-resize to fit container

**Customization:**
- Font size: 12px, 14px, 16px, 18px, 20px
- Theme: Dark (default) or Light
- Fullscreen mode
- Persistent settings (localStorage)

**Terminal Controls:**
- Clear terminal screen
- Reset to initial state
- Download terminal log
- Fullscreen toggle

### Using the Terminal

#### Opening the Terminal

1. **Navigate to Terminal page** - Click Terminal in sidebar
2. **Wait for Connection** - Terminal connects to backend automatically
3. **See Welcome Message** - Displays when ready
   ```
   NeuroGraph Terminal
   Type commands and press Enter to execute.

   $
   ```

#### Running Commands

**Basic Usage:**
1. Type command at `$` prompt
2. Press **Enter** to execute
3. See command output
4. New prompt appears when complete

**Example Commands:**
```bash
$ ls -la
total 48
drwxr-xr-x  12 user  staff   384 Dec 30 10:00 .
drwxr-xr-x   6 user  staff   192 Dec 29 15:30 ..
-rw-r--r--   1 user  staff  1024 Dec 30 09:00 config.json

$ ps aux | grep neurograph
user    1234  0.5  2.3  456789 98765 ?  Ss   09:00   0:45 neurograph-core

$ curl http://localhost:8000/api/v1/health
{"status": "running", "uptime": 308745, "version": "0.62.0"}
```

**Interactive Commands:**
- Most interactive commands supported
- Use `Ctrl+C` to interrupt (future feature)
- Use `Ctrl+D` to send EOF (future feature)

#### Editing Commands

**Current Support:**
- Type characters → Append to command line
- **Backspace** → Delete last character
- **Enter** → Execute command

**Future Support:**
- **Arrow Keys** → Navigate command history
- **Home/End** → Jump to line start/end
- **Ctrl+A/E** → Line start/end (Emacs bindings)
- **Ctrl+U** → Clear line
- **Tab** → Auto-completion

#### Customizing Appearance

**Font Size:**
1. Click font size dropdown (top-right)
2. Select size: 12px, 14px, 16px, 18px, or 20px
3. Terminal resizes immediately
4. Setting saved to localStorage

**Theme:**
1. Click theme dropdown
2. Select "Dark" or "Light"
3. Color scheme updates immediately

**Dark Theme:**
- Background: Dark gray (#1e1e1e)
- Text: Light gray (#d4d4d4)
- Cursor: Green (#00ff00)

**Light Theme:**
- Background: White (#ffffff)
- Text: Black (#000000)
- Cursor: Green (#00ff00)

#### Terminal Actions

**Clear Terminal:**
- Click **Clear** button (🗑️)
- Clears all text from screen
- Prompt reappears
- Does not clear command history

**Reset Terminal:**
- Click **Reset** button (🔄)
- Resets terminal to initial state
- Clears screen and command buffer
- Shows welcome message again

**Download Log:**
- Click **Download** button (💾)
- Downloads terminal output as .txt file
- Filename: `terminal-{timestamp}.txt`
- Includes all visible text

**Fullscreen Mode:**
- Click **Fullscreen** button (⛶)
- Terminal expands to fill entire window
- Hide all UI except terminal
- Click again (or press Esc) to exit

### Advanced Usage

#### Running System Commands

**Module Management:**
```bash
$ neurograph module list
semantic_memory    RUNNING    v1.0.0
graph_engine       RUNNING    v2.1.0
backup_service     STOPPED    v1.2.0

$ neurograph module start backup_service
Starting backup_service...
✓ Module started successfully

$ neurograph module status backup_service
Module: backup_service
Status: RUNNING
Uptime: 00:05:32
CPU: 2.3%
Memory: 45.2 MB
```

**Configuration:**
```bash
$ neurograph config get cdna.sensitivity
0.65

$ neurograph config set cdna.sensitivity 0.75
✓ Configuration updated

$ neurograph config validate
✓ Configuration is valid
```

**System Monitoring:**
```bash
$ neurograph metrics
Tokens: 125,432
Connections: 42
Queries/hour: 1,520
Events/sec: 245.5

$ neurograph health
Status: RUNNING
Uptime: 3d 14h 23m
Version: 0.62.0
All systems operational

$ neurograph logs --tail 10 --follow
[10:23:15] INFO: Processing query #45231
[10:23:15] INFO: Cache hit for query #45231
[10:23:16] INFO: Response sent in 12ms
...
```

#### Scripting

**Example Script:**
```bash
#!/bin/bash
# backup.sh - Backup NeuroGraph configuration

echo "Starting backup..."
neurograph config export > config_backup.json
neurograph module list > modules_backup.txt
tar -czf neurograph_backup_$(date +%Y%m%d).tar.gz \
    config_backup.json \
    modules_backup.txt
echo "Backup complete!"
```

**Run Script:**
```bash
$ chmod +x backup.sh
$ ./backup.sh
Starting backup...
Exporting configuration...
Listing modules...
Creating archive...
Backup complete!
```

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Enter | Execute command |
| Backspace | Delete character |
| Ctrl+L | Clear screen (future) |
| Ctrl+C | Interrupt command (future) |
| Ctrl+D | EOF / Exit (future) |
| ↑ / ↓ | Command history (future) |
| Tab | Auto-complete (future) |
| Ctrl+A | Start of line (future) |
| Ctrl+E | End of line (future) |

### Troubleshooting

**Terminal Not Responding:**
1. Check WebSocket connection (green indicator)
2. Refresh page
3. Check backend is running
4. Review browser console for errors

**Garbled Output:**
1. Click **Reset** button
2. Check terminal encoding (UTF-8)
3. Verify ANSI support in backend

**Commands Not Working:**
1. Ensure backend is running
2. Check command syntax
3. Review logs for errors
4. Try simpler command (e.g., `echo test`)

---

## Settings & Preferences

### Theme Settings

**Switching Themes:**
1. Click **Theme Toggle** button in header (☀️ / 🌙)
2. Or navigate to user menu → Settings → Theme
3. Choose Dark or Light
4. Theme applies immediately
5. Saved to localStorage

**Theme Scopes:**
- Applies to all pages
- Updates syntax highlighting
- Changes chart colors
- Affects terminal colors

### Language Settings

**Changing Language:**
1. Click **Language Selector** in header
2. Choose English (EN) or Russian (RU)
3. Language changes immediately
4. Saved to localStorage

**Translation Coverage:**
- 100% UI text translated
- Error messages translated
- Help text translated
- System messages in selected language

### Sidebar Settings

**Collapsing Sidebar:**
1. Click **Menu** icon (☰) in header
2. Sidebar collapses to icons only
3. Hover to see labels
4. Click again to expand

**Sidebar State:**
- Saved to localStorage
- Persists across sessions
- Independent per device/browser

### Notification Settings (Future)

**Planned Features:**
- Desktop notifications
- Sound alerts
- Email notifications
- Webhook integrations

### Data & Privacy

**Clearing Data:**
1. Browser Settings → Privacy
2. Clear browsing data
3. Select "Cookies and site data"
4. Choose time range
5. Clear data

**What Gets Cleared:**
- Theme preference
- Language preference
- Chat session history
- Sidebar state
- Configuration drafts

**What Persists:**
- Server-side configuration
- Module states
- System data

---

## Troubleshooting

### Common Issues

#### Issue 1: Dashboard Not Loading

**Symptoms:**
- Blank white screen
- "Loading..." forever
- Error message

**Solutions:**
1. **Check Backend Connection**
   ```bash
   curl http://localhost:8000/api/v1/health
   ```
   If fails → Start backend server

2. **Check Browser Console**
   - Press F12 to open DevTools
   - Check Console tab for errors
   - Look for CORS, network, or 404 errors

3. **Clear Browser Cache**
   - Hard refresh: Ctrl+Shift+R (Windows/Linux) or Cmd+Shift+R (Mac)
   - Or clear cache in browser settings

4. **Check Proxy Configuration**
   - Verify vite.config.ts proxy settings
   - Ensure backend URL is correct

#### Issue 2: WebSocket Connection Failed

**Symptoms:**
- Red connection indicator
- "WebSocket disconnected" message
- No real-time updates

**Solutions:**
1. **Check WebSocket URL**
   - Default: `ws://localhost:8000/ws`
   - Verify in .env file or constants.ts

2. **Check Firewall**
   ```bash
   sudo ufw allow 8000/tcp
   ```

3. **Check Backend Logs**
   ```bash
   tail -f backend.log | grep websocket
   ```

4. **Test WebSocket Manually**
   ```javascript
   const ws = new WebSocket('ws://localhost:8000/ws');
   ws.onopen = () => console.log('Connected!');
   ws.onerror = (e) => console.error('Error:', e);
   ```

#### Issue 3: Module Won't Start

**Symptoms:**
- "Failed to start module" error
- Module status shows ERROR
- Red notification

**Solutions:**
1. **Check Module Logs**
   - Click "View Logs" on module card
   - Look for error messages
   - Common errors:
     - Port already in use
     - Missing dependencies
     - Configuration errors

2. **Check Dependencies**
   - View module details → Info tab
   - Verify all dependencies are running
   - Start dependency modules first

3. **Restart Module**
   - Stop module completely
   - Wait 5 seconds
   - Start again

4. **Check System Resources**
   - CPU usage < 90%
   - Memory usage < 85%
   - Disk space > 10% free

#### Issue 4: Configuration Not Saving

**Symptoms:**
- "Failed to save configuration" error
- Changes revert after refresh
- Validation errors

**Solutions:**
1. **Check JSON Syntax**
   - Validate JSON format
   - Use JSON validator: jsonlint.com
   - Look for missing commas, brackets

2. **Check Permissions**
   ```bash
   ls -la config/
   # Should show write permissions
   ```

3. **Check Backend Logs**
   - Look for validation errors
   - Check file system errors

4. **Try Reset to Defaults**
   - Click "Reset to Defaults"
   - Modify one setting at a time
   - Save after each change

#### Issue 5: Chat Not Responding

**Symptoms:**
- Messages send but no response
- Loading spinner forever
- "Error: Failed to get response" message

**Solutions:**
1. **Check API Connection**
   ```bash
   curl -X POST http://localhost:8000/api/v1/chat/message \
     -H "Content-Type: application/json" \
     -d '{"message": "test"}'
   ```

2. **Check WebSocket Channel**
   - Ensure `chat` channel is subscribed
   - Look for WebSocket errors in console

3. **Clear Chat Session**
   - Click "Clear" to reset conversation
   - Or create new chat session

4. **Check Backend Logs**
   - Look for AI/LLM errors
   - Check API rate limits
   - Verify model is loaded

### Performance Issues

#### Slow Loading

**Causes:**
- Large bundle size
- Slow network connection
- Backend latency

**Solutions:**
1. **Enable Compression**
   - Ensure Vite build uses gzip
   - Check `vite build` output

2. **Use Production Build**
   ```bash
   npm run build
   npm run preview
   ```

3. **Optimize Images**
   - Compress images
   - Use WebP format
   - Lazy load images

#### High Memory Usage

**Causes:**
- Too many WebSocket subscriptions
- Memory leaks
- Large data sets

**Solutions:**
1. **Check Browser Task Manager**
   - Chrome: Shift+Esc
   - Look for memory usage

2. **Close Unused Tabs**
   - Each tab loads full app
   - Close duplicate dashboard tabs

3. **Restart Browser**
   - Clear all tabs
   - Restart browser
   - Reload dashboard

### Error Messages

| Error | Meaning | Solution |
|-------|---------|----------|
| `Network Error` | Can't reach backend | Start backend server |
| `Timeout` | Request took too long | Check backend performance |
| `401 Unauthorized` | Not authenticated | Login (future feature) |
| `403 Forbidden` | No permission | Check user permissions |
| `404 Not Found` | Endpoint doesn't exist | Check API version |
| `500 Internal Server Error` | Backend error | Check backend logs |
| `WebSocket Disconnected` | WS connection lost | Check WS URL and firewall |

---

## Keyboard Shortcuts

### Global Shortcuts

| Shortcut | Action |
|----------|--------|
| `Alt+1` | Go to Dashboard (future) |
| `Alt+2` | Go to Modules (future) |
| `Alt+3` | Go to Config (future) |
| `Alt+4` | Go to Bootstrap (future) |
| `Alt+5` | Go to Chat (future) |
| `Alt+6` | Go to Terminal (future) |
| `Ctrl+K` | Open command palette (future) |
| `Ctrl+/` | Toggle sidebar |
| `F1` | Open help (future) |

### Page-Specific Shortcuts

**Dashboard:**
| Shortcut | Action |
|----------|--------|
| `R` | Refresh metrics (future) |
| `E` | Export data (future) |

**Modules:**
| Shortcut | Action |
|----------|--------|
| `/` | Focus search box (future) |
| `Ctrl+A` | Select all modules (future) |

**Chat:**
| Shortcut | Action |
|----------|--------|
| `Enter` | Send message |
| `Shift+Enter` | New line |
| `Ctrl+N` | New chat (future) |
| `Ctrl+L` | Clear chat (future) |

**Terminal:**
| Shortcut | Action |
|----------|--------|
| `Enter` | Execute command |
| `Backspace` | Delete character |
| `Ctrl+L` | Clear screen (future) |
| `Ctrl+C` | Interrupt (future) |

---

## FAQ

### General Questions

**Q: Do I need to install anything?**
A: Yes, you need Node.js and npm. Then run `npm install` in the `src/web` directory.

**Q: What browsers are supported?**
A: Chrome/Edge 90+, Firefox 88+, Safari 14+. Modern browsers with ES6+ support.

**Q: Can I use this on mobile?**
A: Yes, the dashboard is responsive and works on tablets and phones, though some features are optimized for desktop.

**Q: Is there a demo available?**
A: Currently no public demo. You need to run the backend and frontend locally.

**Q: Does this work offline?**
A: No, it requires a connection to the NeuroGraph backend server.

### Dashboard Questions

**Q: How often do metrics update?**
A: Every 1-30 seconds via WebSocket when available, otherwise polls every 30 seconds.

**Q: Can I export metrics data?**
A: Not yet, but this is planned for a future version.

**Q: What does "Fast Path %" mean?**
A: Percentage of queries that used the optimized fast path cache. Higher is better.

**Q: Why are some metrics showing "N/A"?**
A: Either the backend doesn't provide that metric, or there's a connection issue.

### Module Questions

**Q: Why can't I start a module?**
A: Check if dependencies are running, system resources are available, and the module configuration is valid.

**Q: What happens if I stop a critical module?**
A: The system may become unstable. Only stop modules you understand.

**Q: Can I install new modules?**
A: Not through the UI yet. Currently requires backend configuration.

**Q: How do I update a module?**
A: Currently requires backend update. UI-based updates planned for future version.

### Configuration Questions

**Q: What happens if I save invalid configuration?**
A: The system validates before saving and shows error messages if invalid.

**Q: Can I undo configuration changes?**
A: Use "Revert Changes" before saving, or "Reset to Defaults" after saving.

**Q: What are good CDNA scale values?**
A: Depends on your use case. Start with defaults (all 0.5) and adjust incrementally.

**Q: Do I need to restart after config changes?**
A: Some settings require module restart. The UI will indicate when restart is needed.

### Chat Questions

**Q: Is my chat history saved?**
A: Yes, in your browser's localStorage. It's not sent to any server except the NeuroGraph backend.

**Q: Can I export chat conversations?**
A: Not yet, but this feature is planned.

**Q: How do I delete a chat session?**
A: Open Sessions drawer and click the X button next to the session.

**Q: Is there a character limit for messages?**
A: The frontend doesn't impose a limit, but the backend may have restrictions.

### Terminal Questions

**Q: What commands are available?**
A: All commands provided by the NeuroGraph backend CLI. Run `neurograph help` for a list.

**Q: Can I run interactive commands?**
A: Limited support currently. Full interactive terminal support planned.

**Q: Does command history work?**
A: Not yet. Arrow key navigation for command history is planned.

**Q: Can I paste multi-line commands?**
A: Limited support. Consider using a script file for complex multi-line commands.

### Technical Questions

**Q: What port does the development server use?**
A: Default is 5173. Configure in vite.config.ts if needed.

**Q: How do I change the API URL?**
A: Edit `.env` file and set `VITE_API_URL` to your backend URL.

**Q: Can I use HTTPS?**
A: Yes, configure Vite for HTTPS or put behind a reverse proxy (nginx, Caddy).

**Q: How do I build for production?**
A: Run `npm run build`. Output will be in `dist/` directory.

**Q: Where are logs stored?**
A: Browser console (F12) for frontend logs. Backend logs are on the server.

---

## Next Steps

**After mastering the basics:**

1. **Explore Module Details** - Deep-dive into each module's metrics and logs
2. **Experiment with CDNA Scales** - Fine-tune cognitive parameters
3. **Use Chat for Queries** - Learn the AI's capabilities
4. **Create Custom Dashboards** - (Future feature) Build personalized views
5. **Set Up Monitoring Alerts** - (Future feature) Get notified of issues
6. **Integrate with External Tools** - (Future feature) Export to Grafana, Prometheus

**Learn More:**

- Read the CHANGELOG_v0.62.0.md for detailed feature list
- Review the WEB_DASHBOARD_v0_62_0_SPEC.md for technical specifications
- Check the API documentation for backend integration
- Join the community forum for tips and discussions (coming soon)

**Contribute:**

- Report bugs on GitHub
- Suggest features in discussions
- Submit pull requests
- Write documentation improvements

---

## Support

**Getting Help:**

1. **Check this User Guide** - Most common questions answered here
2. **Review Troubleshooting Section** - Common issues and solutions
3. **Check Browser Console** - F12 for error messages
4. **Review Backend Logs** - Server-side error details
5. **GitHub Issues** - Report bugs and request features

**Contact:**

- GitHub: [neurograph-os-mvp](https://github.com/your-org/neurograph-os-mvp)
- Email: support@neurograph.ai (if available)
- Forum: community.neurograph.ai (coming soon)

---

## Appendix

### Glossary

- **CDNA** - Cognitive Dimension Network Architecture
- **SPA** - Single Page Application
- **WebSocket** - Real-time bidirectional communication protocol
- **API** - Application Programming Interface
- **Module** - Independent system component
- **Bootstrap** - System initialization process
- **Metrics** - Quantitative measurements of system performance
- **Dashboard** - Visual monitoring interface
- **Terminal** - Command-line interface

### Default Configuration

**Default CDNA Scales:**
```json
{
  "sensitivity": 0.5,
  "plasticity": 0.5,
  "stability": 0.5,
  "integration": 0.5,
  "differentiation": 0.5,
  "phase_transition": 0.5,
  "criticality": 0.5,
  "meta_awareness": 0.5
}
```

**Default System Settings:**
```json
{
  "api_base_url": "http://localhost:8000",
  "websocket_url": "ws://localhost:8000/ws",
  "log_level": "INFO",
  "max_connections": 100,
  "request_timeout": 30000,
  "enable_cache": true,
  "cache_ttl": 3600
}
```

### Version History

- **v0.62.0** (Dec 30, 2025) - Initial release with 7/9 phases complete
- **v0.61.1** (Dec 29, 2025) - Jupyter integration enhancements
- **v0.61.0** (Previous) - Core Jupyter integration

### Credits

**Built with:**
- React 18.2
- TypeScript 5.2
- Ant Design Pro 2.6
- xterm.js 5.3
- Zustand 4.4

**Generated with:**
- [Claude Code](https://claude.com/claude-code)
- Claude Sonnet 4.5

---

**User Guide Version:** 1.0
**Last Updated:** December 30, 2025
**Dashboard Version:** 0.62.0 (77.8% complete)
