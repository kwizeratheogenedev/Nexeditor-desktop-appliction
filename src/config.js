const API_BASE = 'http://127.0.0.1:3000';

export default API_BASE;

export const API_ENDPOINTS = {
  createMontage: `${API_BASE}/api/create-montage`,
  fetchUrlVideo: `${API_BASE}/api/fetch-url-video`,
  convert: `${API_BASE}/api/convert`,
  extractShorts: `${API_BASE}/api/extract-shorts`,
  generateCaptions: `${API_BASE}/api/generate-captions`,
  reformatShort: `${API_BASE}/api/reformat-short`,
};
