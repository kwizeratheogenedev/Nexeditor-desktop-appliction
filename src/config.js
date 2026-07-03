// Read API base URL from Vite env (set at build) or runtime window global (injected by Tauri runner)
const API_BASE = import.meta.env.VITE_API_URL || (typeof window !== 'undefined' && window.__API_URL) || 'http://127.0.0.1:3000';

export default API_BASE;

export const API_ENDPOINTS = {
  createMontage: `${API_BASE}/api/create-montage`,
  fetchUrlVideo: `${API_BASE}/api/fetch-url-video`,
  convert: `${API_BASE}/api/convert`,
  extractShorts: `${API_BASE}/api/extract-shorts`,
  generateCaptions: `${API_BASE}/api/generate-captions`,
  reformatShort: `${API_BASE}/api/reformat-short`,
};
