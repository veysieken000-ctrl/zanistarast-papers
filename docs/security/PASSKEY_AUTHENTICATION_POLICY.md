# Mira Passkey Authentication Policy

## Amaç

Bu belge, Mira sisteminde kullanılacak Passkey tabanlı kimlik doğrulama kurallarını tanımlar.

## Temel İlkeler

- Passkey, ilk e-posta doğrulaması tamamlandıktan sonra etkinleştirilmelidir.
- Her cihaz kendi Passkey bilgisini güvenli şekilde saklamalıdır.
- Passkey özel anahtarları cihaz dışına çıkarılmamalıdır.

## Desteklenen Kimlik Doğrulama

- Parmak izi
- PIN
- Cihazın güvenli ekran kilidi
- Platform tarafından desteklenen biyometrik yöntemler

## Yeni Cihaz

Yeni bir cihaz eklendiğinde:

1. E-posta kimliği doğrulanmalıdır.
2. Müdebbir kimliği doğrulanmalıdır.
3. Yeni Passkey oluşturulmalıdır.
4. Eski Passkey'ler isteğe bağlı olarak kaldırılabilir.

## Passkey İptali

Aşağıdaki durumlarda Passkey iptal edilebilmelidir:

- Cihaz kaybı
- Güvenlik şüphesi
- Müdebbir talebi

## Rasterast Doğrulaması

Doğrulanacak maddeler:

- Kimlik doğrulama
- Cihaz doğrulaması
- Güvenli Passkey kaydı
- Passkey iptal işlemleri

## Sonuç

Passkey, parola yerine geçmez; güvenli kimlik doğrulama mekanizmasını güçlendiren ek bir doğrulama katmanı olarak kullanılmalıdır.

