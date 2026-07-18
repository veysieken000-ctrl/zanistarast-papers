/**
 * Mira PWA çalışma zamanı yapılandırması.
 *
 * Güvenlik sınırları:
 * - Bu dosyaya MIRA_MUDEBBIR_TOKEN yazılmaz.
 * - Repository Secret yazılmaz.
 * - Parola veya oturum anahtarı yazılmaz.
 * - Kimlik bilgileri tarayıcı depolamasına kaydedilmez.
 *
 * API canlı HTTPS ortamına dağıtıldığında yalnızca apiBaseUrl değeri
 * güncellenecektir.
 */
export const miraConfig = Object.freeze({
  apiBaseUrl: "",
  apiConfigured: false
});


