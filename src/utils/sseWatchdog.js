import * as Sentry from "@sentry/vue";
import { invoke } from "@tauri-apps/api/core";

export class SSEWatchdog {
  /**
   * 初始化 SSE 看门狗
   * @param {string} url SSE 的连接地址
   * @param {number} timeoutMs 看门狗超时时间 (默认 60000 ms)
   * @param {Function} onMessage 正常接收消息的回调
   */
  constructor(url, timeoutMs = 60000, onMessage = null) {
    this.url = url;
    this.timeoutMs = timeoutMs;
    this.onMessage = onMessage;
    
    this.eventSource = null;
    this.watchdogTimer = null;
    this.reconnectAttempts = 0;
    
    this.connect();
  }

  connect() {
    console.log(`[SSE] Connecting to ${this.url}`);
    this.eventSource = new EventSource(this.url);

    this.eventSource.onopen = () => {
      console.log(`[SSE] Connected to ${this.url}`);
      this.reconnectAttempts = 0;
      this.resetWatchdog();
    };

    this.eventSource.onmessage = (event) => {
      // 收到任何消息（包括服务端的 ping）都重置看门狗
      this.resetWatchdog();
      if (this.onMessage && event.data !== "ping") {
        this.onMessage(event);
      }
    };

    this.eventSource.onerror = (err) => {
      console.error("[SSE] Connection error:", err);
      // EventSource 自己会尝试重连，但如果是致命错误或断网，我们需要处理
      this.handleError(new Error("SSE Native Error"));
    };
  }

  resetWatchdog() {
    if (this.watchdogTimer) {
      clearTimeout(this.watchdogTimer);
    }
    
    this.watchdogTimer = setTimeout(() => {
      this.handleWatchdogTimeout();
    }, this.timeoutMs);
  }

  handleWatchdogTimeout() {
    const errorMsg = `[SSE] Watchdog Timeout: No messages received for ${this.timeoutMs}ms. Connection appears dead.`;
    console.error(errorMsg);
    
    // 1. 记录到 Sentry
    Sentry.captureException(new Error(errorMsg));
    
    // 2. 优先通过 Rust 记录 / 处理
    invoke("report_error", { 
      message: errorMsg, 
      extraInfo: "SSE Watchdog Timeout" 
    }).catch(console.error);

    // 3. 强制关闭死连接并重连
    this.reconnect();
  }

  handleError(err) {
    Sentry.captureException(err);
    invoke("report_error", { 
      message: err.message || "SSE Error", 
      extraInfo: "SSE Connection Error" 
    }).catch(console.error);
  }

  reconnect() {
    this.close();
    
    // 指数退避重连
    const delay = Math.min(1000 * Math.pow(2, this.reconnectAttempts), 30000);
    this.reconnectAttempts++;
    
    console.log(`[SSE] Attempting reconnect in ${delay}ms... (Attempt ${this.reconnectAttempts})`);
    
    setTimeout(() => {
      this.connect();
    }, delay);
  }

  close() {
    if (this.eventSource) {
      this.eventSource.close();
      this.eventSource = null;
    }
    if (this.watchdogTimer) {
      clearTimeout(this.watchdogTimer);
      this.watchdogTimer = null;
    }
  }
}
