# Mira Account Recovery Policy

## Amaç

Bu belge, Müdebbir hesabının güvenli şekilde kurtarılması için uygulanacak kuralları tanımlar.

## Kurtarma Yöntemleri

Desteklenen yöntemler:

- Doğrulanmış birincil e-posta
- Doğrulanmış ikinci (kurtarma) e-posta
- Yeni cihazdan yeniden kimlik doğrulama
- Yeni Passkey oluşturulması

## Cihaz Kaybı

Cihaz kaybedildiğinde:

1. Yeni cihazdan giriş yapılır.
2. Birincil e-posta doğrulanır.
3. Gerekirse ikinci e-posta doğrulanır.
4. Yeni Passkey oluşturulur.
5. Eski cihaz oturumları sonlandırılır.

## Hesap Güvenliği

Aşağıdaki durumlarda kurtarma işlemi durdurulmalıdır:

- Kimlik doğrulaması başarısızsa
- Şüpheli oturum tespit edilirse
- Rasterast doğrulaması başarısız olursa

## Acil Durum

Şüpheli erişim durumunda:

- Tüm aktif oturumlar sonlandırılır.
- Tüm Passkey kayıtları iptal edilebilir.
- Secret'lar gerekiyorsa yenilenir.
- Müdebbir hesabı yeniden doğrulanır.

## Rasterast Doğrulaması

Doğrulanacak maddeler:

- Hesap sahibi doğrulaması
- Kurtarma e-postası doğrulaması
- Yeni cihaz doğrulaması
- Passkey yeniden oluşturulması

## Sonuç

Hesap kurtarma işlemleri yalnızca doğrulanmış kimlik bilgileriyle ve güvenlik kontrolleri tamamlandıktan sonra gerçekleştirilmelidir.

