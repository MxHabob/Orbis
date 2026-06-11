import React, { useState } from "react";
import ReactDOM from "react-dom/client";
import { invoke } from "@tauri-apps/api/core";

function SetupScreen() {
  const [url, setUrl] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const handleConnect = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");

    if (!url) {
      setError("الرجاء إدخال عنوان السيرفر أولاً.");
      return;
    }

    // Clean URL
    let formattedUrl = url.trim();
    if (!formattedUrl.startsWith("http://") && !formattedUrl.startsWith("https://")) {
      formattedUrl = `http://${formattedUrl}`;
    }

    setLoading(true);
    try {
      // Save URL and navigate main window
      await invoke("save_server_url", { url: formattedUrl });
    } catch (err: any) {
      setError(err?.toString() || "فشل الاتصال بالسيرفر. تأكد من العنوان وصلاحية الشبكة.");
      setLoading(false);
    }
  };

  return (
    <div className="container">
      <style>{`
        body {
          margin: 0;
          font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
          background: radial-gradient(circle at top right, #111827, #030712);
          color: #f3f4f6;
          display: flex;
          align-items: center;
          justify-content: center;
          min-height: 100vh;
          direction: rtl;
        }
        .container {
          width: 100%;
          max-width: 480px;
          padding: 20px;
        }
        .card {
          background: rgba(17, 24, 39, 0.7);
          backdrop-filter: blur(12px);
          border: 1px solid rgba(255, 255, 255, 0.08);
          border-radius: 16px;
          padding: 40px 30px;
          box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.3), 0 8px 10px -6px rgba(0, 0, 0, 0.3);
          text-align: center;
        }
        .logo {
          font-size: 32px;
          font-weight: 800;
          letter-spacing: 1px;
          margin-bottom: 24px;
          background: linear-gradient(135deg, #60a5fa, #3b82f6);
          -webkit-background-clip: text;
          -webkit-text-fill-color: transparent;
        }
        h2 {
          font-size: 20px;
          margin-top: 0;
          margin-bottom: 12px;
          color: #ffffff;
          font-weight: 600;
        }
        p {
          font-size: 14px;
          color: #9ca3af;
          line-height: 1.6;
          margin-bottom: 30px;
        }
        .form-group {
          margin-bottom: 20px;
          text-align: right;
        }
        label {
          display: block;
          font-size: 13px;
          color: #d1d5db;
          margin-bottom: 8px;
          font-weight: 500;
        }
        input {
          width: 100%;
          box-sizing: border-box;
          padding: 12px 16px;
          background: rgba(31, 41, 55, 0.6);
          border: 1px solid rgba(255, 255, 255, 0.1);
          border-radius: 8px;
          color: #ffffff;
          font-size: 15px;
          direction: ltr;
          transition: border-color 0.2s, box-shadow 0.2s;
        }
        input:focus {
          outline: none;
          border-color: #3b82f6;
          box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
        }
        .btn {
          width: 100%;
          padding: 12px;
          background: linear-gradient(135deg, #3b82f6, #1d4ed8);
          border: none;
          border-radius: 8px;
          color: #ffffff;
          font-size: 15px;
          font-weight: 600;
          cursor: pointer;
          transition: opacity 0.2s, transform 0.1s;
          box-shadow: 0 4px 12px rgba(29, 78, 216, 0.3);
        }
        .btn:hover {
          opacity: 0.95;
        }
        .btn:active {
          transform: scale(0.98);
        }
        .btn:disabled {
          background: #374151;
          color: #9ca3af;
          cursor: not-allowed;
          box-shadow: none;
        }
        .error-msg {
          margin-top: 15px;
          color: #f87171;
          font-size: 13px;
          background: rgba(248, 113, 113, 0.1);
          padding: 10px;
          border-radius: 6px;
          border: 1px solid rgba(248, 113, 113, 0.2);
        }
      `}</style>

      <div className="card">
        <div className="logo">Orbis</div>
        <h2>إعداد خادم المدرسة</h2>
        <p>
          الرجاء إدخال عنوان IP الخاص بسيرفر المدرسة للاتصال بالنظام المحلي. يمكنك الحصول على العنوان من شاشة تشغيل السيرفر.
        </p>

        <form onSubmit={handleConnect}>
          <div className="form-group">
            <label htmlFor="ip-address">عنوان السيرفر (IP Address)</label>
            <input
              id="ip-address"
              type="text"
              placeholder="192.168.1.100:3001"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              disabled={loading}
              autoFocus
            />
          </div>

          <button type="submit" className="btn" disabled={loading}>
            {loading ? "جاري الاتصال..." : "حفظ واتصال"}
          </button>
        </form>

        {error && <div className="error-msg">{error}</div>}
      </div>
    </div>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <SetupScreen />
  </React.StrictMode>,
);
