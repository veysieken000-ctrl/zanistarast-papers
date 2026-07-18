import { miraApi } from "./api-client.js";

const statusElement = document.querySelector(".status");
const statusDotElement = document.querySelector(".status-dot");
const statusTextElement = statusElement?.querySelector("span:last-child");

function updateApiStatus(state, message) {
  if (statusElement) {
    statusElement.dataset.state = state;
    statusElement.setAttribute("aria-label", `Bağlantı durumu: ${message}`);
  }

  if (statusTextElement) {
    statusTextElement.textContent = message;
  }

  if (!statusDotElement) {
    return;
  }

  if (state === "online") {
    statusDotElement.style.background = "#22c55e";
    statusDotElement.style.boxShadow = "0 0 14px rgb(34 197 94 / 70%)";
    return;
  }

  if (state === "offline") {
    statusDotElement.style.background = "#ef4444";
    statusDotElement.style.boxShadow = "0 0 14px rgb(239 68 68 / 70%)";
    return;
  }

  statusDotElement.style.background = "#f59e0b";
  statusDotElement.style.boxShadow = "0 0 14px rgb(245 158 11 / 70%)";
}

async function checkApiHealth() {
  updateApiStatus("checking", "API denetleniyor");

  try {
    const response = await miraApi.health();

    if (response?.status === "ok") {
      updateApiStatus("online", "Mira API hazır");
      return;
    }

    updateApiStatus("offline", "API yanıtı doğrulanamadı");
  } catch (error) {
    console.warn("Mira API sağlık kontrolü başarısız:", error);
    updateApiStatus("offline", "Mira API bağlantısı yok");
  }
}

void checkApiHealth();




