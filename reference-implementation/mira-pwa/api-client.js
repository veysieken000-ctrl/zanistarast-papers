const DEFAULT_API_BASE_URL = "http://127.0.0.1:3000";

/**
 * Mira API ile iletişim sınırı.
 *
 * Güvenlik kuralları:
 * - MIRA_MUDEBBIR_TOKEN bu dosyaya yazılmaz.
 * - Token localStorage veya sessionStorage içinde saklanmaz.
 * - Kimlik doğrulama daha sonra sunucu kontrollü güvenli oturumla yapılır.
 * - API isteklerinde tarayıcı çerezleri yalnızca sunucu güvenli biçimde
 * yapılandırıldıktan sonra kullanılacaktır.
 */
export class MiraApiClient {
  constructor(baseUrl = DEFAULT_API_BASE_URL) {
    this.baseUrl = baseUrl.replace(/\/+$/, "");
  }

  async health() {
    return this.request("/health", {
      method: "GET"
    });
  }

  async createSession(title) {
    return this.request("/sessions", {
      method: "POST",
      body: JSON.stringify({ title })
    });
  }

  async getSession(sessionId) {
    return this.request(`/sessions/${encodeURIComponent(sessionId)}`, {
      method: "GET"
    });
  }

  async sendMessage(sessionId, message) {
    return this.request(
      `/sessions/${encodeURIComponent(sessionId)}/messages`,
      {
        method: "POST",
        body: JSON.stringify({ message })
      }
    );
  }

  async listTasks() {
    return this.request("/tasks", {
      method: "GET"
    });
  }

  async getTask(taskId) {
    return this.request(`/tasks/${encodeURIComponent(taskId)}`, {
      method: "GET"
    });
  }

  async request(path, options = {}) {
    const response = await fetch(`${this.baseUrl}${path}`, {
      ...options,
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
        Accept: "application/json",
        ...(options.headers ?? {})
      }
    });

    const contentType = response.headers.get("content-type") ?? "";
    const isJson = contentType.includes("application/json");

    const payload = isJson
      ? await response.json()
      : await response.text();

    if (!response.ok) {
      const message =
        typeof payload === "object" && payload !== null && "error" in payload
          ? payload.error
          : `Mira API isteği başarısız oldu: HTTP ${response.status}`;

      throw new Error(message);
    }

    return payload;
  }
}

export const miraApi = new MiraApiClient();



